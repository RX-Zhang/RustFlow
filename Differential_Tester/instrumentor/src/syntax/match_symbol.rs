use std::{path::Path, process::Command};

// 调用系统命令 nm 来提取指定可执行文件或库文件中的符号表，并过滤出符合特定条件的符号名称。
// 返回以 __C 结尾的函数名称列表
pub fn symbol_list<P: AsRef<Path>>(path: P) -> Vec<String> {
    let output = Command::new("nm")
        .arg("-g") // 全局符号
        .arg(path.as_ref())
        .output()
        .expect("failed to execute nm, try disabling ground_truth option");
    // 获取输出
    let captured_stdout = String::from_utf8(output.stdout).expect("nm fails utf8");
    // 检查每一行是否包含三个部分，并且第三部分（符号名）是否以 "__C" 结尾，并且不包含 "cgoexp"。
    captured_stdout
        .lines()
        .filter_map(|line| {
            let splitted = line.split(' ').collect::<Vec<_>>();
            if splitted.len() == 3 && splitted[2].ends_with("__C") && !line.contains("cgoexp") {
                let symbol_name = splitted[2];
                Some(symbol_name.trim_start_matches('_').to_owned())
            } else {
                None
            }
        })
        .collect::<Vec<_>>() // 返回符号列表
}
