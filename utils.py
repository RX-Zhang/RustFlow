import os
import re
import subprocess

from error import Error
from pathlib import Path
from collections import defaultdict
from contextlib import contextmanager


@contextmanager
def cd(path: Path):
    """Sets the cwd within the context

    Args:
        path (Path): The path to the cwd

    Yields:
        None
    """

    origin = Path().absolute()
    try:
        os.chdir(path)
        yield
    finally:
        os.chdir(origin)

def tag(content: str, tag_name: str) -> str:
    '''

    Args:
        content:
        tag_name:

    Returns:
        if content = NULL  =>   NULL
        if content != NULL =>   <tag_name>
                                content
                                </tag_name>
     '''
    if not content:
        return content
    return f"\n<{tag_name}>\n{content}\n</{tag_name}>\n"

def compile_and_record_query(
        code: str,
        work_dir: str,
        prompt: str = "",
        responses: str = "",
        log_id: str = "",
        is_logged: bool = False
) -> subprocess.CompletedProcess:
    '''
    Compile Rust code and output errors.
    Args:
        code: Rust code block
        work_dir: Workspace # work_dir/wspace/
        prompt: Prompt
        responses: Full LLM feedback information
        log_id: Log identifier
        is_logged: Whether to log

    Returns:
        comp_output: Compilation output information
    '''
    if Path(f"{work_dir}").is_dir():
        with cd(f"{work_dir}"):
            subprocess.run(f"cargo clean", capture_output=True, shell=True)
    else:
        subprocess.run(f"cargo new --lib {work_dir}", capture_output=True, shell=True)
        os.makedirs(f"{work_dir}/logs", exist_ok=True)
        with open(f"{work_dir}/Cargo.toml", "a") as fw:  # add some dependencies by default
            fw.write('rand = "0.8.4"\n')
            fw.write('libc = "0.2"\n')
            fw.write('regex = "1.10.2"\n')  # c urlparser benchmark
            fw.write('lazy_static = "1.4.0"\n')  # go ACH benchmark
            fw.write('once_cell = "1.19.0"\n')  # go ACH benchmark

    with open(f"{work_dir}/src/lib.rs", "w") as f:
        f.write(code)

    with cd(f"{work_dir}"):
        comp_output = subprocess.run(
            f'RUSTFLAGS="-Z track-diagnostics -Z time-passes" cargo build --manifest-path Cargo.toml',
            capture_output=True,
            shell=True,
        )

    if is_logged:
        os.makedirs(f"{work_dir}/logs", exist_ok=True)

        with open(f"{work_dir}/logs/{log_id}_conversion.txt", "w") as f:
            contents = (
                f"===================== Prompt =====================\n\n"
                f"{prompt}\n\n\n"
                f"==================== Response ====================\n\n"
                f"{responses}"
            )
            f.write(contents)

        with open(f"{work_dir}/logs/{log_id}_code.rs", "w") as f:
            f.write(code)

        with open(f"{work_dir}/logs/{log_id}_err.txt", "wb") as file:
            file.write(comp_output.stderr)

    return comp_output

def parse_error_timepass(stderr):
    '''
    Parse error messages and compilation steps during the Rust compilation process.
    Args:
        stderr: Contains the standard error stream output during compilation, passed in as a byte stream.

    Returns:
        errors: A list containing all compilation errors.
        err_code_num: A dictionary recording the occurrence count of each error code.
        err_diag_num: A dictionary recording the occurrence count of each error diagnostic.
        compilation_steps: A list containing the compilation steps.
        len(errors): The total number of errors.
    '''
    lines = stderr.decode("utf-8").splitlines()
    ln_cnt = 0
    line = lines[ln_cnt]

    while (f"Compiling wspace" not in line):
        ln_cnt += 1
        line = lines[ln_cnt]

    relevant_lines = lines[ln_cnt + 1 :]

    errors, compilation_steps = [], []
    cur_err_body, err_block = "", False
    common_comp_steps = ["free_global_ctxt", "total"]
    for line in relevant_lines:
        if re.match(r"error: could not compile \`wspace\` \(lib\) due to \d+ previous errors?",line):
            break
        if line.startswith("time:"):
            if err_block:
                errors.append(Error(cur_err_body))
                cur_err_body = ""
                err_block = False
            comp_step = re.split(r"\s+", line)[-1]  # line.split(r"\s+")[-1]
            if comp_step not in common_comp_steps:
                compilation_steps.append(comp_step)
        elif re.match(r"error(\[E\d\d\d\d\])?:", line) is not None:
            if err_block:
                errors.append(Error(cur_err_body))
            cur_err_body = line + "\n"
            err_block = True
        elif err_block:
            cur_err_body = cur_err_body + line + "\n"
        else:
            pass

    err_code_num, err_diag_num = defaultdict(int), defaultdict(int)
    for err in errors:
        err_code_num[err.code] += 1
        err_diag_num[err.diagnostic] += 1

    return errors, err_code_num, err_diag_num, compilation_steps, len(errors)

def parse_error_coarse(stderr):
    '''
    Parse rough error information during the Rust compilation process.
    Args:
        stderr: Error information.

    Returns:
        errors: A list of parsed error objects.
        err_c_num_dict: The number of occurrences of each error type.
        err_comp_phase_num_dict: The number of errors occurring in each compilation phase.
    '''
    msg_blocks = stderr.decode("utf-8").split("\n\n")
    # err_blocks, err_codes = [], []
    errors = []
    err_c_num_dict = defaultdict(int)
    err_comp_phase_num_dict = defaultdict(int)
    for body in msg_blocks[:-1]:
        if (
            "Finished" not in body
            and "warning:" not in body
            and len(body.split("\n")) > 3
        ):
            err_c_match = re.search(r"error\[E[0-9]+\]", body)
            # err_nc_match = re.search(r"error: ", body)
            diag_match = re.search(r"-Ztrack-diagnostics.*", body)

            # elif err_nc_match is not None and len(body.split("\n")) > 2:  # second part is needed because of error summary at the end of the file
            if err_c_match is not None:
                err_c = err_c_match.group(0)
            else:
                err_c = "E[NOCODE]"

            # precaution against some strange reason
            if diag_match is not None:
                compile_step = diag_match.group(0).split("/")[1]
            else:
                compile_step = "NotFound"

            body = re.sub(
                r"\s*(Compiling|Updating).*\n", "", body
            )  # the first block contains other logs, clean them

            err = Error(body)
            errors.append(err)

            err_c_num_dict[err_c] += 1
            err_comp_phase_num_dict[compile_step] += 1

    return errors, err_c_num_dict, err_comp_phase_num_dict
