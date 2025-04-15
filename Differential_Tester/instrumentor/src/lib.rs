use std::path::PathBuf;
use clap::Parser;

pub mod communication;
pub mod impls;
pub mod main_entry;
pub mod mangle;
pub mod syntax;
pub mod template;



#[derive(Parser)]
pub struct InstrConfig {
    #[clap(long, short, default_value = "/home/jn_cndt4/VScode_project/instruct_test/data/rust/fail/libopenaptx/aptx_qmf_polyphase_analysis/gpt4/Restart/aptx_qmf_polyphase_analysis.rs")]
    pub file: PathBuf, // 必要参数,输入文件的路径(rs)
    #[clap(long, short, default_value = "/home/jn_cndt4/VScode_project/instruct_test/rust-instrumentor-test_2.0/output")]
    pub output: PathBuf, // 必要参数,输出目录的路径
    #[clap(long, default_value_t = true)]
    pub capture_stdout: bool, // 是否捕获标准输出
    #[clap(long, default_value_t = true)]
    pub wrapper_structs: bool, // 是否生成包装器结构体以启用自定义的 serde 行为。
    #[clap(long, default_value_t = true)]
    pub arbitrary_precision: bool, // 是否支持在 serde 序列化和反序列化浮点数时使用任意精度
    #[clap(long, default_value = "/home/jn_cndt4/VScode_project/instruct_test/c-instrumentor-test_2.0/ground_truth/_build/libground_truth.so")]
    pub ground_truth: Option<PathBuf>, // 匹配外部符号(C函数库的地址)
    #[clap(long, default_value = "1000")]
    pub multi_examples: Option<usize>, // 测试失败时记录多个反例的数量
    #[clap(long, short)]
    pub modular: bool, // 可选参数，默认为 false,是否启用模块化测试套件。   clap中,bool类型为开关,默认为false
    #[clap(long, short)]
    pub check: bool, // 可选参数，默认为 false,是否对输出目录进行 Cargo 检查。
    #[clap(long, default_value_t = 300)]
    pub timeout: u64, // 超时时间
}

// 检查这个字符串是否是预定义的一组宏之一
fn handled_macros(path: &str) -> bool {
    matches!(
        path,
        "println" | "format" | "print" | "panic" | "write" | "writeln"
    )
}
