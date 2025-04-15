use std::{
    fs::{self, File}, // 文件系统的操作
    io::Read,
};
use anyhow::Context; // 错误处理
use clap::Parser; // 解析命令行参数
use syn::{visit::Visit, visit_mut::VisitMut}; // 解析和操作 Rust 代码的抽象语法树
use instrumentor::InstrConfig;  // lib.rs

// 读取 Rust 源代码文件，进行仪器化处理，然后将处理后的代码和相关文件输出到指定的目录。
// 它涉及到解析和操作 AST，生成代码，处理输出路径，并进行必要的检查和验证。
fn main() -> anyhow::Result<()> {
    let config = InstrConfig::parse(); // 解析配置
    if !config.file.is_file() {
        anyhow::bail!("expect a rust file");
    }
    // 读取文件内容
    let mut file = File::open(&config.file).context("failed to open file")?;
    let mut src = String::new(); // 保存.rs文件内容(string)
    file.read_to_string(&mut src)
        .context("unable to read file")?;
    // 解析 Rust 代码为 AST
    let mut ast = syn::parse_file(&src).context("unable to parse file")?;
    // 生成实现 impls.rs 文件代码 (ast -> proc_macro2::TokenStream -> syn 库的结构(ast) -> 格式化的 Rust 代码(string))
    let impls_code = prettyplease::unparse(&syn::parse2(instrumentor::impls::code(&ast, &config))?); // Rust 源代码字符串
    // 处理 AST (将lazy_static! 宏转换为 once_cell::sync::Lazy)
    instrumentor::syntax::lazy_static::Replace.visit_file_mut(&mut ast); 
    // 处理 Rust 代码中的impl方法(重令名方法名,替换self为实际类型)
    let mut ast = instrumentor::mangle::mangle_associated_methods(ast); 
    // 获取函数细节: HashMap<函数名称, SignatureData(返回类型,参数列表,是否是纯函数)>
    let mut function_symbols: std::collections::HashMap<String, instrumentor::syntax::SignatureData> = instrumentor::syntax::FunctionSymbols::collect(&ast);
    // 创建 MarkPure 实例,遍历AST,更新function_symbols中的 pure(是否为纯函数)
    instrumentor::syntax::MarkPure(&mut function_symbols).visit_file(&ast);
    // 如果不执行模块化测试,找到函数入口点
    let main_entry = (!config.modular)
        .then(|| instrumentor::main_entry::find_main_entry(&ast, &function_symbols)); 
    // 声明一个全局状态结构体: GlobalState {captured_stdout: RefCell<String>, arena: typed_arena::Arena<u8>}
    // 及实现方法 GlobalState() 和 reset()
    let declare_global_state = config.declare_global_state(); 
    // 遍历和修改ast(调用visit_item_fn_mut,visit_expr_call_mut和visit_macro_mut函数进行处理)
    config.instrument_calls(&mut ast, &function_symbols);
    // 为枚举、结构体及其字段上自动添加特定特性和属性(#[...])
    instrumentor::syntax::DeriveSerde(&config).visit_file_mut(&mut ast);
    // 生成调用C函数的 Rust 代码片段
    let extern_c_block = config.extern_c_block(&function_symbols, main_entry.as_deref());
    // 生成一个 replay 的 Rust 测试函数
    let counter_examples_replay = config.counter_examples_replay(&function_symbols, main_entry.as_deref());
    // 生成一个 fuzz 的 Rust 测试函数
    let harnesses = config.harnesses(&function_symbols, main_entry.as_deref());
    // 合成最终测试代码 [test]
    let harnesses = if let Some(counter_examples_replay) = counter_examples_replay 
    {
        quote::quote!(
            #[cfg(feature = "replay")]
            #counter_examples_replay

            #[cfg(not(feature = "replay"))]
            #harnesses
        )
    } else {
        harnesses
    };

    // 生成一些外部函数的包装器 (wrapper) 函数
    let extern_wrappers = config.extern_wrappers(&function_symbols);
    // 生成 正反例结构体 代码片段
    let counter_examples_container = config.counter_examples_container();
    // 生成序列化段代码
    let communication = config.communication();
    // ----------- 生成整个 lib.rs 文件代码 ----------------
    let instrumented = quote::quote!(//! Instrumented version
    #![feature(min_specialization)]

    #[cfg(all(feature = "replay", feature = "fuzzing"))]
    std::compile_error!("feature \"replay\" and feature \"fuzzing\" cannot be enabled at the same time");

    use bolero::{TypeGenerator, ValueGenerator};
    use serde::{Deserialize, Serialize};
    use std::fmt::Write as _;
    use std::cell::RefCell;
    pub mod impls;
    use impls::*;

    #declare_global_state

    #[cfg(feature = "fuzzing")]
    #extern_c_block

    #ast

    #extern_wrappers

    #communication

    #[cfg(test)]
    mod test {
        use super::*;
        #counter_examples_container

        #harnesses
    }

    );
    // 解析代码片段: TokenStream -> AST -> String
    let formatted = prettyplease::unparse(&syn::parse2(instrumented)?);
    // 输出结果
    if config.output.exists() {
        println!("{formatted}");
        anyhow::bail!("output path exists, instrumented program printed")
    }

    // 创建文件并写入生成代码
    fs::create_dir(&config.output)?;
    let toml_file = config.output.join("Cargo.toml");
    fs::create_dir(config.output.join("src"))?;
    let target_file = config.output.join("src/lib.rs");
    let impls_file = config.output.join("src/impls.rs");

    fs::write(&toml_file, instrumentor::template::cargo_toml(&config))?;
    fs::write(&impls_file, impls_code)?;
    fs::write(&target_file, formatted)?;

    // 检查
    if config.check {
        use std::process::Command;
        Command::new("cargo")
            .arg("check")
            .arg("--tests")
            .arg("--manifest-path")
            .arg(&toml_file)
            .env("RUSTFLAGS", "-Awarnings")
            .status()?;
    }

    Ok(())
}
