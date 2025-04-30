import copy
from dataclasses import dataclass
from typing import Any, Optional, List, Tuple, Union
import logging
import json
import random
import tempfile
import functools
import multiprocessing
from functools import partial

from overrides import override
from itertools import starmap
from subprocess import CalledProcessError

import llms
from llms import Prompt, QueryEngine
import settings
from fixer import Fixer

from utils import (compile_and_record_query, parse_error_timepass, parse_error_coarse, tag)
from settings import Options
import oracle

class Enhancement:
    def __init__(self, replay_dir: str, positive_examples: str, negative_examples: str):
        '''
        Args:
            replay_dir: Path to the replay directory.
            positive_examples: Positive examples in string format.
            negative_examples: Negative examples in string format.
        '''
        N_EXAMPLES = 10

        cov_to_ce = oracle.group_examples_by_coverage(replay_dir, negative_examples, N_EXAMPLES)

        _, ce_group = random.choice(list(cov_to_ce.items()))
        #  len(ce_group) <= N_EXAMPLES
        if len(ce_group) > N_EXAMPLES:
            ce_group = random.sample(ce_group, N_EXAMPLES)
        else:
            ce_group = ce_group

        self.ce_group = ce_group

    def enhancement(
        self, context: str,
            textual_examples: str,
            query_engine: QueryEngine
    ) -> str:
        return ""


class LLMExplain(Enhancement):
    def __init__(self, replay_dir: str, positive_examples: str, negative_examples: str):
        super().__init__(replay_dir, positive_examples, negative_examples)

    @override
    def enhancement(
        self, context: str, textual_examples: str, query_engine: QueryEngine
    ) -> str:
        '''
        Enhance the problem using textual_examples and obtain an answer from LLM.
        Args:
            context: Textual problem.
            textual_examples: Textual representation of input/output examples.
            query_engine: Query engine used to send queries to LLM and retrieve answers.
        Returns:
            Information returned by LLM.
        '''
        logging.info("Enhancing prompt with LLM-Based root cause explanations.")

        explain_prompt = Prompt(
            context=context,
            instruction="Tell me the root cause of the issue and how to fix it in the Rust code.",
            constraints=[
                "Just provide the reason for the error and repair suggestions, no further information is needed."
            ],
            extra_information=(
                "Below are several sets of input/output examples included in the <testcases> tag.\n"
                +"The 'Expected' contains the running result of the original C code, while the 'Actual' contains the running result of the Rust code above."
                + tag(textual_examples, "testcases")
            ),
        )

        answer = query_engine.query(explain_prompt)

        enhancement = (
            "\nHere is a possible explanation and solution in <answers>, please use it as reference information when solving problems."
            + tag(answer, "answers")
        )

        return enhancement


@functools.total_ordering
class Candidate:
    def __init__(
        self,
        rust_code: str,
        semantics_fix_constrains: list[str],
        positive_examples: str,
        negative_examples: str,
        extra: Union[None, Enhancement, Tuple[str, List[Tuple[str, str]]]],
    ) -> None:
        self.rust_code = rust_code
        self.semantics_fix_constrains = semantics_fix_constrains
        self.positive_examples = positive_examples
        self.negative_examples = negative_examples
        ne = json.loads(self.negative_examples)
        pe = json.loads(self.positive_examples)
        self.score = len(pe) / (len(pe) + len(ne)) # 1: 只有正例
        self.extra = extra # Enhancement 或 LLMExplain 对象

    def hint(self, n_examples: int) -> str:
        '''
        Generate a prompt containing a certain number of negative examples.
        Args:
            n_examples: Total number of examples to include in the prompt.
        Returns:
            hint: string.
        '''
        n_negatives = n_examples
        logging.info(f"Hinted with  {n_negatives} negative examples")

        negative_examples = json.loads(self.negative_examples)

        if len(negative_examples) > n_negatives:
            negative_examples = random.sample(negative_examples, n_negatives)

        examples = list(
            starmap(
                lambda idx, example: f"I/O_example_{idx}:\n{{ {textual_example_negative(example)} }}",
                enumerate(negative_examples),
            )
        )  # { Example 0:\n Arguments:\n  Argument 0: {..}\n  Argument 1: {..}\n Expected Output: {..}\n ...}

        preamble = "Consider the following input/output examples included in the <testcases> tag.\n"

        hint = preamble + tag("\n".join(examples), "testcases")

        return hint

    def prompt(
        self,
        query_engine: QueryEngine,
        src_code: str,
        src_lang: str,
        n_examples: int,
        few_shot: bool,
        history: List[Tuple[str, str]] = [],
    ) -> Prompt:
        if self.ok:
            assert not self.extra
            raise RuntimeError("Ok candidate should not have this.")
        
        if n_examples == 0:
            assert len(history) == 0, "CAPR requires counter examples"

        # Counter examples enhancement
        if len(self.extra.ce_group) > n_examples:
            ce_group = random.sample(self.extra.ce_group, n_examples)
        else:
            ce_group = self.extra.ce_group

        textual_examples = list_examples(ce_group)

        extra_information: str
        if few_shot and n_examples > 0:
            extra_information = (
                "Below are several sets of input/output examples included in the <cases> tag.\n"
                + "The 'Expected' contains the running result of the original C code, while the 'Actual' contains the running result of the Rust code above."
                + tag(textual_examples, "cases")
            )
        else:
            extra_information = ""

        if len(history) > 0:
            return Prompt(
                context="In the previous modifications, the program still had incorrect input and output.",
                instruction="Please modify the given code again to obtain the expected output of the given test input.",
                constraints= [],
                extra_information=extra_information,
                history=history
            )

        context = (
            f"You are a semantic repair expert. The <code> tag below contains the original {src_lang.capitalize()} code."
            + tag(src_code, "code")
            + f"You will also receive the compiled Rust code contained in the <code> tag below, which is the translation of the {src_lang.capitalize()} code above, but this translation will not provide the expected output for certain inputs."
            + tag(self.rust_code, "code")
        )

        prompt = Prompt(
            context = context,
            instruction = "Make changes to the given Rust code to obtain the expected output of the given test input.",
            constraints = self.semantics_fix_constrains,
            extra_information=extra_information,
        )

        return prompt

    @property
    def ok(self) -> bool:
        return self.score == 1

    def __eq__(self, other):
        if not isinstance(other, Candidate):
            raise NotImplementedError
        return self.score == other.score

    def __lt__(self, other):
        if not isinstance(other, Candidate):
            raise NotImplementedError
        return self.score < other.score


class CandidateFactory:
    def __init__(
        self,
        src_code: str,
        src_code_json: str,
        language: str,
        submodule_name: str,
        sem_fix: str,   # options.sem_fix
        workspace_name: str,
        semantics_fix_constrains: list[str],
    ) -> None:
        self.src_code = src_code
        self.src_code_json = src_code_json
        self.language = language
        self.submodule_name = submodule_name
        self.workspace_name = workspace_name
        self.semantics_fix_constrains = semantics_fix_constrains
        if sem_fix == "base":
            Extra = Enhancement
        elif sem_fix == "llm-explain":
            Extra = LLMExplain
        else:
            raise NotImplementedError
        self.Extra = Extra

    def construct_candidate(
        self,
        rust_codes: list[str],
    ) -> list[Candidate]:
        '''
            Generate fuzzing code, execute fuzz testing to obtain positive and negative examples, and then construct a candidate object.
        Args:
            rust_codes: Rust source code.
        Returns:
            candidate.
        '''
        candidates: list[Candidate] = []
        for rust_code in rust_codes:
            with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as tmp_dir:
                src_dir = tmp_dir

                with open(src_dir + f"/{self.submodule_name}.{self.language}","w") as f:
                    f.write(self.src_code)
                with open(src_dir + f"/{self.submodule_name}.json","w") as f:
                    f.write(self.src_code_json)
                with open(src_dir + f"/{self.submodule_name}.rs", "w") as f:
                    f.write(rust_code)

                workspace = tmp_dir + "/replay"
                try:

                    oracle.instrument(self.language, src_dir, self.submodule_name, workspace)
                except CalledProcessError:
                    logging.info("The current cand_code failed to generate test code for C or Rust, try the next cand_code.")
                    continue
                logging.info("Successfully generated C and Rust test code.")

                validation_result = oracle.verify(workspace, self.submodule_name)

                if not validation_result:
                    logging.info("The current cand_code failed to generate oracle, try the next cand_code.")
                    continue

                positive_examples, negative_examples = validation_result

                candidate: Candidate
                try:
                    # create candidate
                    candidate = Candidate(rust_code, self.semantics_fix_constrains, positive_examples, negative_examples, None)
                except json.decoder.JSONDecodeError:
                    logging.info("The current cand_code construct candidate failed, try the next cand_code.")
                    continue
                if not candidate.ok:
                    # Enhancement or LLMExplain
                    candidate.extra = self.Extra(workspace, positive_examples, negative_examples)
                else:
                    candidate.extra = oracle.compute_coverage_by_libfuzzer_corpus(workspace)
                logging.info("construct candidate successfully")
                candidates.append(candidate)
                if candidate.ok:
                    break

        candidates.sort(reverse=True)
        return candidates

    def process_rust_code(self,
                        rust_code: str):
        with tempfile.TemporaryDirectory(ignore_cleanup_errors=True) as tmp_dir:
            src_dir = tmp_dir

            with open(src_dir + f"/{self.submodule_name}.{self.language}", "w") as f:
                f.write(self.src_code)
            with open(src_dir + f"/{self.submodule_name}.json", "w") as f:
                f.write(self.src_code_json)
            with open(src_dir + f"/{self.submodule_name}.rs", "w") as f:
                f.write(rust_code)

            workspace = tmp_dir + "/replay"
            try:

                oracle.instrument(self.language, src_dir, self.submodule_name, workspace)
            except CalledProcessError:
                logging.info("The current cand_code failed to generate test code for C or Rust, try the next cand_code.")
                return None
            # logging.info("Successfully generated C and Rust test code.")

            validation_result = oracle.verify(workspace, self.submodule_name)

            if not validation_result:
                logging.info("The current cand_code failed to generate oracle, try the next cand_code.")
                return None

            positive_examples, negative_examples = validation_result

            candidate: Candidate
            try:
                # create candidate
                candidate = Candidate(rust_code, self.semantics_fix_constrains, positive_examples, negative_examples, None)
            except json.decoder.JSONDecodeError:
                logging.info("The current cand_code construct candidate failed, try the next cand_code.")
                return None
            if not candidate.ok:
                # Enhancement or LLMExplain
                candidate.extra = self.Extra(workspace, positive_examples, negative_examples)
            else:
                candidate.extra = oracle.compute_coverage_by_libfuzzer_corpus(workspace)
            # logging.info("construct candidate successfully")
            return candidate


    def construct_candidate_mul_process(
        self,
        rust_codes: list[str],
    ) -> list[Candidate]:
        '''
            Generate fuzzing code, execute fuzz testing to obtain positive and negative examples, and then construct a candidate object.
        Args:
            rust_codes: Rust source code.
        Returns:
            candidate
        '''
        if not rust_codes:
            logging.info("rust code is none, construct candidate failed")
            return []

        logging.info("Start constructing candidate")
        if len(rust_codes) == 1:

            result = self.process_rust_code(rust_codes[0])
            return [result] if result is not None else []

        with multiprocessing.Pool() as pool:  # processes=1
            process_func = partial(self.process_rust_code)
            candidates = pool.map(process_func, rust_codes)

        logging.info("construct candidate successful")

        candidates = [candidate for candidate in candidates if candidate is not None]

        candidates.sort(reverse=True)
        return candidates


@dataclass(eq=False, repr=False)
class SemanticsStrategy:
    comp_fixer:Fixer
    cand_index: int
    factory: CandidateFactory
    options: Options
    query_engine: QueryEngine
    global_constraints: list[str]
    budget: int

    def optimize(self, candidate: Candidate, few_shot = True) -> Candidate:
        '''
        Iteratively optimize the candidate solution until specific conditions are met or the budget limit is reached.
        In each round of optimization, it attempts to fix issues in the current candidate solution and evaluates whether the repaired candidate is better than the current best candidate.
        If the conditions are satisfied (e.g., the ok attribute of the candidate is true), the optimized candidate is returned; otherwise, the iteration continues until the budget limit is reached.
        '''
        round_idx = 1
        history: List[Tuple[str, str]] = []
        while round_idx <= self.budget :
            logging.info(f"Starting the {round_idx}-th round of fixing. Beam size = 1, history length = {len(history)}")

            new_history, new_candidates = self.fix(candidate, history, round_idx, few_shot)

            # update history
            history = new_history

            # new_candidates.sort(reverse=True)
            logging.info(f"find a new candidate. Highest score = {new_candidates.score}")

            candidate = new_candidates
            if candidate.ok:
                return candidate

            round_idx += 1

        return candidate

    def fix(self, candidate: Candidate,
            history: List[Tuple[str, str]],
            round_idx: int,
            few_shot: bool
            ) -> tuple[list[tuple[str, str]], Candidate]:
        '''
        Use LLM to fix semantic inconsistencies in the code.
        Args:
            candidate: Candidate object
            history: Historical information
            round_idx: Round index
            few_shot: Few-shot flag
        Returns:
            candidate: Repaired candidate object
        '''
        prompt = candidate.prompt(
            self.query_engine,
            self.factory.src_code,
            self.factory.language,
            self.options.n_prompt_examples,
            few_shot,
            history=history[:(self.options.conversation_window_size * 2)],

        )
        REP_THOLD = 3
        trial = 0
        min_num_errs = 2 ** 32
        best_answer_processed = ""
        responses = ""
        new_rust_code: str

        is_history = False if round_idx == 1 or self.options.feedback_strategy == "BaseRepair" else True
        recode_prompt = copy.deepcopy(prompt)
        recode_prompt.constraints = prompt.constraints if is_history else self.global_constraints + prompt.constraints + settings.test_constraints_prompt
        while trial < REP_THOLD:

            responses, new_rust_code = self.query_engine.generate_code(prompt, history = is_history)

            comp_out = compile_and_record_query(new_rust_code, self.src_dir)

            comp_out = parse_error_timepass(comp_out.stderr)
            num_errs = comp_out[-1]
            if new_rust_code == "":
                continue
            elif num_errs < min_num_errs:
                min_num_errs = num_errs
                best_answer_processed = new_rust_code

            if not num_errs:
                break  # 退出循环
            logging.info("Fixed code does not compile. Giving it another try.")
            trial += 1

        if best_answer_processed == "":
            logging.info("The LLMs call has encountered a malfunction and cannot respond.")
            return history,candidate

        comp_out = compile_and_record_query(best_answer_processed, self.src_dir,
                                            self.query_engine.stringify_prompt(recode_prompt), responses,
                                            log_id=f"sem_fix_cand_{self.cand_index}_budget_{round_idx}", is_logged=True)

        init_comp_out = parse_error_timepass(comp_out.stderr)


        answer_processed = best_answer_processed
        have_error: bool = True if init_comp_out[-1] else False

        if init_comp_out[-1] and self.comp_fixer is not None:
            if self.comp_fixer.fix_type == "msft":
                logging.info("\tTranspilation FAILED. Attempting to fix compilation errors via LLM.")
                _, _, _, _, init_num_err = init_comp_out

                _, fnl_num_err, _ = self.comp_fixer.fix(answer_processed, init_comp_out, self.src_dir)
                logging.info(f"\t\tNum errors decreased from {init_num_err} to {fnl_num_err}.")
                if not fnl_num_err:
                    have_error =  False

        if have_error:
            logging.info("Could not find a fix that compiles.")
            return history, candidate

        new_candidate = self.factory.construct_candidate_mul_process([answer_processed])

        if self.options.conversation:
            # prompt.constraints = self.global_constraints + prompt.constraints + settings.test_constraints_prompt
            history.append((llms.USER, str(recode_prompt)))
            history.append((llms.ASSISTANT, answer_processed))


        if not new_candidate or new_candidate[0] <= candidate:
            logging.info("Found candidate of bad quality.")
            return history,candidate

        return history, new_candidate[0]

    @property
    def src_dir(self) -> str:
        return f"{self.options.work_dir}/wspace"

def list_examples(negative_examples: List[Any]) -> str:
    '''
    Generate a string that lists the details of all negative examples, including their input parameters, expected output, and actual output.
    Args:
        negative_examples: Negative examples

    Returns:

    '''
    RETURN_VOID = "\"Program execution successful, no return value\""
    examples_list = ""
    for ce_idx, s_ce in enumerate(negative_examples):
        Expected_Output: str
        Actual_Output: str
        expect_return_results = ""
        actual_return_results = ""
        if s_ce["actual"] == "ExecutionFailure":
            Actual_Output = "Under this input parameter, the program crashes"
        else:
            output = s_ce["actual"]["ExecutionSuccess"]
            if isinstance(output, str):
                output_part = output.split(':', 1)
                actual_return_results = output_part[0]
                if len(output_part)>1:
                    Actual_Output = simplify_data(json.loads(output_part[1]))
                    Actual_Output = RETURN_VOID if Actual_Output is None else Actual_Output

        if s_ce["expected"] == "ExecutionFailure":
            Expected_Output = "Under this input parameter, the program crashes"
        else:
            output = s_ce["expected"]["ExecutionSuccess"]
            if isinstance(output, str):
                output_part = output.split(':', 1)
                expect_return_results = output_part[0]
                if len(output_part)>1:
                    Expected_Output = simplify_data(json.loads(output_part[1]))
                    Expected_Output = RETURN_VOID if Expected_Output is None else Expected_Output

        arguments = []
        for arg_idx, arg in enumerate(s_ce["args"]):
            arg = json.loads(arg)
            arg = simplify_data(arg)
            arguments.append(arg)

        if (s_ce["expected"] != "ExecutionFailure" and s_ce["actual"] != "ExecutionFailure") and (
                expect_return_results != "output" and actual_return_results != "output"):
            examples_list = (
                examples_list + f"\n Case_{ce_idx}:\n "
                                f"{{"
                                f"\"args\":{arguments},\n"
                                f"\"Expected Output\":{RETURN_VOID},\n"
                                f"\"Expected {expect_return_results}\":{Expected_Output},\n"
                                f"\"Actual Output\":{RETURN_VOID},\n"
                                f"\"Actual {actual_return_results}\":{Actual_Output}\n"
                                f"}}"
            )
        else:
            examples_list = (
                    examples_list + f"\n Case_{ce_idx}:\n "
                                    f"{{"
                                    f"\"args\":{arguments},\n"
                                    f"\"Expected Output\":{Expected_Output},\n"
                                    f"\"Actual Output\":{Actual_Output}\n"
                                    f"}}"
            )

    return examples_list


def simplify_data(json_data):
    '''
    Simplify the representation of JSON data.
    eg: input: {"key1": [1, 2, 3, 4, 5, 6, 7]}
        output: {"key1": [1, 2, 3, 4, "... and 2 other elements"]}
    '''
    MAX_ARRAY_LENGTH = 5
    if isinstance(json_data, dict):
        return {key: simplify_data(value) for key, value in json_data.items()}
    elif isinstance(json_data, list):
        if len(json_data) > MAX_ARRAY_LENGTH:
            n_removed = len(json_data) - MAX_ARRAY_LENGTH
            return [simplify_data(value) for value in json_data[:MAX_ARRAY_LENGTH]] + [
                f"... and {n_removed} other elements"
            ]
        else:
            return [simplify_data(value) for value in json_data]

    return json_data


def textual_example_all_negative(example: Any) -> str:
    '''
    Convert an example data into a string that describes the example.
    Args:
        example: Example

    Returns: (str)
        Argument {arg_idx}: {arg}
        Expected Output: {output}

    '''
    RETURN_VOID = "\"Program execution successful, no return value\""

    Expected_Output: str
    Actual_Output: str
    expect_return_results = ""
    actual_return_results = ""
    # expected
    try:
        if example["expected"] == "ExecutionFailure":
            Expected_Output = "Under this input parameter, the program crashes"
        else:
            output = example["expected"]["ExecutionSuccess"]
            if isinstance(output, str):
                output_part = output.split(':',1)
                expect_return_results = output_part[0]
                if len(output_part)>1:
                    Expected_Output = simplify_data(json.loads(output_part[1]))
                    Expected_Output = RETURN_VOID if Expected_Output is None else Expected_Output

            else:
                Expected_Output = example["expected"]
    except KeyError:
        raise  ValueError("KeyError")

    # actual
    if example["actual"] == "ExecutionFailure":
        Actual_Output = "Under this input parameter, the program crashes"
    else:
        output = example["actual"]["ExecutionSuccess"]
        if isinstance(output, str):
            output_part = output.split(':',1)
            actual_return_results = output_part[0]
            if len(output_part) > 1:
                Actual_Output = simplify_data(json.loads(output_part[1]))
                Actual_Output = RETURN_VOID if Actual_Output is None else Actual_Output

        else:
            Actual_Output = example["actual"]

    arguments = []
    for arg_idx, arg in enumerate(example["args"]):
        arg = json.loads(arg)
        arg = simplify_data(arg)
        arguments.append(arg)


    if (example["expected"] != "ExecutionFailure" and example["actual"] != "ExecutionFailure" ) and (expect_return_results != "output" and actual_return_results != "output"):  # no return
        return (f"\"args\":{arguments},\n"
                f"\"Expected Output\":{RETURN_VOID},\n"
                f"\"Expected {expect_return_results}\":{Expected_Output},\n"
                f"\"Actual Output\":{RETURN_VOID},\n"
                f"\"Actual {actual_return_results}\":{Actual_Output}\n")
    else:  # have return
        return (f"\"args\":{arguments},\n"
                f"\"Expected Output\":{Expected_Output},\n"
                f"\"Actual Output\":{Actual_Output}\n")

def textual_example_negative(example: Any) -> str:
    '''
    Convert an example data into a string that describes the example.
    Args:
        example: Example

    Returns: (str)
        Argument {arg_idx}: {arg}
        Expected Output: {output}

    '''
    RETURN_VOID = "\"No return value\""

    Expected_Output: str = ""
    expect_return_results = ""
    # expected
    try:
        if example["expected"] == "ExecutionFailure":
            Expected_Output = "Under this input parameter, the program crashes"
        else:
            output = example["expected"]["ExecutionSuccess"]
            if isinstance(output, str):
                output_part = output.split(':',1)
                expect_return_results = output_part[0]
                if len(output_part)>1:
                    Expected_Output = simplify_data(json.loads(output_part[1]))
                    Expected_Output = RETURN_VOID if Expected_Output is None else Expected_Output

            else:
                Expected_Output = example["expected"]
    except KeyError:
        raise  ValueError("KeyError")

    arguments = []
    for arg_idx, arg in enumerate(example["args"]):
        arg = json.loads(arg)
        arg = simplify_data(arg)
        arguments.append(arg)

    if example["expected"] != "ExecutionFailure":
        if expect_return_results != "output":  # no return
            return (f"\"Args\":{arguments},\n"
                    f"\"Output\":{RETURN_VOID},\n"
                    f"\"{expect_return_results}\":{Expected_Output},\n")
        else:  # have return
            return (f"\"Args\":{arguments},\n"
                    f"\"Output\":{Expected_Output},\n")
    else:
        return ""


def textual_example_positive(example: Any) -> str:
    '''
    Convert an example data into a string that describes the example.
    Args:
        example: Example

    Returns: (str)
        Argument {arg_idx}: {arg}
        Expected Output: {output}

    '''
    output:str = ""

    try:
        arguments = example["args"]

        outputs = example["actual"]["ExecutionSuccess"]
        if isinstance(outputs, str):
            output_part = outputs.split(':',1)
            if len(output_part) > 1:
                output = output_part[1]
        else:
            output = example["actual"]
    except KeyError:
        raise  ValueError("KeyError")

    return (f"\"args\":{arguments},\n"
            f"\"Output\":{output},")