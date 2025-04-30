use std::{fs, path::PathBuf};  //文件相关
use clap::Parser; //用于解析命令行参数的库
use anyhow::Context;  //错误处理相关

// src/lib中自定义的处理函数
use c_instrumentor::{
    cmake_lists,  // 用于生成 CMakeLists.txt 文件的内容，帮助构建 C/C++ 项目。
    instrumented_cpp,  // 用于生成特定的 C++ 文件内容。这个函数可能会根据输入数据生成 instrumented.cpp 文件。
    instrumented_h,  // 用于生成特定的头文件内容，通常与 C++ 文件相对应。
    parser::{fn_sig, struct_def}, //用于解析 C 函数签名。用于解析 C 结构体定义
    template::{ARENA_H, FUSER_HPP, JSON_HPP, RUNTIME_C, RUNTIME_H}, //字符串模版
    Input,
};

// 使用 clap 库来定义和解析命令行参数。
// 程序需要两个参数：file（输入文件json路径）   -f / --file
//               output（输出目录路径） -o / --output
#[derive(Parser)]
struct Cli {
    /// Input file path
    #[arg(long, short, default_value = "/home/jn_cndt4/VScode_project/instruct_test/data/c/libopenaptx/aptx_qmf_polyphase_analysis/aptx_qmf_polyphase_analysis.json")]
    file: PathBuf,

    /// Output dir path
    #[arg(long, short, default_value = "/home/jn_cndt4/VScode_project/instruct_test/c-instrumentor-test_2.0/ground_truth")]
    output: PathBuf,
}

// 创建包含Cmake的C/C++项目
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 创建输出目录及其子目录 include
    fs::create_dir(&cli.output)?;
    fs::create_dir(cli.output.join("include"))?;
    // 将预定义的 C++ 和 C 头文件内容写入 include 目录下。这些内容来自 c_instrumentor/src/template.rs 模块中的常量。
    fs::write(cli.output.join("include/arena.h"), ARENA_H)?;
    fs::write(cli.output.join("include/fuser.hpp"), FUSER_HPP)?;
    fs::write(cli.output.join("include/json.hpp"), JSON_HPP)?;
    fs::write(cli.output.join("include/runtime.h"), RUNTIME_H)?;
    fs::write(cli.output.join("include/runtime.c"), RUNTIME_C)?;
    // 读取和解析输入 JSON 文件，将其转换为 Input 结构体。然后调用 expose_everything 方法来处理数据。
    let mut c_data: Input = serde_json::from_str(&fs::read_to_string(&cli.file)?)?;
    // c_data
    //     .type_defs
    //     .is_empty()
    //     .then_some(())
    //     .context("type defs not handled")?;
    // remove static designators of fucntions
    c_data.expose_everything(); // 去除函数声明和函数实现中的 static 和 inline 修饰符，返回纯粹的函数签名和实现
    // 生成 CMakeLists.txt 文件，用于构建项目。文件内容由 c_instrumentor::cmake_lists 函数生成。
    let benchmark_name = cli.file.file_name().context("basename failed")?;
    fs::write(
        cli.output.join("CMakeLists.txt"),
        cmake_lists(benchmark_name)?,
    )?;
    // 将Input.structs 存入 Vec<CStruct>中
    let c_structs = c_data
        .structs
        .iter()
        .map(|item| {
            struct_def(&item[..])
                .map(|(_, parsed)| parsed)
                .ok()
                .context("failed to parse struct definitions")
        })
        .collect::<Result<Vec<_>, _>>()?;
    // 将Input.func_decls 存入 Vec<CFnSig>中
    let c_fn_sigs = c_data
        .func_decls
        .iter()
        .map(|item| {
            fn_sig(&item[..])
                .map(|(_, parsed)| parsed)
                .ok()
                .context("failed to parse function signatures")
        })
        .collect::<Result<Vec<_>, _>>()?;
    // 使用 c_instrumentor::instrumented_cpp 和 c_instrumentor::instrumented_h 函数生成 C++ 源文件和头文件。
    fs::write(
        cli.output.join("instrumented.cpp"),
        instrumented_cpp(benchmark_name, &c_structs, &c_fn_sigs)?,
    )?;
    fs::write(
        cli.output.join("instrumented.h"),
        instrumented_h(&c_fn_sigs)?,
    )?;
    // 生成与输入数据相关的 C 文件头和源文件。这些文件的内容由 c_data 中的方法 create_header 和 into_c 生成。
    fs::write(
        cli.output.join(benchmark_name).with_extension("h"),
        c_data.create_header(),
    )?;
    fs::write(
        cli.output.join(benchmark_name).with_extension("c"),
        c_data.into_c(),
    )?;
    Ok(())
}
