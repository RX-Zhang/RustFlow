import json
import logging
import shutil
import copy

from llms import QueryEngine, Prompt
from utils import *
import settings

class Transpiler:
    def __init__(
        self,
        trans_type,
        comp_fixer,
        query_engine: QueryEngine,
        options,
        global_constraints,
        common_translate_constraints,
    ) -> None:
        self.trans_type = trans_type  # base
        self.comp_fixer = comp_fixer
        self.query_engine = query_engine
        self.options = options
        self.src_lang = options.language
        self.benchmark = options.benchmark_name
        self.fname = options.submodule_name
        self.transpl_attempt_budget = options.transpl_attempt_budget
        self.work_dir = options.work_dir
        self.cot_version = options.cot_version if options.cot else "base"
        self.few_shots = options.few_shot
        self.global_constraints = global_constraints
        self.common_translate_constraints = common_translate_constraints
        self.model_params = {"temperature": options.initial_temperature}
        self.benchmark_path = f"benchmark/{self.src_lang}/{self.benchmark}"
        self.hint = ""

    def transpile(self, hinted_fix = False, feedback = False):
        '''
        self.prompt == base
        Returns:
            compiles: Compilation result
        '''
        if self.trans_type == "base":
            return self.transpile_base(hinted_fix, feedback)
        elif self.trans_type == "cot":
            return self.transpile_cot(hinted_fix, feedback)
        else:
            raise ValueError(f"Invalid prompt: {self.trans_type}")


    def write_src_code_to_res_dir(self, res_dir: str, src_code: str):
        '''
        Write the given source code src_code and its corresponding JSON into the specified result directory res_dir.
        Args:
            res_dir: Result directory
            src_code: Source code

        '''
        with open(f"{res_dir}/{self.fname}.{self.src_lang}", "w") as fw:
            fw.write(src_code)
        subprocess.run(
            f"cp {self.benchmark_path}/{self.fname}/{self.fname}.json {res_dir}/",
            shell=True,
        )

    # @profile
    def transpile_base(self, hinted_fix, feedback):
        '''
            Translate the specified source code from one language to Rust, and perform save compilation and fix compilation errors.
        Returns:
            compiles: Compilation result
        '''
        compiles = False
        logging.info(f"Now transpiling {self.fname}.")

        with open(f"{self.benchmark_path}/{self.fname}/{self.fname}.{self.src_lang}", "r") as f:
            code = f.read()

        src_dir = f"{self.work_dir}/wspace/"
        res_dir = f"{self.work_dir}/results/"
        base_few_shots = self.few_shots_prompt()

        prompt = Prompt(
            context=(
                f"You are a code translation expert, and below is a piece of {self.src_lang.capitalize()} code within <code> tags."
                + tag(code, "code")
            ),
            instruction = self.cot_instruction(),
            constraints = self.common_translate_constraints,
            extra_information=self.hint,
            few_shots=f"There are some translation examples in the <example> tag, please refer to them."
                        + tag(base_few_shots, "example") if self.few_shots else ""
        )

        recode_prompt = copy.deepcopy(prompt)
        recode_prompt.constraints = self.global_constraints + recode_prompt.constraints + settings.test_constraints_prompt

        compiled_answer: list[str] = []
        error_answer: list[str] = []

        for attempt in range(1, self.transpl_attempt_budget + 1):
            # generate code
            responses, cand_answer = self.query_engine.generate_code(prompt, model_params=self.model_params, fix=False)


            # delete fn main()
            cand_answer = [self.delete_main(answer) for answer in cand_answer]

            for index, (response, answer) in enumerate(zip(responses, cand_answer)):

                log_ids = "feedback" if feedback else "first"
                comp_out = compile_and_record_query(answer, src_dir,
                                                    self.query_engine.stringify_prompt(recode_prompt),
                                                    response, log_id=f"base_{log_ids}_cand_{index}", is_logged=True)

                cand_init_comp_out = parse_error_timepass(comp_out.stderr)
                num_errs = cand_init_comp_out[-1]
                if answer == "":
                    continue
                if not num_errs:
                    compiled_answer.append(answer)
                elif num_errs <= 5:
                    error_answer.append(answer)

            if compiled_answer or error_answer:
                break

        if not compiled_answer and not error_answer:
            return compiles


        if error_answer and self.comp_fixer is not None:
            if self.comp_fixer.fix_type == "msft":
                logging.info("\tThere are candidate errors, start compiling and fixing them via LLM.")
                for answer in error_answer:
                    comp_out = compile_and_record_query(answer, src_dir)

                    init_comp_out = parse_error_timepass(comp_out.stderr)
                    _, _, _, _, init_num_err = init_comp_out

                    rust_code, fnl_num_err, num_llm_call = self.comp_fixer.fix(answer, init_comp_out, src_dir)
                    logging.info(f"\t\tNum errors decreased from {init_num_err} to {fnl_num_err}.")
                    if not fnl_num_err:
                        compiled_answer.append(rust_code)

        elif error_answer and self.comp_fixer is None:
            logging.info("\tTranspilation FAILED. No fixer is set.")

        if compiled_answer:
            compiles = True

            if os.path.exists(f"{res_dir}"):
                shutil.rmtree(f"{res_dir}")
            os.makedirs(f"{res_dir}")

            for index, answer in enumerate(compiled_answer):
                candidate_path = f"{res_dir}/candidate_{index}"
                os.makedirs(candidate_path)
                self.write_src_code_to_res_dir(candidate_path, code)
                with open(f"{candidate_path}/{self.fname}.rs", "w") as fw:
                    fw.write(answer)

        # clean project to reduce size
        with cd(f"{src_dir}"):
            subprocess.run("cargo clean", capture_output=True, shell=True)
        return compiles

    # @profile
    def transpile_cot(self, hinted_fix, feedback):
        '''
            Translate the specified source code from one language to Rust, and perform save compilation and fix compilation errors, with chain-of-thought (CoT).
        Returns:
            compiles: Compilation result
        '''
        compiles = False
        logging.info(f"Now transpiling {self.fname}.")

        with open(f"{self.benchmark_path}/{self.fname}/{self.fname}.{self.src_lang}", "r") as f:
            code = f.read()

        src_dir = f"{self.work_dir}/wspace/"
        res_dir = f"{self.work_dir}/results/"
        cot_few_shots = self.few_shots_prompt()

        prompt = Prompt(
            context=(
                    f"You are a code translation expert, and below is a piece of {self.src_lang.capitalize()} code within <code> tags."
                    + tag(code, "code")
            ),
            instruction = self.cot_instruction(),
            constraints = self.common_translate_constraints,
            extra_information = self.hint,
            few_shots=f"There are some translation examples in the <example> tag, please refer to them."
                    + tag(cot_few_shots, "example") if self.few_shots else ""
        )

        recode_prompt = copy.deepcopy(prompt)
        recode_prompt.constraints = self.global_constraints + recode_prompt.constraints + settings.test_constraints_prompt

        compiled_answer: list[str] = []
        error_answer: list[str] = []


        for attempt in range(1, self.transpl_attempt_budget + 1):

            responses, cand_answer = self.query_engine.generate_code(prompt, model_params=self.model_params,fix=False)

            # delete fn main()
            cand_answer = [self.delete_main(answer) for answer in cand_answer]

            for index, (response, answer) in enumerate(zip(responses, cand_answer)):
                log_ids = "feedback" if feedback else "frist"

                comp_out = compile_and_record_query(answer, src_dir,
                                                    self.query_engine.stringify_prompt(recode_prompt),
                                                    response, log_id=f"cot_{log_ids}_cand_{index}", is_logged=True)

                cand_init_comp_out = parse_error_timepass(comp_out.stderr)
                num_errs = cand_init_comp_out[-1]
                if answer == "":
                    continue
                if not num_errs:
                    compiled_answer.append(answer)
                elif num_errs <= 5:
                    error_answer.append(answer)

            if compiled_answer or error_answer:
                break
        if not compiled_answer and not error_answer:
            logging.info("\tcompiled_answer and error_answer is None")
            return compiles

        if error_answer and self.comp_fixer is not None:
            if self.comp_fixer.fix_type == "msft":
                logging.info("\tThere are candidate errors, start compiling and fixing them via LLM.")
                for index, answer in enumerate(error_answer):
                    comp_out = compile_and_record_query(answer, src_dir)

                    init_comp_out = parse_error_timepass(comp_out.stderr)
                    _, _, _, _, init_num_err = init_comp_out

                    rust_code, fnl_num_err, num_llm_call = self.comp_fixer.fix(answer, init_comp_out, src_dir, index)
                    logging.info(f"\t\tNum errors decreased from {init_num_err} to {fnl_num_err}.")
                    if not fnl_num_err:
                        compiled_answer.append(rust_code)

        elif error_answer and self.comp_fixer is None:
            logging.info("\tTranspilation FAILED. No fixer is set.")

        if compiled_answer:
            compiles = True

            if os.path.exists(f"{res_dir}"):
                shutil.rmtree(f"{res_dir}")
            os.makedirs(f"{res_dir}")

            for index, answer in enumerate(compiled_answer):
                candidate_path = f"{res_dir}/candidate_{index}"
                os.makedirs(candidate_path)
                self.write_src_code_to_res_dir(candidate_path, code)
                with open(f"{candidate_path}/{self.fname}.rs", "w") as fw:
                    fw.write(answer)

        # clean project to reduce size
        with cd(f"{src_dir}"):
            subprocess.run("cargo clean", capture_output=True, shell=True)
        return compiles


    def few_shots_prompt(self) -> str:
        if self.cot_version == "base":
            return ''' 
```c
#include <stdint.h>
void flip_sign(int32_t* n, int k);
void flip_sign(int* n, int k) {
    *n = ~(*n) + k;
}```\n
```rust
fn flip_sign(n: &mut i32, k: i32) {
    *n = (!*n).wrapping_add(k);
}```
'''
        elif self.cot_version == "go":
            return ''' 
```c
#include <stdint.h>
void flip_sign(int32_t* n, int k);
void flip_sign(int* n, int k) {
    *n = ~(*n) + k;
}```\n
```go
package main
func flip_sign(n *int32, k int32) {
    *n = ^*n + k
}```\n
```rust
fn flip_sign(n: &mut i32, k: i32) {
    *n = (!*n).wrapping_add(k);
}```
'''
        elif self.cot_version == "llvm":
            return ''' 
```c
#include <stdint.h>
void flip_sign(int32_t* n, int k);
void flip_sign(int* n, int k) {
    *n = ~(*n) + k;
}```\n
```llvm ir
define void @flip_sign(i32* %n, i32 %k) #0 {
entry:
    %n.addr = alloca i32*, align 8
    %k.addr = alloca i32, align 4
    store i32* %n, i32** %n.addr, align 8
    store i32 %k, i32* %k.addr, align 4
    %0 = load i32*, i32** %n.addr, align 8
    %1 = load i32, i32* %0, align 4
    %not = xor i32 %1, -1
    %2 = load i32, i32* %k.addr, align 4
    %add = add i32 %not, %2
    %3 = load i32*, i32** %n.addr, align 8
    store i32 %add, i32* %3, align 4
    ret void
}```\n
```rust
fn flip_sign(n: &mut i32, k: i32) {
    *n = (!*n).wrapping_add(k);
}```
'''
        elif self.cot_version == "ast":
            return ''' 
```c
#include <stdint.h>
void flip_sign(int32_t* n, int k);
void flip_sign(int* n, int k) {
    *n = ~(*n) + k;
}```\n

```ast
FunctionDef(
    name="flip_sign",
    return_type="void",
    params=[
        Param(type="int32_t*", name="n"),
        Param(type="int", name="k")
    ],
    body=[
        Assignment(
            target=Dereference(var="n"),
            value=BinaryOp(
                op="+",
                left=UnaryOp(
                    op="~",
                    operand=Dereference(var="n")
                ),
                right=Variable(name="k")
            )
        )
    ]
)```\n
```rust
fn flip_sign(n: &mut i32, k: i32) {
    *n = (!*n).wrapping_add(k);
}```
'''
        elif self.cot_version == "explain":
            return ''' 
```c
#include <stdint.h>
void flip_sign(int32_t* n, int k);
void flip_sign(int* n, int k) {
    *n = ~(*n) + k;
}```\n
function description:
This function implements numerical transformation through pointer operations, specifically executing:
1. Invert the integer pointed to by the pointer bit by bit (~operation).
2. Add the inverse result to the parameter k.
3. Write back the calculation result through a pointer.\n
```rust
fn flip_sign(n: &mut i32, k: i32) {
    *n = (!*n).wrapping_add(k);
}```
'''
        else:
            raise ValueError("types should be in ['base', 'go', 'llvm', 'ast', 'explain']")

    def cot_instruction(self) -> str :
        if self.cot_version == "base":
            return f"Please help me translate the {self.src_lang.capitalize()} code above into Rust code.\n"
        elif self.cot_version == "go":
            return "Please translate this code into Go language first, use it as a reference, and then translate it into Rust code."
        elif self.cot_version == "llvm":
            return "Please translate this code into LLVM IR language first, use it as a reference, and then translate it into Rust code."
        elif self.cot_version == "ast":

            return "Please translate this code into Abstract Syntax Tree (AST), use it as a reference, and then translate it into Rust code."
        elif self.cot_version == "explain":
            return "Please briefly describe the purpose and execution process of this code as a reference, and then translate it into Rust code."
        else:
            raise ValueError("types should be in ['go', 'llvm', 'ast', 'explain']")

    def delete_main(self, answer: str) -> str :
        if answer == "":
            return answer
        index = answer.find("fn main()")
        if index != -1:
            answer = answer[:index]
        return answer