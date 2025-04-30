import logging

from argparse_dataclass import ArgumentParser
from typing import List, Tuple, Optional
import shutil
from dataclasses import asdict

import settings
from fixer import Fixer
from llms import QueryEngineFactory
from transpiler import Transpiler
from settings import Options
from utils import *

from semantics import Candidate, CandidateFactory, SemanticsStrategy

common_translate_constraints = [
    "Ensure the Rust code is safe and idiomatic.",
    "Use Rust's preferred data types, ensure that using a stable compiler will not result in errors.",
]


def record_cov_data(report: str, show: List[Tuple[str, str]], work_dir: str):
    '''
        Write the coverage report and execution information to a text file in the specified directory.
    Args:
        report: String content of the coverage report
        show: A list containing execution counts and line information, where each element is a tuple (exec_count, line)
        work_dir: Path of the working directory, used to determine the storage location of the files
    '''
    os.makedirs(f"{work_dir}/cov", exist_ok=True)
    with open(f"{work_dir}/cov/cov_report.txt", "w") as f:
        f.write(report)

    with open(f"{work_dir}/cov/cov_show.txt", "w") as f:
        for exec_count, line in show:
            print(f"{exec_count}|{line}", file=f)


def construct_factory(options: Options, translate_constraints: list[str]) -> CandidateFactory:
    '''
        Constructs and returns a CandidateFactory object based on the given options parameter.
    Args:
        options: Contains the configuration information needed to build the CandidateFactory object
        translate_constraints: List of general translation constraints

    Returns:
        CandidateFactory
    '''
    src_code: str
    src_code_json: str

    with open(f"{options.res_dir}/candidate_{0}/{options.submodule_name}.{options.language}", "r") as f:
        src_code = f.read()
    with open(f"{options.res_dir}/candidate_{0}/{options.submodule_name}.json", "r") as f:
        src_code_json = f.read()


    factory = CandidateFactory(
        src_code,
        src_code_json,
        options.language,
        options.submodule_name,
        options.sem_fix,
        options.work_dir,
        translate_constraints
    )
    return factory


def latest_rust_code(options: Options) -> list[str]:
    '''
        Reads the latest Rust code.
    Args:
        options: Contains the configuration information required by the function.

    Returns:
        rust_code: The content of the file is returned as a string.
    '''
    rust_code: list[str] = []

    entries = os.listdir(options.res_dir)
    # use os.path.isdir
    subdirectories = [entry for entry in entries if os.path.isdir(os.path.join(options.res_dir, entry))]

    for i in range(len(subdirectories)):
        with open(f"{options.res_dir}/candidate_{i}/{options.submodule_name}.rs", "r") as f:
            rust_code.append(f.read())

    return rust_code


def initial_transpilation(
    transpiler: Transpiler, options: Options
) -> Optional[Tuple[list[Candidate], CandidateFactory]]:
    '''
        Attempts to perform an initial transpilation operation and returns a valid candidate object.
    Args:
        transpiler: Transpiler object used to perform code transpilation operations.
        options: Options object containing configuration and parameter information.

    Returns:
        If a compilable transpilation result is successfully found:
            candidate: A valid transpilation candidate object.
            factory: Factory object used to construct the candidate object.
        If no valid transpilation result is found (after reaching the number of attempts and still unable to compile):
            Returns None.
    '''
    INIT_ATTEMPT_BUDGTE = 2 #
    for _ in range(INIT_ATTEMPT_BUDGTE):
        compiles = transpiler.transpile()
        if compiles:
            # Check semantic equivalence.
            logging.info("Found a compiling transpilation. Checking semantic equivalence...")
            factory = construct_factory(options, common_translate_constraints)  # Create factory, retrieve source code and corresponding JSON file from the file
            rust_code = latest_rust_code(options)  # Obtain the translated Rust source code
            candidate = factory.construct_candidate_mul_process(rust_code)
            if not candidate:
                logging.info("not candidate, Rerun initial_transpilation!")
                continue

            logging.info("initial_transpilation runs successfully, code is compiled successfully")
            return candidate, factory
        else:
            logging.info("Candidate does not compile. Retrying.")

    logging.info("initial_transpilation runs failed, return None")
    return None


def main(options):

    # clear work_dir
    if os.path.exists(options.work_dir):
        shutil.rmtree(options.work_dir)
    os.makedirs(options.work_dir)

    # Create log related configurations
    logging.basicConfig(
        filename="%s/transpilation.log" % options.work_dir,
        level=logging.INFO,
        filemode="w",
        format="%(name)s - %(levelname)s - %(message)s",
    )
    logging.info("%s transpilation has started." % options.benchmark_name)

    # Save Path Information=====Create a path and save the result======
    res_all = f'result_all/{options.benchmark_name}/{options.model}/{options.feedback_strategy}'
    os.makedirs(res_all, exist_ok=True)
    if options.cot:
        res_path = f"{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}/{options.is_few_shot}/cot-{options.cot_version}/candidates-{options.num_candidates}"
        res_success_path = f'translation_result/success/{options.language}/{options.benchmark_name}/{options.model}/{options.feedback_strategy}/{options.is_few_shot}/cot-{options.cot_version}/candidates-{options.num_candidates}/{options.submodule_name}'
        res_fail_path = f'translation_result/fail/{options.language}/{options.benchmark_name}/{options.model}/{options.feedback_strategy}/{options.is_few_shot}/cot-{options.cot_version}/candidates-{options.num_candidates}/{options.submodule_name}'
    else:
        res_path = f"{options.language}/{options.benchmark_name}/{options.submodule_name}/{options.model}/{options.feedback_strategy}/{options.is_few_shot}/cot-no/candidates-{options.num_candidates}"
        res_success_path = f'translation_result/success/{options.language}/{options.benchmark_name}/{options.model}/{options.feedback_strategy}/{options.is_few_shot}/cot-no/candidates-{options.num_candidates}/{options.submodule_name}'
        res_fail_path = f'translation_result/fail/{options.language}/{options.benchmark_name}/{options.model}/{options.feedback_strategy}/{options.is_few_shot}/cot-no/candidates-{options.num_candidates}/{options.submodule_name}'


    global_constraints = []
    if options.language == "c":
        global_constraints.append("Please use wrapping_* (or checked_*) operations to simulate C semantics.")

    # Create a translation engine
    query_engine = QueryEngineFactory.create_engine(options.model, global_constraints, options.cot_version, options.num_candidates) if options.cot else QueryEngineFactory.create_engine(options.model, global_constraints, num_responses = options.num_candidates)

    # Instantiate repair class
    comp_fixer = None if options.comp_fix == "no" else Fixer(options.comp_fix, query_engine, global_constraints, common_translate_constraints, options.comp_fix_attempt_budget)

    fallback = options.fallback_opt  # "fix"
    restart_budget = options.restart_budget
    fix_budget = options.fix_budget
    # Instantiate translation class
    transpiler = Transpiler(
        "cot" if options.cot  else "base",
        comp_fixer,
        query_engine,
        options,
        global_constraints,
        common_translate_constraints,
    )

    # First time generating Rust translation through LLM, conducting fuzz testing, and then building a candidate object to return
    transpilation = initial_transpilation(transpiler, options)

    if not transpilation:  # transpilation is None
        logging.info("Failed to find compilable/checkable candidate. Return Code: 0.")
        if os.path.isdir(options.res_dir):
            rust_code = latest_rust_code(options)[0]
        else:
            with open(f"{options.work_dir}/wspace/src/lib.rs", "r") as f:
                rust_code = f.read()
        if rust_code != "":
            # save results
            os.makedirs(res_fail_path, exist_ok=True)
            with open(f"{res_fail_path}/{options.submodule_name}.rs", 'w') as file:
                file.write(rust_code)
            with open(f"{res_all}/fail.txt", 'a') as file:
                file.write(res_path + "\n")
            print("\nCompilation failed or candidate build failed!!!")
            return
        else: # not save
            os.makedirs(res_fail_path, exist_ok=True)
            with open(f"{res_fail_path}/{options.submodule_name}.rs", 'w') as file:
                file.write(rust_code)
            with open(f"{res_all}/fail.txt", 'a') as file:
                file.write(res_path + "\n")

            print("\ncompile failed, Rust code is empty!!!")
            return

    candidates, factory = transpilation

    best_candidate = candidates[0] # Only look at the first one (sorted)
    if best_candidate.ok:
        record_cov_data(*best_candidate.extra, options.work_dir)
        logging.info("Transpilation finished. Equivalent transpilation has been found at initial attempt. Return Code: 1.")
        # save results
        os.makedirs(res_success_path, exist_ok=True)
        with open(f"{res_success_path}/{options.submodule_name}.rs", 'w') as file:
            file.write(best_candidate.rust_code)
        with open(f"{res_all}/success.txt", 'a') as file:
            file.write(res_path + "\n")
        print("\nCode translation successful!!!")
        return

    # FALLBACK
    fixed_once = False
    logging.info(f"Transpilation is not equivalent: best candidate score = {best_candidate.score}.")
    # Iterative semantic repair
    if fallback == "fix":
        for index, candidate in enumerate(candidates):
            logging.info("Now attempting LLM-based semantics fixing.")
            semantics_strategy = SemanticsStrategy(
                comp_fixer,
                index,
                factory,
                options,
                query_engine,
                global_constraints,
                budget=fix_budget,
            )
            # Multiple calls to fix using LLM to regenerate semantically consistent Rust code, build a candidate, and then return the best one
            candidate = semantics_strategy.optimize(candidate,options.few_shot)
            if candidate.ok:
                fixed_once = True
                logging.info("Current errors has been cleaned. Verifying again.")
                new_candidates = factory.construct_candidate_mul_process([candidate.rust_code])
                if new_candidates:
                    candidate = new_candidates[0]
                if candidate.ok:
                    record_cov_data(*candidate.extra, options.work_dir)
                    logging.info(f"Equivalent transpilation has been found by {options.feedback_strategy} strategy. Return Code: 2")
                    # save results
                    os.makedirs(res_success_path, exist_ok=True)
                    with open(f"{res_success_path}/{options.submodule_name}.rs", 'w') as file:
                        file.write(candidate.rust_code)
                    with open(f"{res_all}/success.txt", 'a') as file:
                        file.write(res_path + "\n")
                    print("\nCode translation successful!!!")
                    return
                else:
                    logging.info(f"Transpilation is not equivalent: candidate score = {candidate.score}.")
                    pass
    else: # restart
        for restart_idx in range(restart_budget):
            if options.hinted and best_candidate :
                transpiler.hint = best_candidate.hint(options.n_prompt_examples)

            compiles = transpiler.transpile(hinted_fix=options.hinted, feedback=True)
            if not compiles:
                logging.info("The code cannot be compiled in restart, trying again.")
                continue

            logging.info("Found a compiling transpilation. Checking semantic equivalence...")
            restart_rust_codes = latest_rust_code(options)
            candidates = factory.construct_candidate_mul_process(restart_rust_codes)

            if not candidates:
                logging.info(f"construct candidate failed, restart!")
                continue

            best_candidate = candidates[0]

            if best_candidate.ok:
                record_cov_data(*best_candidate.extra, options.work_dir)
                logging.info(f"Equivalent transpilation has been found by {fallback} strategy. Restart id: {restart_idx}. Return Code: 2")
                # save results
                os.makedirs(res_success_path, exist_ok=True)
                with open(f"{res_success_path}/{options.submodule_name}.rs", 'w') as file:
                    file.write(best_candidate.rust_code)
                with open(f"{res_all}/success.txt", 'a') as file:
                    file.write(res_path + "\n")
                print("\nCode translation successful!!!")
                return
            else:
                logging.info(f"Transpilation is not equivalent: candidate score = {best_candidate.score}.")

    if fixed_once:
        logging.info("Fallback process failed cleaning semantic errors.")  # special failure case

    logging.info("Fallback process failed cleaning semantic errors. Return Code: 3")

    # save results
    if os.path.isdir(options.res_dir):
        rust_code = latest_rust_code(options)[0]
    else:
        with open(f"{options.work_dir}/wspace/src/lib.rs", "r") as f:
            rust_code = f.read()

    os.makedirs(res_fail_path, exist_ok=True)
    with open(f"{res_fail_path}/{options.submodule_name}.rs", 'w') as file:
        file.write(rust_code)
    with open(f"{res_all}/fail.txt", 'a') as file:
            file.write(res_path + "\n")
    print("\nCode translation failed!!!")


if __name__ == "__main__":
    parser = ArgumentParser(Options)
    setting_option = parser.parse_args()
    main(setting_option)
    # Clear excess content to prevent occupying more memory
    with cd(f"{setting_option.work_dir}/wspace/"):
        subprocess.run(f"cargo clean", capture_output=True, shell=True)

    print(f"\nProgram runs successfully, called LLM {settings.llm_call_count} times successfully")
