import copy
import random
import logging
from llms import Prompt, QueryEngine
from utils import *
import settings


class Fixer:
    def __init__(
        self, fix_type, query_engine: QueryEngine, global_constraints, compile_fix_constrains, comp_fix_attempt_budget=3
    ) -> None:
        self.comp_fix_attempt_budget = comp_fix_attempt_budget
        self.fix_type = fix_type  # options.comp_fix
        self.query_engine = query_engine
        self.global_constraints = global_constraints
        self.compile_fix_constrains = compile_fix_constrains

    def fix(self, rust_code="", comp_out=None, work_dir=None, log_id = ""):
        '''
        Call the comp_fix_msft_work function to fix Rust code compilation errors.
        Args:
            rust_code: Rust code
            comp_out: Tuple containing compilation output
            work_dir: Workspace
            log_id: Log ID
        Returns:
            rust_code: Fixed Rust code
            len(errors): Number of remaining errors
            num_llm_call: Number of calls to llm
        '''
        return self.comp_fix_msft_multiple(rust_code, comp_out, work_dir, log_id)

    def cargo_fix(self, work_dir):
        '''
        Use the cargo fix command to automatically repair Rust code.
        Args:
            work_dir: Workspace # work_dir/wspace
        '''
        with cd(f"{work_dir}"):
            subprocess.run(f"cargo clean", capture_output=True, shell=True)

            # Use Rust's build tool Cargo to compile the project
            comp_output_bf_cfix = subprocess.run(
                f'RUSTFLAGS="-Z track-diagnostics -Z time-passes" cargo build --manifest-path Cargo.toml',
                capture_output=True,
                shell=True,
            )  # Return execution status or error information
            _, _, _, _, init_num_errors = parse_error_timepass(comp_output_bf_cfix.stderr)

            # Automatically fix compiler warnings and errors in Rust code.
            subprocess.run(f"cargo fix --allow-no-vcs", capture_output=True, shell=True)

            subprocess.run(f"cargo clean", capture_output=True, shell=True)
            # Recompile
            comp_output_af_cfix = subprocess.run(
                f'RUSTFLAGS="-Z track-diagnostics -Z time-passes" cargo build --manifest-path Cargo.toml',
                capture_output=True,
                shell=True,
            )
            _, _, _, _, fnl_num_errors = parse_error_timepass(comp_output_af_cfix.stderr)

            logging.info(f"\tNumber of errors decreased from {init_num_errors} to {fnl_num_errors} with cargo fix.")

    def comp_fix_msft_single(self, rust_code, init_comp_out, work_dir, log_id):
        '''
        Call LLM to fix compilation errors in the given Rust code.
        Args:
            rust_code: Rust code
            init_comp_out: Tuple containing compilation output
            work_dir: Workspace
            log_id: Log ID
        Returns:
            rust_code: Fixed Rust code.
            len(errors): Number of remaining compilation errors
            num_llm_call: Number of calls to LLM (Large Language Model) used to fix compilation errors
        '''
        errors = init_comp_out[0]  # Init_comp_out [0]: Contains a list of all compilation errors.
        num_llm_call = 0
        while errors:
            snap = rust_code
            error = random.choice(errors)

            cur_errors = {error}
            rep_counter = 0
            while True:
                cur_err = cur_errors.pop()
                # Build prompt message (single error message)
                prompt = Prompt(
                    context=(
                        f"You are a programming assistant responsible for fixing errors in the code, which is the Rust code contained within the <code> tags."
                        # f"You are given a Rust code contained in <code> tags."
                        + tag(rust_code, "code")
                        + "This code does not compile. Here are some error messages contained in <error-message> tags"
                        + tag(cur_err.body, "error-message")
                    ),
                    instruction="Please repair the code based on these error messages and provide the complete code after repair.",
                    constraints=self.compile_fix_constrains,
                )

                recode_prompt = copy.deepcopy(prompt)
                recode_prompt.constraints = self.global_constraints + prompt.constraints + settings.test_constraints_prompt

                # #Use LLM to fix code
                responses, rust_code = self.query_engine.generate_code(prompt)

                num_llm_call += 1  # increment before log

                if rust_code == "":
                    continue
                # Compile and output errors
                comp_output = compile_and_record_query(rust_code, work_dir,
                                                        self.query_engine.stringify_prompt(recode_prompt),
                                                        responses,f"com_fix_cand_{log_id}_{num_llm_call}",
                                                        is_logged=True)
                # parse error
                fnl_comp_out = parse_error_timepass(comp_output.stderr)
                new_errors = fnl_comp_out[0]
                cur_errors = set(new_errors) - set(errors)

                if not cur_errors: # Solved the current error
                    errors = new_errors
                    break

                rep_counter += 1
                if rep_counter == 2:
                    rust_code = snap
                    break

            # we project that 12 would give #llm_calls similar to adv-err-fix
            if num_llm_call >= 5:
                break

        return rust_code, len(errors), num_llm_call



    def comp_fix_msft_multiple(self, rust_code, init_comp_out, work_dir, log_id):
        '''
        Call LLM to fix compilation errors in the given Rust code.
        Args:
            rust_code: Rust code
            init_comp_out: Tuple containing compilation output
            work_dir: Workspace
            log_id: Log ID
        Returns:
            rust_code: Fixed Rust code.
            len(errors): Number of remaining compilation errors
            num_llm_call: Number of calls to LLM (Large Language Model) used to fix compilation errors
        '''
        errors = init_comp_out[0]
        num_llm_call = 0
        while errors:

            if len(errors) > 5:
                break

            snap = rust_code

            # #Build a prompt containing all error information
            error_bodies = "\n".join([err.body for err in errors])

            prompt = Prompt(
                context=(
                    f"You are a syntax repair expert, and below is a piece of Rust code enclosed within <code> tags."
                    # f"You are given a Rust code contained in <code> tags."
                    + tag(rust_code, "code")
                    + "This code does not compile. Here are some errors messages contained in <errors> tags"
                    + tag(error_bodies, "errors")
                ),
                instruction="Please repair the code based on these error messages and provide the complete code after repair.",
                constraints=self.compile_fix_constrains,
            )

            recode_prompt = copy.deepcopy(prompt)
            recode_prompt.constraints = self.global_constraints + prompt.constraints + settings.test_constraints_prompt

            # Use LLM to fix code
            responses, new_rust_code = self.query_engine.generate_code(prompt)
            num_llm_call += 1  # increment before log

            if new_rust_code == "":
                logging.info(f"No code generated for prompt num_{num_llm_call}")
                continue

            # Compile and output errors
            comp_output = compile_and_record_query(new_rust_code,
                                                    work_dir,
                                                    self.query_engine.stringify_prompt(recode_prompt),
                                                    responses,
                                                    f"com_fix_cand_{log_id}_{num_llm_call}",
                                                    is_logged=True)
            # parse error
            fnl_comp_out = parse_error_timepass(comp_output.stderr)
            new_errors = fnl_comp_out[0]

            # Update code and error list (if improved)
            if len(new_errors) < len(errors):
                rust_code = new_rust_code
                errors = new_errors
            else:
                rust_code = snap

            if num_llm_call > 3:
                break

        return rust_code, len(errors), num_llm_call