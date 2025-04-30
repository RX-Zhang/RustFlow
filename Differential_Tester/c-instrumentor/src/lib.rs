pub mod parser;
pub mod template;

use std::path::Path; // 用于处理文件路径
use anyhow::Context;  // 错误处理库
use serde::{Deserialize, Serialize}; // 一个序列化/反序列化库，用于将数据结构与不同的格式（如 JSON、TOML、YAML 等）转换

// 从JSON 文件中读取的输入数据。
#[derive(Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "Includes")]
    pub includes: Vec<String>, // 包含的头文件
    #[serde(rename = "Defines", default)]
    pub defines: Vec<String>, // 宏定义
    #[serde(rename = "TypeDefs", default)]
    pub type_defs: Vec<String>, // 类型定义
    #[serde(rename = "Globals", default)]
    pub globals: Vec<String>, // 全局变量定义
    #[serde(rename = "Structs")]
    pub structs: Vec<String>, // 结构体定义
    #[serde(rename = "Function Declarations")]
    pub func_decls: Vec<String>, // 函数声明
    #[serde(rename = "Function Implementations")]
    pub func_defs: Vec<String>, // 函数实现
    #[serde(rename = "Enums", default)]
    pub enums: Vec<String>, // 枚举定义
}

// Input 的方法实现
impl Input {
    // 去除函数声明和函数实现中的 static 和 inline 修饰符，返回纯粹的函数签名和实现。
    pub fn expose_everything(&mut self) {
        for func_decl in &mut self.func_decls {
            *func_decl = func_decl
                .trim_start_matches("static")
                .trim_start()
                .trim_start_matches("inline")
                .trim_start()
                .to_owned();
        }
        for func_def in &mut self.func_defs {
            *func_def = func_def
                .trim_start_matches("static")
                .trim_start()
                .trim_start_matches("inline")
                .trim_start()
                .to_owned();
        }
    }
    // 生成一个 C 语言头文件的内容。它将结构体中的所有相关字段（如包含的头文件、宏定义、类型定义、结构体定义等）连接成一个字符串。
    pub fn create_header(&self) -> String {
        self.includes.to_owned().join("\n")
            + "\n"
            + Self::INLINE_HACK
            + "\n"
            + &self.defines.to_owned().join("\n")
            + "\n"
            + &self.type_defs.to_owned().join("\n")
            + "\n"
            + &self.enums.to_owned().join("\n")
            + "\n"
            + &self
                .structs
                .iter()
                .map(|s| s.replace("const ", ""))
                .collect::<Vec<_>>()
                .join("\n")
            + "\n"
            + &self.func_decls.to_owned().join("\n")
    }
    // 一个包含 C 语言代码的字符串常量，用于重定义 malloc 和 free 函数，以及定义一个 ignore 函数。这通常用于将动态内存分配替换为某种上下文分配机制，可能是为了测试或兼容性目的。
    const REDEFINE_ALLOC: &'static str = r#"#include <runtime.h>
#define malloc(x) context_alloc((x))
void ignore(void* _ptr) {}
#define free(x) ignore((x))
"#;
    // 一个宏定义，用于处理 inline 关键字的定义。它可能用于解决在某些编译环境中 inline 关键字的处理问题。
    const INLINE_HACK: &'static str = "#define inline";
    // 生成一个 C 语言源文件的内容。它将结构体中的所有字段（包括头文件包含、宏定义、类型定义、全局变量、结构体定义、函数声明和实现等）连接成一个完整的 C 源文件的字符串。
    pub fn into_c(self) -> String {
        self.includes.join("\n")
            + "\n"
            + Self::REDEFINE_ALLOC
            + "\n"
            + Self::INLINE_HACK
            + "\n"
            + &self.defines.join("\n")
            + "\n"
            + &self.type_defs.join("\n")
            + "\n"
            + &self.globals.join("\n")
            + "\n"
            + &self.enums.to_owned().join("\n")
            + "\n"
            + &self.structs.join("\n")
            + "\n"
            + &self.func_decls.join("\n")
            + "\n"
            + &self.func_defs.join("\n")
            + "\n"
    }
}

// 用于表示一个 C 语言结构体的定义
#[derive(Serialize, Deserialize, Debug)]
pub struct CStruct {
    pub ident: String, // 表示结构体的名称
    pub fields: Vec<(String, String, Vec<String>)>, // 表示结构体的字段(变量名/容器名,变量类型,变量为空/容器大小)
}
// 表示一个 C 语言函数的签名
#[derive(Serialize, Deserialize, Debug)]
pub struct CFnSig {
    pub ident: String, // 表示函数的名称
    pub args: Vec<(String, Vec<String>)>, // 表示函数的参数列表(参数类型,)
    pub ret: String, // 表示函数的返回类型
}

// 生成一个 CMake 构建配置文件的内容。
pub fn cmake_lists<P: AsRef<Path>>(filename: P) -> anyhow::Result<String> {
    Ok(format!(
        "cmake_minimum_required(VERSION 3.27.7)

project(ground_truth)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED True)

add_library(
    ground_truth SHARED instrumented.h instrumented.cpp
    {} {} include/runtime.c
)

include_directories(include)

FIND_PACKAGE(Boost)
IF (Boost_FOUND)
    INCLUDE_DIRECTORIES(${{Boost_INCLUDE_DIR}})
    ADD_DEFINITIONS( \"-DHAS_BOOST\" )
    target_link_libraries(ground_truth PRIVATE ${{CMAKE_THREAD_LIBS_INIT}} Boost::boost)
ENDIF()",
        filename
            .as_ref()
            .with_extension("c")
            .to_str()
            .context("cmakelists")?,  // 将传入的 filename 修改为 .c 扩展名，并转换为字符串
        filename
            .as_ref()
            .with_extension("h")
            .to_str()
            .context("cmakelists")?  // 将传入的 filename 修改为 .h 扩展名，并转换为字符串
    ))
}

// 生成一个字符串，其中包含了一组 C 语言函数签名的外部原型声明。
// 这些函数签名是从 c_fn_sigs 参数中获取的，c_fn_sigs 是一个 CFnSig 结构体的切片（即数组的引用）
pub fn instrumented_h(c_fn_sigs: &[CFnSig]) -> anyhow::Result<String> {
    Ok(c_fn_sigs
        .iter()
        .map(|c_fn_sig| extern_prototype(c_fn_sig))
        .collect::<anyhow::Result<Vec<_>>>()?
        .join("\n"))
}

// 生成一个 C++ 源文件的字符串，其中包括必要的头文件、辅助函数、模板特化以及为 C 函数和结构体生成的 C++ 代码。
// 最终生成的字符串可以被写入到 .cpp 文件中，用于编译和链接到项目中。
pub fn instrumented_cpp<P: AsRef<Path>>(
    filename: P,  // 文件名
    c_structs: &[CStruct], // C 结构体的数组引用
    c_fn_sigs: &[CFnSig], // C 函数签名的数组引用
) -> anyhow::Result<String> {
    let file_str = filename.as_ref().to_str().unwrap_or("").to_string();
    let main_fun_str = file_str.split('.').next().unwrap_or(&file_str);

    Ok(format!(
        r#"#include <iostream>
#include <string.h>
#include "fuser.hpp"
#include "json.hpp"
#include <cstring>
#include <exception>
#include <limits>
extern "C" {{
    #include "runtime.h"
    #include "{}"
}}

char* string2cstr(std::string string, void* (*alloc)(size_t)) {{
    auto ptr = static_cast<char*>(alloc(string.length() + 1));
    strcpy(ptr, string.c_str());
    return ptr;
}}

{}

{}

{}

template <>
struct fuser::serializer<char*>
{{
    static nlohmann::ordered_json serialize(char* const& val)
    {{
        if (val) {{
            return serializer<std::string>::serialize(val);
        }} else {{
            return nullptr;
        }}
    }}
}};

template <>
struct fuser::deserializer<char*>
{{
    static char* deserialize(nlohmann::ordered_json const& json)
    {{
        if (json.is_null())
            return nullptr;
        else 
            return string2cstr(deserializer<std::string>::deserialize(json), context_alloc);
    }}
}};

template <typename T>
struct fuser::serializer<T*>
{{
    static nlohmann::ordered_json serialize(T* const& val)
    {{
        if (val) {{
            return serializer<T>::serialize(*val);
        }} else {{
            return nullptr;
        }}
    }}
}};

template <typename T>
struct fuser::deserializer<T*>
{{
    static T* deserialize(nlohmann::ordered_json const& json)
    {{
        if (json.is_null())
            return nullptr;
        else {{
            auto obj = deserializer<T>::deserialize(json);
            auto ptr = static_cast<T*>(context_alloc(sizeof(T)));
            *ptr = obj;
            return ptr;
        }}
    }}
}};

template <typename T>
struct fuser::serializer<
    T,
    typename std::enable_if<
        std::is_enum<T>::value
    >::type
> 
{{
    static nlohmann::ordered_json serialize(T const& val)
    {{
        return fuser::serialize<std::uint32_t>(static_cast<int>(val));
    }}
}};

template <typename T>
struct fuser::deserializer<
    T,
    typename std::enable_if<
        std::is_enum<T>::value
    >::type
> 
{{
    static T deserialize(nlohmann::ordered_json const& json)
    {{
        return static_cast<T>(fuser::deserialize<std::uint32_t>(json));
    }}
}};

// template <>
// struct fuser::serializer<unsigned long> : numeric_serializer<unsigned long, std::uintmax_t> {{}};

// template <>
// struct fuser::deserializer<unsigned long> : numeric_deserializer<unsigned long, std::uintmax_t> {{}};

// template <>
// struct fuser::serializer<char> : numeric_serializer<std::int8_t, std::uintmax_t> {{}};

// template <>
// struct fuser::deserializer<char> : numeric_deserializer<std::int8_t, std::uintmax_t> {{}};
    "#,
        filename
            .as_ref()
            .with_extension("h")
            .to_str()
            .context("instrumented.cpp")?,
        c_structs
            .iter()
            .map(|c_struct| fuse_struct(c_struct))
            .collect::<Vec<_>>()
            .join("\n"),
        c_fn_sigs
            .iter()
            .filter(|c_fn_sig| c_fn_sig.ident != main_fun_str)// 过滤掉 ident 为 "main" 的项
            .map(|c_fn_sig| extern_wrapper(c_fn_sig))
            .collect::<anyhow::Result<Vec<_>>>()?
            .join("\n"),
        c_fn_sigs
            .iter()
            .filter(|c_fn_sig| c_fn_sig.ident == main_fun_str) // 过滤掉 ident 为 "main" 的项
            .map(|c_fn_sig| extern_wrapper_main(c_fn_sig))
            .collect::<anyhow::Result<Vec<_>>>()?
            .join("\n")

    ))
}

// 用于检查给定的 C 类型名称是否是原始类型
fn c_type_is_primitive(type_name: &str) -> bool {
    let type_name = type_name.trim_start_matches("const ");
    match type_name {
        "uint8_t" | "uint16_t" | "uint32_t" | "uint64_t" | "int8_t" | "int16_t" | "int32_t"
        | "int64_t" | "void" | "bool" | "int" | "float" | "double" | "long" | "long long"
        | "char" | "size_t" | "unsigned long" | "unsigned int" | "unsigned" => true,
        _ => false,
    }
}

// 根据类型名称映射到对应的外部类型
fn map_c_type_to_extern_type(type_name: &str) -> &str {
    if c_type_is_primitive(type_name) { // 是原始类型
        type_name
    } else { // 不是原始类型
        "char*"
    }
}

// 根据给定的 C 语言函数签名（包括返回类型、函数名和参数），生成 C++ 代码中用来调用 C 函数的----外部原型声明
// 可以在 C++ 代码中用于调用 C 函数，确保类型匹配和接口正确。
fn extern_prototype(c_fn_sig: &CFnSig) -> anyhow::Result<String> {
    use std::fmt::Write;
    let mut w = String::new();
    let output_ty = map_c_type_to_extern_type(&c_fn_sig.ret);
    write!(&mut w, "{output_ty} {}__C", &c_fn_sig.ident)?; // 生成函数名和返回类型
    // 生成参数列表
    let args = c_fn_sig.args.iter().map(|(ty, array_decls)| {
        let ty = map_c_type_to_extern_type(ty);
        if array_decls.is_empty() {
            format!("{ty}")
        } else {
            format!("char*")
        }
    });
    writeln!(&mut w, "({});", args.collect::<Vec<_>>().join(", "))?;

    Ok(w)
}

// 生成 C++ 代码，该代码作为一个外部 C 函数的包装器，用于处理来自外部调用的 C++ 函数
// 这个包装器函数可以被 C 代码调用，并负责将输入数据从 JSON 格式转换为 C++ 函数所需的格式，同时将 C++ 函数的返回值转换回 JSON 格式以便返回给外部。
fn extern_wrapper(c_fn_sig: &CFnSig) -> anyhow::Result<String> {
    use std::fmt::Write;
    let mut w = String::new();

    // 写入函数签名(extern "C" + 返回值 + 函数名称__C)
    write!(&mut w, r#"extern "C" "#)?;
    let output_ty = map_c_type_to_extern_type(&c_fn_sig.ret);
    write!(&mut w, "{output_ty} {}__C", &c_fn_sig.ident)?;
    // 处理函数(定义)
    let args = c_fn_sig
        .args
        .iter()
        .enumerate()
        .map(|(index, (ty, array_decls))| {
            let ty = map_c_type_to_extern_type(ty);
            if !array_decls.is_empty() {
                format!("char* extern_input{index}")
            } else {
                let array_decls = array_decls
                    .iter()
                    .map(|s| format!("[{s}]"))
                    .collect::<Vec<_>>()
                    .join("");
                format!("{ty} extern_input{index}{array_decls}")
            }
        });
    writeln!(&mut w, "({}) {{", args.collect::<Vec<_>>().join(", "))?;

    // 对函数输入数据进行转化处理
    for (index, (ty, array_decls)) in c_fn_sig.args.iter().enumerate() {
        if array_decls.is_empty() {
            if c_type_is_primitive(ty) {
                writeln!(&mut w, "auto input{index} = extern_input{index};")?;
            } else {
                writeln!(&mut w, "auto input{index} = fuser::deserialize<{ty}>(nlohmann::ordered_json::parse(extern_input{index}));")?;
            }
        } else {
            let array_decls = array_decls
                .iter()
                .map(|s| format!("[{s}]"))
                .collect::<Vec<_>>()
                .join("");
            writeln!(&mut w, "{ty} input{index}{array_decls};")?;
            writeln!(&mut w, "fuser::array_deserialize<{ty}{array_decls}>(nlohmann::ordered_json::parse(extern_input{index}), input{index});")?;
        }
    }
    // 调用 C++ 函数--处理输入数据类型
    let inputs = (0..c_fn_sig.args.len())
        .map(|index| format!("input{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    // 调用 C++ 函数--处理函数名称和返回类型
    if c_fn_sig.ret == "void" {
        writeln!(&mut w, "{}({inputs});", c_fn_sig.ident)?;
    } else {
        writeln!(&mut w, "auto output = {}({inputs});", c_fn_sig.ident)?;
        if !c_type_is_primitive(&c_fn_sig.ret) {
            writeln!(
                &mut w,
                "std::string output_repr = fuser::serialize(&output).dump();"
            )?;
            writeln!(
                &mut w,
                "auto extern_output = string2cstr(output_repr, malloc);"
            )?;
        } else {
            writeln!(&mut w, "auto extern_output = output;")?;
        }
    }
    
    // 处理和修改输入数据
    for (index, (ty, array_decls)) in c_fn_sig.args.iter().enumerate() {
        if !array_decls.is_empty() {
            let array_decls = array_decls
                .iter()
                .map(|s| format!("[{s}]"))
                .collect::<Vec<_>>()
                .join("");
            writeln!(
                &mut w,
                "strcpy(extern_input{index}, fuser::array_serialize<{ty}{array_decls}>(input{index}).dump().c_str());"
            )?;
        } else if !c_type_is_primitive(ty) {
            writeln!(
                &mut w,
                "strcpy(extern_input{index}, fuser::serialize(&input{index}).dump().c_str());"
            )?;
        }
    }

    // 重置上下文和返回结果
    writeln!(&mut w, "context_reset();")?;

    // return
    if c_fn_sig.ret == "void" {
        writeln!(&mut w, "return;")?;
    } else {
        writeln!(&mut w, "return extern_output;")?;
    }
    writeln!(&mut w, "}}")?;

    Ok(w)
}

fn extern_wrapper_main(c_fn_sig: &CFnSig) -> anyhow::Result<String> {
    use std::fmt::Write;
    let mut w = String::new();

    // 写入函数签名(extern "C" + 返回值 + 函数名称__C)
    write!(&mut w, r#"extern "C" "#)?;
    let output_ty = map_c_type_to_extern_type(&c_fn_sig.ret);
    write!(&mut w, "{output_ty} {}__C", &c_fn_sig.ident)?;
    // 处理函数(定义)
    let args = c_fn_sig
        .args
        .iter()
        .enumerate()
        .map(|(index, (ty, array_decls))| {
            let ty = map_c_type_to_extern_type(ty);
            if !array_decls.is_empty() {
                format!("char* extern_input{index}")
            } else {
                let array_decls = array_decls
                    .iter()
                    .map(|s| format!("[{s}]"))
                    .collect::<Vec<_>>()
                    .join("");
                format!("{ty} extern_input{index}{array_decls}")
            }
        });
    writeln!(&mut w, "({},","int* success_flag" )?;
    writeln!(&mut w, "{}) {{", args.collect::<Vec<_>>().join(", "))?;
    
    //函数主体部分
    writeln!(&mut w, "*success_flag = 0;")?;
    if c_fn_sig.ret != "void"
    {
        if c_type_is_primitive(&c_fn_sig.ret){
            writeln!(&mut w, "{} extern_output = std::numeric_limits<{}>::min();",c_fn_sig.ret,c_fn_sig.ret)?;
        }else
        {
            writeln!(&mut w, "char* extern_output = nullptr;")?;
        }
    } 
    writeln!(&mut w, "try{{")?;
    
    // 对函数输入数据进行转化处理
    for (index, (ty, array_decls)) in c_fn_sig.args.iter().enumerate() {
        if array_decls.is_empty() {
            if c_type_is_primitive(ty) {
                writeln!(&mut w, "auto input{index} = extern_input{index};")?;
            } else {
                writeln!(&mut w, "auto input{index} = fuser::deserialize<{ty}>(nlohmann::ordered_json::parse(extern_input{index}));")?;
            }
        } else {
            let array_decls = array_decls
                .iter()
                .map(|s| format!("[{s}]"))
                .collect::<Vec<_>>()
                .join("");
            writeln!(&mut w, "{ty} input{index}{array_decls};")?;
            writeln!(&mut w, "fuser::array_deserialize<{ty}{array_decls}>(nlohmann::ordered_json::parse(extern_input{index}), input{index});")?;
        }
    }
    // 调用 C++ 函数--处理输入数据类型
    let inputs = (0..c_fn_sig.args.len())
        .map(|index| format!("input{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    // 调用 C++ 函数--处理函数名称和返回类型
    if c_fn_sig.ret == "void" {
        writeln!(&mut w, "{}({inputs});", c_fn_sig.ident)?;
    } else {
        writeln!(&mut w, "auto output = {}({inputs});", c_fn_sig.ident)?;
        if !c_type_is_primitive(&c_fn_sig.ret) {
            writeln!(
                &mut w,
                "std::string output_repr = fuser::serialize(&output).dump();"
            )?;
            writeln!(
                &mut w,
                "extern_output = string2cstr(output_repr, malloc);"
            )?;
        } else {
            writeln!(&mut w, "extern_output = static_cast<{}>(output);",c_fn_sig.ret)?;
        }
    }
    
    // 处理和修改输入数据
    for (index, (ty, array_decls)) in c_fn_sig.args.iter().enumerate() {
        if !array_decls.is_empty() {
            let array_decls = array_decls
                .iter()
                .map(|s| format!("[{s}]"))
                .collect::<Vec<_>>()
                .join("");
            writeln!(
                &mut w,
                "strcpy(extern_input{index}, fuser::array_serialize<{ty}{array_decls}>(input{index}).dump().c_str());"
            )?;
        } else if !c_type_is_primitive(ty) {
            writeln!(
                &mut w,
                "strcpy(extern_input{index}, fuser::serialize(&input{index}).dump().c_str());"
            )?;
        }
    }

    // if c_type_is_primitive(&c_fn_sig.ret){
    //     writeln!(&mut w, "extern_output = static_cast<{}>(output);",c_fn_sig.ret)?;
    // }else if c_fn_sig.ret != "void"
    // {
    //     writeln!(&mut w, "std::string output_str = \"\";")?;
    // }
    // extern_output = static_cast<int>(output);


    // 重置上下文和返回结果
    writeln!(&mut w, "context_reset();")?;

    writeln!(&mut w, "*success_flag = 1;")?;
    writeln!(&mut w, "}}catch(...){{")?;
    writeln!(&mut w, "*success_flag = 0;")?;
    writeln!(&mut w, "}}")?;

    // return
    if c_fn_sig.ret == "void" {
        writeln!(&mut w, "return;")?;
    } else {
        writeln!(&mut w, "return extern_output;")?;
    }
    // //catch
    // writeln!(&mut w, "}}catch(...){{")?;
    // if c_fn_sig.ret == "void" {
    //     writeln!(&mut w, "return;")?;
    // }else if c_type_is_primitive(&c_fn_sig.ret) {
    //     writeln!(&mut w, "return std::numeric_limits<{}>::min();",c_fn_sig.ret)?;
    // }
    //  else {
    //     writeln!(&mut w, "exit(1);")?;
    // }
    writeln!(&mut w, "}}")?;

    Ok(w)
}

// 用于生成一个 Boost Fusion 库的宏调用，用于将一个 C 结构体定义适配到 Boost Fusion 库中,使得结构体能够与 Boost Fusion 库的 序列化、反序列化 等功能兼容，并能够在 C++ 中进行更复杂的数据处理操作
// 函数作用: 生成BOOST_FUSION_ADAPT_STRUCT(结构体名,结构体字段名称1,结构体字段名称2...)函数调用
fn fuse_struct(c_struct: &CStruct) -> String {
    let field_names = c_struct
        .fields
        .iter()
        .map(|(name, _, _)| name.to_owned())
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "BOOST_FUSION_ADAPT_STRUCT({}, {})",
        &c_struct.ident, field_names
    )
}
