//! Purely syntactic instrumentation

// pub mod associated;

pub mod lazy_static;
pub mod match_symbol;

use std::collections::HashMap;

use quote::format_ident;
use syn::{LitInt,parse_quote, visit::Visit, visit_mut::VisitMut, File};

use crate::handled_macros; // lib.rs
use crate::InstrConfig; // lib.rs

use self::match_symbol::symbol_list;

// 根据 SignatureData 的 pure 字段来判断是否需要全局状态。
fn require_global_state(data: &SignatureData) -> bool {
    !data.pure
}

impl InstrConfig {
    // 生成一个 proc_macro2::TokenStream，用于声明一个全局状态结构体及其相关方法。
    pub fn declare_global_state(&self) -> proc_macro2::TokenStream {
        let fields = self.global_state_fields();
        let fields_create = self.global_state_fields_create();
        let fields_reset = self.global_state_fields_reset();
        quote::quote!(
            struct GlobalState {
                #fields
            }
            impl GlobalState {
                fn new() -> Self {
                    GlobalState {
                        #fields_create
                    }
                }

                fn reset(&mut self) {
                    #fields_reset
                }
            }
        )
    }

    // 根据 self.capture_stdout 的值 生成用于声明 GlobalState 结构体字段的代码。
    fn global_state_fields(&self) -> proc_macro2::TokenStream {
        if self.capture_stdout {
            quote::quote!(captured_stdout: RefCell<String>, arena: typed_arena::Arena<u8>)
        } else {
            quote::quote!(arena: typed_arena::Arena<u8>)
        }
    }
    // 生成 GlobalState 结构体的构造函数中字段的初始化代码。
    fn global_state_fields_create(&self) -> proc_macro2::TokenStream {
        if self.capture_stdout {
            quote::quote!(captured_stdout: RefCell::new(String::default()), arena: typed_arena::Arena::default())
        } else {
            quote::quote!(arena: typed_arena::Arena::default())
        }
    }
    // 生成 GlobalState 结构体的重置方法中的代码。
    fn global_state_fields_reset(&self) -> proc_macro2::TokenStream {
        if self.capture_stdout {
            quote::quote!(self.captured_stdout.borrow_mut().clear(); self.arena = typed_arena::Arena::default();)
        } else {
            quote::quote!(self.arena = typed_arena::Arena::default();)
        }
    }

    // 对一个 File AST（抽象语法树）进行遍历和修改，可能是为了在函数调用处插入一些仪器代码或进行其他类型的代码注入。
    pub fn instrument_calls(
        &self,
        ast: &mut File,
        function_symbols: &HashMap<String, SignatureData>,
    ) {
        InstrumentCalls {
            config: self,
            function_symbols,
        }
        .visit_file_mut(ast);
    }

    // 生成一个包含多个计数示例（CounterExample）和正面示例（PositiveExample）的 Rust 容器
    // 同时定义与这些示例相关的类型和常量。
    pub fn counter_examples_container(&self) -> Option<proc_macro2::TokenStream> {
        self.multi_examples.map(|max_num_examples| {
            quote::quote!(
                use std::sync::Mutex;
                // type ExecutionResult = Option<String>;
                #[derive(PartialEq, Debug, Clone, serde::Serialize, serde::Deserialize)]
                enum ExecutionResult {
                    ExecutionSuccess(String),
                    ExecutionFailure,
                }
                use ExecutionResult::*;
                impl std::convert::From<Option<String>> for ExecutionResult {
                    fn from(value: Option<String>) -> Self {
                        match value {
                            Some(result) => ExecutionSuccess(result),
                            None => ExecutionFailure,
                        }
                    }
                }
                #[derive(PartialEq, Debug, serde::Serialize, serde::Deserialize)]
                struct CounterExample {
                    args: std::vec::Vec<String>,
                    actual: ExecutionResult,
                    expected: ExecutionResult,
                }
                #[derive(PartialEq, Debug, serde::Serialize, serde::Deserialize)]
                struct PositiveExample {
                    args: std::vec::Vec<String>,
                    actual: ExecutionResult,
                }
                static COUNTER_EXAMPLES: Mutex<std::vec::Vec<CounterExample>> = Mutex::new(vec![]);
                static POSITIVE_EXAMPLES: Mutex<std::vec::Vec<PositiveExample>> = Mutex::new(vec![]);
                const MAX_NUM_EXAMPLES: usize = #max_num_examples;
            )
        })
    }

    // 生成一个测试函数，该函数会读取测试数据、执行被测试的函数，并将测试结果与预期结果进行比较。
    pub fn counter_examples_replay(
        &self,
        function_symbols: &HashMap<String, SignatureData>,
        main_entry: Option<&str>,
    ) -> Option<proc_macro2::TokenStream> {
        if !self.multi_examples.is_some() || !main_entry.is_some() {
            return None;
        }
        let main_entry = main_entry.unwrap(); // 获取主函数名称
        let data = function_symbols.get(main_entry).unwrap(); // 获取对应的函数信息
        let rust_symbol = quote::format_ident!("{main_entry}__Rust");
        // 遍历 data.inputs，为每个输入类型生成对应的 Rust 变量声明。
        let counstruct_args_c = data.inputs.iter().enumerate().map(|(index, ty)| {
            let ty = match ty {
                &TypeKind::Primitive(ty_str) => {
                    let ident = quote::format_ident!("{ty_str}");
                    quote::quote!(#ident)
                }
                // generate `Box` instead so that `TypeGenerator` generates
                TypeKind::RefMut(_, ty_str) => quote::quote!(Box<#ty_str>),
                TypeKind::Ref(_, ty_str) => quote::quote!(Box<#ty_str>),
                TypeKind::Complex(ty_str) => quote::quote!(#ty_str),
                TypeKind::OutofScope => unreachable!(),
            };
            let ident = quote::format_ident!("input{index}");
            quote::quote!(
                let mut #ident: #ty = serde_json::from_str(args.next().unwrap()).unwrap();
            )
        });
        let counstruct_args_p = data.inputs.iter().enumerate().map(|(index, ty)| {
            let ty = match ty {
                &TypeKind::Primitive(ty_str) => {
                    let ident = quote::format_ident!("{ty_str}");
                    quote::quote!(#ident)
                }
                // generate `Box` instead so that `TypeGenerator` generates
                TypeKind::RefMut(_, ty_str) => quote::quote!(Box<#ty_str>),
                TypeKind::Ref(_, ty_str) => quote::quote!(Box<#ty_str>),
                TypeKind::Complex(ty_str) => quote::quote!(#ty_str),
                TypeKind::OutofScope => unreachable!(),
            };
            let ident = quote::format_ident!("input{index}");
            quote::quote!(
                let mut #ident: #ty = serde_json::from_str(args.next().unwrap()).unwrap();
            )
        });
        // 生成 Rust 函数参数
        let rust_args = data.inputs.iter().enumerate().map(|(index, kind)| {
            let ident = quote::format_ident!("input{index}");
            match kind {
                TypeKind::RefMut(..) => quote::quote!(&mut *#ident),
                TypeKind::Ref(..) => quote::quote!(&*#ident),
                _ => quote::quote!(#ident),
            }
        });
        // 如果 data 需要全局状态，生成 global_state 的声明并将其添加到参数列表中。
        let rust_args = if require_global_state(data) {
            let rust_args = std::iter::once(quote::quote!(&mut global_state)).chain(rust_args);
            quote::quote!(#( #rust_args ),*)
        } else {
            quote::quote!(#( #rust_args ),*)
        };
        let create_global_state = require_global_state(data).then(|| quote::quote!(let mut global_state = std::panic::AssertUnwindSafe(GlobalState::new());));
        let reset_global_state =
            require_global_state(data).then(|| quote::quote!(global_state.reset();));
        // 处理函数返回类型
        let unwrap_result = data.output.as_ref().and_then(|output_ty| {
            if let TypeKind::Complex(tokens) = output_ty {
                if let Ok(syn::TypePath { path, .. }) = syn::parse2(tokens.clone()) {
                    if let Some(segment) = path.segments.last() {
                        if segment.ident == "Result" {
                            return Some(quote::quote!(.unwrap()))
                        }
                    }
                }
            }
            None
        });


        let warper_output_void = data.inputs // 比较C语言和Rust的输入参数 -> 代码片段
        .iter()
        .enumerate()
        .find(|(_, data)| data.is_mutable_ref()) // 找到第一个满足条件的元素
        .map(|(index, _)| {
            let index_literal = LitInt::new(&index.to_string(), proc_macro2::Span::call_site());
            let ident = quote::format_ident!("input{}", index); // 修正格式化

            // 调用比较函数
            quote::quote!(
                let actual_result = match execution_result {
                    None => execution_result,
                    Some(serialized_result) => {
                        if serialized_result == "null" &&  io_example.expected != ExecutionFailure{
                            Some(format!("ret_input{}:{}", #index_literal, serialize__Rust(& #ident)))
                        } else {
                            Some(format!("output:{}", serialized_result))
                        }
                    }
                };
            )
        });
        let warper_output_ret = quote::quote!(
            let actual_result = match execution_result {
                None => execution_result,
                Some(serialized_result) => {
                        Some(format!("output:{}", serialized_result))
                }
            };
        );

        let counter_warper_output = match data.output{
            None => warper_output_void,
            _ => Some(warper_output_ret)
        };

        let positive_wrapper_output = quote::quote!(
            let actual_result = match execution_result {
                None => execution_result,
                Some(serialized_result) => {
                        Some(format!("output:{}", serialized_result))
                }
            };
        );

        let counter_to_positive = match data.output{
            None => quote::quote!(
                positive_examples.push(PositiveExample{
                args: io_example.args,
                actual: ExecutionSuccess("output:null".to_string()),
            })),
            _ =>  quote::quote!(
                positive_examples.push(PositiveExample{
                args: io_example.args,
                actual: io_example.expected,
            }))
        };

        // 生成测试函数代码
        Some(quote::quote!(
            #[test] fn replay() {
                #create_global_state
                use std::io::{stdin, Read};
                let mut json = String::new();
                stdin().read_to_string(&mut json);
                let io_examples: std::vec::Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();

                let mut io_examples_counter: Vec<CounterExample> = Vec::new();
                let mut io_examples_positive: Vec<PositiveExample> = Vec::new();

                for value in io_examples.iter() {
                    if let Ok(counter_example) = serde_json::from_value::<CounterExample>(value.clone()) {
                        io_examples_counter.push(counter_example);
                    } else if let Ok(positive_example) = serde_json::from_value::<PositiveExample>(value.clone()) {
                        io_examples_positive.push(positive_example);
                    }
                }

                let mut counter_examples = vec![];
                let mut positive_examples = vec![];
                for io_example in io_examples_counter {
                    let mut args = io_example.args.iter();
                    #( #counstruct_args_c )*
                    let execution_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        #rust_symbol(#rust_args)#unwrap_result
                    )).ok().map(|result| serialize__Rust(&result));
                    #reset_global_state

                    #counter_warper_output

                    let execution_result = ExecutionResult::from(actual_result);
                    let results_are_equal = match (&execution_result, &io_example.expected) {
                        (ExecutionSuccess(result1), ExecutionSuccess(result2)) => {
                            let result1_res ;
                            let result2_res ;
                            if let Some(pos) = result1.find(':') {
                                result1_res = &result1[pos + 1..];
                            }else{
                                result1_res = result1;
                            }
                            if let Some(pos) = result2.find(':') {
                                result2_res = &result2[pos + 1..];
                            }else{
                                result2_res = result2;
                            }
                            structural_eq(
                                &serde_json::from_str::<serde_json::Value>(result1_res).unwrap(),
                                &serde_json::from_str::<serde_json::Value>(result2_res).unwrap(),
                            )
                        }
                        _ => false
                    };
                    if !results_are_equal {
                        counter_examples
                            .push(CounterExample {
                                args: io_example.args,
                                actual: execution_result,
                                expected: io_example.expected,
                            });
                    } else {
                        # counter_to_positive
                    }
                }

                for io_example in io_examples_positive {
                    let mut args = io_example.args.iter();
                    #( #counstruct_args_p )*
                    let execution_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                        #rust_symbol(#rust_args)#unwrap_result
                    )).ok().map(|result| serialize__Rust(&result));
                    #reset_global_state

                    #positive_wrapper_output

                    let execution_result = ExecutionResult::from(actual_result);
                    let results_are_equal = match (&execution_result, &io_example.actual) {
                        (ExecutionSuccess(result1), ExecutionSuccess(result2)) => {
                            let result1_res ;
                            let result2_res ;
                            if let Some(pos) = result1.find(':') {
                                result1_res = &result1[pos + 1..];
                            }else{
                                result1_res = result1;
                            }
                            if let Some(pos) = result2.find(':') {
                                result2_res = &result2[pos + 1..];
                            }else{
                                result2_res = result2;
                            }
                            structural_eq(
                                &serde_json::from_str::<serde_json::Value>(result1_res).unwrap(),
                                &serde_json::from_str::<serde_json::Value>(result2_res).unwrap(),
                            )
                        }
                        _ => false
                    };
                    if results_are_equal {
                        positive_examples.push(io_example);
                    } 
                }

                panic!(
                    "counter examples: {}\npositive examples: {}\n",
                    serde_json::to_string(&counter_examples).unwrap(),
                    serde_json::to_string(&positive_examples).unwrap(),
                );
            }
        ))
    }

    // 用于生成 Rust 代码片段，通常是在宏扩展中使用。
    pub fn extern_c_block(
        &self,
        function_symbols: &HashMap<String, SignatureData>,
        main_entry: Option<&str>,
    ) -> proc_macro2::TokenStream {
        let symbol_list = self.ground_truth.as_deref().map(|path| symbol_list(path));
        extern_c_block(function_symbols, symbol_list, main_entry)
    }

    // 用于生成 Rust 代码片段，主要用于测试相关的功能。
    pub fn harnesses(
        &self,
        function_symbols: &HashMap<String, SignatureData>,
        main_entry: Option<&str>,
    ) -> proc_macro2::TokenStream {
        harnesses(
            function_symbols,
            self.multi_examples.is_some(),
            self.timeout,
            ComparisonKind::Structural,
            main_entry,
        )
    }

    /// Generate wrapper functions for non-pure ones
    // 生成一些外部函数的包装器 (wrapper) 函数。
    // 这些包装器函数用于在 Rust 和外部 C 函数交互时，处理输入参数、调用外部 C 函数、清理内存，
    // 以及将外部 C 函数的返回值转换回 Rust 的格式。
    // 具体来说，该函数会根据给定的 function_symbols，为每个符合条件的外部函数生成对应的包装器函数。
    pub fn extern_wrappers(
        &self,
        function_symbols: &HashMap<String, SignatureData>,
    ) -> Option<proc_macro2::TokenStream> {
        if !self.modular {
            return None;
        }
        // 过滤需要包装的函数
        let iter = function_symbols
            .iter()
            .filter(|(_, data)| {
                !data.is_out_of_scope()
                    && !data
                        .output
                        .as_ref()
                        .is_some_and(|output| output.is_mutable_ref())
            })
            .filter(|(_, data)| require_global_state(data))
            .map(|(symbol, data)| { //  为每个符号生成包装器函数
                // 定义包装器函数的名称和生命周期
                let extern_fn = quote::format_ident!("{symbol}__C");
                let wrapper = quote::format_ident!("{symbol}__C__wrapper");
                // TODO handle all lifetimes
                let mut lifetime_generics: syn::Generics = parse_quote!(<>);
                if require_global_state(data) {
                    lifetime_generics.params.push(parse_quote!('state))
                }
                if data.is_elided() {
                    lifetime_generics.params.push(parse_quote!('elided))
                }
                let where_clause = (lifetime_generics.params.len() > 1).then(|| quote::quote!(where 'state: 'elided));
                let lifetime_generics =
                    (!lifetime_generics.params.is_empty()).then(|| lifetime_generics);
                // 处理输入参数和输出类型
                let params = data.params();
                let arg_types = data.inputs.iter().map(|ty| match ty {
                    &TypeKind::Primitive(ty_str) => {
                        let ident = quote::format_ident!("{ty_str}");
                        quote::quote!(#ident)
                    }
                    TypeKind::RefMut(lifetime, ty_str) => quote::quote!(&#lifetime mut #ty_str),
                    TypeKind::Ref(lifetime, ty_str) => quote::quote!(&#lifetime #ty_str),
                    TypeKind::Complex(ty_str) => quote::quote!(#ty_str),
                    TypeKind::OutofScope => unreachable!(),
                });
                let input_signature = if require_global_state(data) {
                    let params = std::iter::once(quote::quote!(global_state)).chain(params);
                    let arg_types =
                        std::iter::once(quote::quote!(&'state GlobalState)).chain(arg_types);
                    quote::quote!(#( #params: #arg_types ),*)
                } else {
                    quote::quote!(#( #params: #arg_types ),*)
                };
                let output_ty = data.output.as_ref().map(|output| match output {
                    TypeKind::Primitive(ty_str) => {
                        let ident = quote::format_ident!("{ty_str}");
                        quote::quote!(-> #ident)
                    }
                    TypeKind::Ref(lifetime, ty_str) => quote::quote!(-> &#lifetime #ty_str),
                    TypeKind::Complex(ty_str) => quote::quote!(-> #ty_str),
                    TypeKind::OutofScope | TypeKind::RefMut(..) => unreachable!(),
                });
                // 生成包装器函数的主体
                let prepare_extern_args = data.prepare_extern_args();
                let extern_args = data.extern_args();
                let extern_args_cleanup = data.extern_args_cleanup();

                let set_args = data
                    .inputs
                    .iter()
                    .enumerate()
                    .filter(|(_, data)| data.is_mutable_ref())
                    .map(|(index, _)| {
                        let ident = quote::format_ident!("input{index}");
                        let extern_ident = quote::format_ident!("extern_input{index}");
                        quote::quote!(*#ident = deserialize__Rust(&*global_state, #extern_ident);)
                    });

                let produce_output = data.output.as_ref().map(|data| match data {
                    TypeKind::Primitive(_) => quote::quote!(let rust_output = extern_output;),
                    TypeKind::Complex(_) => {
                        quote::quote!(let rust_output = deserialize__Rust(&*global_state, extern_output);)
                    }
                    TypeKind::Ref(..) => unimplemented!(),
                    TypeKind::RefMut(..) => unreachable!(),
                    TypeKind::OutofScope => unreachable!(),
                });
                let extern_output_cleanup = data.extern_output_cleanup();
                let yield_output = data.output.as_ref().map(|_| quote::quote!(rust_output));

                quote::quote!(fn #wrapper #lifetime_generics(#input_signature) #output_ty #where_clause {
                    unsafe {
                        #( #prepare_extern_args )*
                        let extern_output = #extern_fn(#( #extern_args ),*);
                        #( #set_args )*
                        #( #extern_args_cleanup )*
                        #produce_output
                        #extern_output_cleanup
                        #yield_output
                    }
                })
            });

        Some(quote::quote!(#( #iter )*))
    }
}

pub struct DeriveSerde<'me>(pub &'me InstrConfig);

// 在枚举、结构体及其字段上自动添加特定特性和属性的功能。
impl VisitMut for DeriveSerde<'_> {
    // 在枚举类型上添加特性（derive traits）和属性.
    // 如果发现 #[derive] 属性，提取现有的特性名称并存储。
    // 如果发现 #[repr] 属性，标记为已处理（repred）。
    // 如果枚举没有应用 #[repr(u32)] 属性，则为其添加该属性。
    // 对未包含在现有 #[derive] 列表中的特性，添加 #[derive(trait)]。
    fn visit_item_enum_mut(&mut self, item_enum: &mut syn::ItemEnum) {
        let traits: Vec<syn::Path> = vec![
            parse_quote!(serde_repr::Serialize_repr),
            parse_quote!(serde_repr::Deserialize_repr),
            parse_quote!(TypeGenerator),
            parse_quote!(Debug),
            parse_quote!(Clone),
        ];

        let mut derived: Vec<String> = vec![];
        let mut repred = false;

        for attr in item_enum.attrs.iter_mut() {
            if attr.path().is_ident("derive") {
                let nested = attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .unwrap();
                for item in nested {
                    derived.push(quote::quote!(#item).to_string());
                }
            } else if attr.path().is_ident("repr") {
                repred = true;
            }
        }
        for trait_ in traits {
            let path = quote::quote!(#trait_).to_string();
            if !derived.iter().any(|derived| derived == &path) {
                item_enum.attrs.push(parse_quote!(#[derive(#trait_)]));
            }
        }
        if !repred {
            item_enum.attrs.push(parse_quote!(#[repr(u32)]));
        }
        syn::visit_mut::visit_item_enum_mut(self, item_enum)
    }

    // 在结构体上添加特性（derive traits）和属性
    // 如果发现 #[derive] 属性，提取并存储现有的特性。
    // 对于未包含在现有 #[derive] 列表中的特性，添加 #[derive(trait)]。
    // 如果启用了 wrapper_structs，还会为结构体添加 serde(from = WrapperStruct, into = WrapperStruct) 属性，用于实现结构体的序列化和反序列化包装。
    fn visit_item_struct_mut(&mut self, item_struct: &mut syn::ItemStruct) {
        let traits: Vec<syn::Path> = vec![
            parse_quote!(Serialize),
            parse_quote!(Deserialize),
            parse_quote!(TypeGenerator),
            parse_quote!(Debug),
            parse_quote!(Clone),
        ];
        let mut derived: Vec<String> = vec![];
        let mut repred = false;

        for attr in item_struct.attrs.iter_mut() {
            if attr.path().is_ident("derive") {
                let nested = attr
                    .parse_args_with(
                        syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                    )
                    .unwrap();
                for item in nested {
                    derived.push(quote::quote!(#item).to_string());
                }
            } else if attr.path().is_ident("repr") {
                if self.0.wrapper_structs {
                    let nested = attr
                        .parse_args_with(
                            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                        )
                        .unwrap();
                    assert_eq!(nested.len(), 1);
                    let item = &nested[0];
                    if &quote::quote!(#item).to_string() != "C" {
                        unimplemented!("Struct has repr other than \"C\", which is not supported");
                    }
                }
                repred = true;
            }
        }
        for trait_ in traits {
            let path = quote::quote!(#trait_).to_string();
            if !derived.iter().any(|derived| derived == &path) {
                item_struct.attrs.push(parse_quote!(#[derive(#trait_)]));
            }
        }
        if self.0.wrapper_structs {
            if !repred {
                item_struct.attrs.push(parse_quote!(#[repr(C)]));
            }
            let ident = &item_struct.ident;
            let wrapper_ident = &format!("{ident}Wrapper");
            item_struct
                .attrs
                .push(parse_quote!(#[serde(from = #wrapper_ident, into = #wrapper_ident)]));
        }
        syn::visit_mut::visit_item_struct_mut(self, item_struct)
    }

    // 在结构体上的字段上添加特性（derive traits）和属性
    // 对于可变引用和不可变引用的字段，分别添加 ref_mut_generator 和 ref_generator 属性。
    // 将适当的生成器属性插入到字段的属性列表中。
    fn visit_field_mut(&mut self, field: &mut syn::Field) {
        // if field type contains reference
        let field_ty = &field.ty;
        let field_ty_str = quote::quote!(#field_ty).to_string();
        if field_ty_str == "String" {
            field
                .attrs
                .push(parse_quote!(#[generator(string_generator())]))
        } else if field_ty_str == "Box<str>" {
            field
                .attrs
                .push(parse_quote!(#[generator(boxed_str_generator())]))
        } else if field_ty_str == "f32" {
            field
                .attrs
                .push(parse_quote!(#[generator(f32_generator())]))
        } else if field_ty_str == "f64" {
            field
                .attrs
                .push(parse_quote!(#[generator(f64_generator())]))
        }

        if let syn::Type::Reference(syn::TypeReference { mutability, .. }) = &field_ty {
            if mutability.is_some() {
                field
                    .attrs
                    .push(parse_quote!(#[generator(ref_mut_generator())]))
            } else {
                field
                    .attrs
                    .push(parse_quote!(#[generator(ref_generator())]))
            }
        }

        // if let syn::Type::Array(..) = &field_ty {
        //     field.attrs.push(parse_quote!(#[serde(with = "arrays")]))
        // }

        syn::visit_mut::visit_field_mut(self, field)
    }
}

#[derive(Clone, Debug)]
pub enum LifetimeInfo {
    Explicit(proc_macro2::TokenStream),
    Elided(proc_macro2::TokenStream),
}

// 用于将 LifetimeInfo 类型的实例转换为 proc_macro2::TokenStream。
// 它是用于处理 Rust 语言中与生命周期相关的代码生成或宏扩展的一个部分。
impl quote::ToTokens for LifetimeInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            LifetimeInfo::Explicit(lifetime) => lifetime.to_tokens(tokens),
            LifetimeInfo::Elided(lifetime) => lifetime.to_tokens(tokens),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeKind {
    /// Primitive types like `i32`
    Primitive(&'static str), // 表示基本数据类型
    /// `&mut T`
    RefMut(LifetimeInfo, proc_macro2::TokenStream), // 一个可变引用 &mut T
    /// `&T`
    Ref(LifetimeInfo, proc_macro2::TokenStream), // 不可变引用 &T
    /// `Box<T>`, `T` where `T` is a struct, etc.
    Complex(proc_macro2::TokenStream), // 表示复杂类型，如 Box<T> 或自定义结构体等。
    OutofScope, // 表示超出范围的类型，用于标记不再适用的类型。
}

impl TypeKind {
    // 判断 TypeKind 是否为基本数据类型（Primitive）。
    fn is_primitive(&self) -> bool {
        match self {
            Self::Primitive(..) => true,
            _ => false,
        }
    }
    // 判断 TypeKind 是否为可变引用类型（RefMut）。
    fn is_mutable_ref(&self) -> bool {
        match self {
            Self::RefMut(..) => true,
            _ => false,
        }
    }
}

pub trait CheckOutofScope<'a> {
    fn is_out_of_scope(&'a self) -> bool;
}

impl CheckOutofScope<'_> for TypeKind {
    // 用于判断 TypeKind 枚举的实例是否表示超出范围的类型。
    fn is_out_of_scope(&self) -> bool {
        match self {
            Self::OutofScope => true,
            _ => false,
        }
    }
}

impl<'a, I: 'a> CheckOutofScope<'a> for I
where
    &'a I: IntoIterator<Item = &'a TypeKind>,
{
    // 用于检查 self 中的任意元素是否属于 OutofScope 类型。
    fn is_out_of_scope(&'a self) -> bool {
        self.into_iter().any(|ty| ty.is_out_of_scope())
    }
}

impl CheckOutofScope<'_> for SignatureData {
    // 用于检查 self 的 inputs 和 output 属性是否有 OutofScope 类型的元素。
    fn is_out_of_scope(&self) -> bool {
        self.inputs.is_out_of_scope() || self.output.is_out_of_scope()
    }
}

pub trait CheckisElided<'a> {
    fn is_elided(&'a self) -> bool;
}

impl CheckisElided<'_> for LifetimeInfo {
    // 用于检查 self 是否是 LifetimeInfo::Elided 变体。
    fn is_elided(&self) -> bool {
        matches!(self, LifetimeInfo::Elided(..))
    }
}

impl CheckisElided<'_> for TypeKind {
    // 用于检查 self 是否是 Ref 或 RefMut 变体，并进一步检查它们的生命周期是否是“elided”类型。
    fn is_elided(&self) -> bool {
        match self {
            Self::Ref(lifetime, _) | Self::RefMut(lifetime, _) => lifetime.is_elided(),
            _ => false,
        }
    }
}

impl<'a, I: 'a> CheckisElided<'a> for I
where
    &'a I: IntoIterator<Item = &'a TypeKind>,
{
    // 用于检查 self 中的任何元素是否是“elided”。
    fn is_elided(&'a self) -> bool {
        self.into_iter().any(|ty| ty.is_elided())
    }
}

impl CheckisElided<'_> for SignatureData {
    // 用于检查 self 的 inputs 和 output 是否包含任何“elided”元素。
    fn is_elided(&self) -> bool {
        self.inputs.is_elided() || self.output.is_elided()
    }
}

pub struct FunctionSymbols(HashMap<String, SignatureData>);

pub struct SignatureData {
    output: Option<TypeKind>, //函数的返回类型
    inputs: Vec<TypeKind>, //函数的参数列表
    pub pure: bool, // 函数是否是纯函数
}

impl FunctionSymbols {
    // 从给定的 File 对象中收集 SignatureData 并将其存储在一个 HashMap 中。
    pub fn collect(file: &File) -> HashMap<String, SignatureData> {
        let mut vis = Self(HashMap::new());
        vis.visit_file(file);
        vis.0
    }
}

/// Resolve elided lifetimes at the same time.
/// TODO remove `TypeKind` later
/// 用于将 syn crate 中的 Type 类型转换为自定义的 TypeKind 类型。
fn syn_type_to_type_kind(ty: &syn::Type) -> TypeKind {
    match ty {
        syn::Type::Array(_) => {
            let mut ty = ty.clone();
            ApplyLifetime(quote::quote!('elided)).visit_type_mut(&mut ty);
            TypeKind::Complex(quote::quote!(#ty))
        }
        syn::Type::BareFn(_) => TypeKind::OutofScope,
        syn::Type::Group(_) => TypeKind::OutofScope,
        syn::Type::ImplTrait(_) => TypeKind::OutofScope,
        syn::Type::Infer(_) => TypeKind::OutofScope,
        syn::Type::Macro(_) => TypeKind::OutofScope,
        syn::Type::Never(_) => TypeKind::OutofScope,
        syn::Type::Paren(_) => TypeKind::OutofScope,
        syn::Type::Path(type_path) => {
            let path = &type_path.path;
            let type_string: String = quote::quote!(#path).to_string();

            if type_string.ends_with("c_int") || type_string.ends_with("i32") {
                TypeKind::Primitive("i32")
            } else if type_string.ends_with("i64") {
                TypeKind::Primitive("i64")
            } else if type_string.ends_with("u32") {
                TypeKind::Primitive("u32")
            } else if type_string.ends_with("u64") {
                TypeKind::Primitive("u64")
            } else if type_string.ends_with("usize") {
                TypeKind::Primitive("usize")
            } else if type_string.ends_with("isize") {
                TypeKind::Primitive("isize")
            } else if type_string.ends_with("f32") {
                TypeKind::Primitive("f32")
            } else if type_string.ends_with("f64") {
                TypeKind::Primitive("f64")
            } else if type_string.ends_with("bool") {
                TypeKind::Primitive("bool")
            } else if type_string.ends_with("char") {
                TypeKind::Primitive("char")
            } else if type_string.ends_with("u8") {
                TypeKind::Primitive("u8")
            } else if type_string.ends_with("i8") || type_string.ends_with("c_char") {
                TypeKind::Primitive("i8")
            } else if type_string.ends_with("u16") {
                TypeKind::Primitive("u16")
            } else if type_string.ends_with("i16") {
                TypeKind::Primitive("i16")
            }
            else {
                let mut ty = ty.clone();
                ApplyLifetime(quote::quote!('elided)).visit_type_mut(&mut ty);
                TypeKind::Complex(quote::quote!(#ty))
            }
        }
        syn::Type::Ptr(_) => TypeKind::OutofScope,
        syn::Type::Reference(reference) => {
            let lifetime_info = match &reference.lifetime {
                Some(lifetime) => LifetimeInfo::Explicit(quote::quote!(#lifetime)),
                None => LifetimeInfo::Elided(quote::quote!('elided)),
            };
            let mut inner_ty = reference.elem.as_ref().clone();
            ApplyLifetime(quote::quote!('elided)).visit_type_mut(&mut inner_ty);
            match reference.mutability {
                Some(_) => TypeKind::RefMut(lifetime_info, quote::quote!(#inner_ty)),
                None => TypeKind::Ref(lifetime_info, quote::quote!(#inner_ty)),
            }
        }
        syn::Type::Slice(_) => TypeKind::OutofScope,
        syn::Type::TraitObject(_) => TypeKind::OutofScope,
        syn::Type::Tuple(_) => {
            let mut ty = ty.clone();
            ApplyLifetime(quote::quote!('elided)).visit_type_mut(&mut ty);
            TypeKind::Complex(quote::quote!(#ty))
        }
        syn::Type::Verbatim(_) => TypeKind::OutofScope,
        _ => unreachable!(),
    }
}

impl Visit<'_> for FunctionSymbols {
    // 用于处理 syn::ItemFn 类型的 Rust 函数项，并将函数的签名信息收集到一个 HashMap 中。
    fn visit_item_fn(&mut self, item_fn: &syn::ItemFn) {
        let fn_name = item_fn.sig.ident.to_string();
        if fn_name == "main" {
            return;
        }
        let output = match &item_fn.sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(syn_type_to_type_kind(ty.as_ref())),
        };
        let inputs = item_fn.sig.inputs.iter().map(|arg| match arg {
            syn::FnArg::Typed(pat_type) => syn_type_to_type_kind(pat_type.ty.as_ref()),
            _ => panic!("free standing function item should not contain self type"),
        });
        self.0.insert(
            fn_name,
            SignatureData {
                output,
                inputs: inputs.collect(),
                pure: false,
            },
        );
    }
}

/// Mark which functions must be pure, and therefore is not threaded with
/// a `GlobalState`.
///
/// Currently, functions mentioned in a static item/main function are forced to be pure.

// 元祖结构体,只包含一个匿名的 mut HashMap<String, SignatureData> 类型的变量
pub struct MarkPure<'me>(pub &'me mut HashMap<String, SignatureData>);
struct MarkPureInner<'me>(&'me mut HashMap<String, SignatureData>);
impl Visit<'_> for MarkPureInner<'_> {
    // 用于处理函数调用表达式。在遇到函数调用时，它提取函数的名称，并在 self.0 中找到对应的 SignatureData 记录。
    fn visit_expr_call(&mut self, expr_call: &'_ syn::ExprCall) {
        match &*expr_call.func {
            syn::Expr::Path(ref path) => {
                let path = &path.path;
                if let Some(symbol) = path.get_ident() {
                    let symbol = symbol.to_string();
                    if let Some(data) = self.0.get_mut(&symbol) {
                        data.pure = true;
                    }
                }
            }
            _ => {}
        }
        syn::visit::visit_expr_call(self, expr_call);
    }
}

impl Visit<'_> for MarkPure<'_> {
    // 用于处理 Rust 代码中的静态项（static 变量或常量）。
    fn visit_item_static(&mut self, item_static: &'_ syn::ItemStatic) {
        MarkPureInner(&mut *self.0).visit_expr(&item_static.expr);
    }

    // 用于处理 Rust 代码中的函数项（fn），并且在特定条件下对函数体进行操作。
    fn visit_item_fn(&mut self, item_fn: &'_ syn::ItemFn) {
        if item_fn.sig.ident == "main" {
            MarkPureInner(&mut *self.0).visit_block(&item_fn.block);
        }
    }
}

// 生成一个包含 C 语言外部函数声明的 Rust 代码块，允许 Rust 程序调用 C 语言中的函数。
// 它根据输入参数和返回类型创建函数声明，并根据符号列表匹配最合适的 C 语言函数名称。
fn extern_c_block(
    function_symbols: &HashMap<String, SignatureData>,
    symbol_list: Option<Vec<String>>,
    main_entry: Option<&str>,
) -> proc_macro2::TokenStream {
    let iter = function_symbols
        .iter()
        .filter(|(_, data)| !data.is_out_of_scope())
        .filter(|(name, _)| match main_entry {
            Some(main_entry) => &name[..] == main_entry,
            None => true,
        })
        .map(|(symbol, data)| {
            let link_name = symbol_list
                .as_ref()
                .map(|symbol_list| {
                    let symbol = format!("{symbol}__C");
                    symbol_list.iter().max_by_key(|&given_symbol| {
                        lcs::LcsTable::new(symbol.as_bytes(), given_symbol.as_bytes())
                            .longest_common_subsequence()
                            .len()
                    })
                })
                .flatten()
                .map(|name| quote::quote!(#[link_name = #name]));
            let symbol = quote::format_ident!("{}__C", symbol);
            let args = data.inputs.iter().map(|data| match data {
                &TypeKind::Primitive(type_str) => {
                    let type_id = quote::format_ident!("{type_str}");
                    quote::quote!(#type_id)
                }
                _ => quote::quote!(*mut i8),
            });
            match data.output {
                Some(TypeKind::Primitive(type_str)) => {
                    let type_id = quote::format_ident!("{type_str}");
                    quote::quote!(#link_name fn #symbol(_: *mut i32, #(_: #args),* ) -> #type_id;)
                }
                Some(_) => quote::quote!(#link_name fn #symbol(_: *mut i32,  #(_: #args),* ) -> *mut i8;),
                None => quote::quote!(#link_name fn #symbol(_: *mut i32,  #(_: #args),* );),
            }
        });

    quote::quote!(#[link(name = "ground_truth")]
    extern "C" {
        #( #iter )*
    })
}

// 生成的测试代码用于验证 C 和 Rust 之间的接口是否按预期工作。
// 它创建了一个测试函数，该函数会调用 C 函数和 Rust 函数，然后比较它们的结果。
fn harnesses(
    function_symbols: &HashMap<String, SignatureData>,
    multi_examples: bool,
    timeout: u64,
    comparison_kind: ComparisonKind,
    main_entry: Option<&str>,
) -> proc_macro2::TokenStream {
    let comparison_kind = match comparison_kind {
        ComparisonKind::Bultin => ComparisonKind::String,
        _ => comparison_kind,
    };
    let iter = function_symbols
        .iter()
        .filter(|(_, data)| !data.is_out_of_scope())
        .filter(|(name, _)| {
            match main_entry {
                Some(main_entry) => &name[..] == main_entry,
                None => true,
            }
        })
        .map(|(symbol, data)| {
            // 生成测试用例宏
            let fuzz = quote::format_ident!("fuzz_{symbol}");
            let rust_symbol = quote::format_ident!("{symbol}__Rust");
            let extern_symbol = quote::format_ident!("{symbol}__C");
            // 全局状态管理
            let create_global_state = require_global_state(data).then(|| quote::quote!(let mut global_state = std::panic::AssertUnwindSafe(GlobalState::new());));
            let reset_global_state = require_global_state(data).then(|| quote::quote!(global_state.reset();));
            let params = data.params();  // 获取输入参数
            // 参数类型生成
            let arg_types = data.inputs.iter().map(|ty| match ty {
                &TypeKind::Primitive(ty_str) => {
                    let ident = quote::format_ident!("{ty_str}");
                    quote::quote!(#ident)
                }
                // generate `Box` instead so that `TypeGenerator` generates
                TypeKind::RefMut(_, ty_str) => quote::quote!(Box<#ty_str>),
                TypeKind::Ref(_, ty_str) => quote::quote!(Box<#ty_str>),
                TypeKind::Complex(ty_str) => quote::quote!(#ty_str),
                TypeKind::OutofScope => unreachable!(),
            });
            let input_filter = data.input_filter(); // 筛选和过滤输入参数
            let prepare_extern_args = data.prepare_extern_args(); // 外部参数处理
            // Rust函数的输入参数处理 -> 代码片段
            let rust_args = data.inputs.iter().enumerate().map(|(index, kind)| {
                let ident = quote::format_ident!("input{index}");
                match kind {
                    TypeKind::RefMut(..) => quote::quote!(&mut *#ident),
                    TypeKind::Ref(..) => quote::quote!(&*#ident),
                    _ => quote::quote!(#ident),
                }
            });
            let rust_args = if require_global_state(data) {
                let rust_args = std::iter::once(quote::quote!(&mut global_state))
                    .chain(rust_args);
                quote::quote!(#( #rust_args ),*)
            } else {
                quote::quote!(#( #rust_args ),*)
            };
            // 为返回值生成.unwrap()  -> 代码片段
            let unwrap_result = data.output.as_ref().and_then(|output_ty| {
                if let TypeKind::Complex(tokens) = output_ty {
                    if let Ok(syn::TypePath { path, .. }) = syn::parse2(tokens.clone()) {
                        if let Some(segment) = path.segments.last() {
                            if segment.ident == "Result" {
                                return Some(quote::quote!(.unwrap()))
                            }
                        }
                    }
                }
                None
            });
            let extern_args = data.extern_args(); // 获取外部参数 -> 代码片段
            let compare_input = data.inputs // 比较C语言和Rust的输入参数 -> 代码片段
                .iter()
                .enumerate()
                .filter(|(_, data)| data.is_mutable_ref())
                .map(|(index, _)| {
                    let ident = quote::format_ident!("input{index}");
                    let extern_ident = quote::format_ident!("extern_input{index}");
                    let ident_str = quote::format_ident!("input{index}_str");
                    let extern_ident_str = quote::format_ident!("extern_input{index}_str");
                    let comparison = compare_input_data(
                        index,
                        quote::quote!(#ident_str),
                        quote::quote!(#extern_ident_str),
                        comparison_kind,
                    );
                    // 调用比较函数
                    quote::quote!(
                        let #ident_str = serialize__Rust(&#ident);
                        let #extern_ident_str = std::ffi::CStr::from_ptr(#extern_ident).to_str().unwrap();
                        #comparison
                    )
                });

            // 将输入参数序列化为 repr -> 代码片段
            let prepare_input_reprs = &multi_examples.then(|| {
                let reprs = data.prepare_input_reprs();
                quote::quote!(#( #reprs )*)
            });
            // 记录反例 -> 代码片段
            let record_counter_example = multi_examples.then(|| {
                let record_counter_example = data.record_counter_example();
                quote::quote!(#( #record_counter_example )*)
            });
            // 生成比较输出值（output）的代码片段
            let compare_output = data.output.as_ref().map(|kind| {
                match kind {
                    // 处理基本类型输出
                    TypeKind::Primitive(ty_str) => {
                        let ty_str = ty_str.to_string();
                        let comparison = if ty_str == "f32" || ty_str == "f64" {
                            let eq_func = format_ident!("{ty_str}_eq");
                            quote::quote!(!#eq_func(output, extern_output))
                        } else {
                            quote::quote!(output != extern_output)
                        };
                        quote::quote!(
                            if #comparison {
                                std::panic::panic_any((
                                    ExecutionSuccess(format!("output:{}",output_repr.to_owned())),
                                    ExecutionSuccess(format!("output:{}",extern_output_repr.to_owned())),
                                ))
                            }
                        )
                    }
                    // 处理引用类型和复杂类型输出
                    TypeKind::RefMut(..) | TypeKind::Ref(..) | TypeKind::Complex(_) => {
                        let output = compute_repr(quote::quote!(output_repr), comparison_kind);
                        let extern_output = compute_repr(quote::quote!(extern_output_repr), comparison_kind);

                        let comparison = match comparison_kind {
                            ComparisonKind::Structural => quote::quote!(!structural_eq(&#output, &#extern_output)),
                            _ => quote::quote!(#output != #extern_output)
                        };

                        quote::quote!(
                            if #comparison {
                                std::panic::panic_any((
                                    ExecutionSuccess(format!("output:{}",output_repr.to_owned())),
                                    ExecutionSuccess(format!("output:{}",extern_output_repr.to_owned())),
                                ))
                            }
                        )
                    }
                    // 处理超出范围的类型
                    TypeKind::OutofScope => unreachable!(),
                }
            });
            // extend the lifetime of owners
            // 生成清理外部参数和输出的代码片段
            let extern_args_cleanup = data.extern_args_cleanup();
            let extern_output_cleanup = data.extern_output_cleanup();

            // ------- 模糊测试主逻辑 ---------------
            // 调用外部C函数
            let call_extern_function = quote::quote!(
                {
                    use crash_handler as ch;
                    use crash_handler::jmp;
                    let mut jmp_buf = std::mem::MaybeUninit::uninit();
                    let mut jmp_buf = Mutex::new(jmp_buf);
                    let mut _handler = None;
                    // let val = jmp::sigsetjmp(jmp_buf.as_mut_ptr(), 1); 设置恢复点
                    let val = jmp::sigsetjmp(jmp_buf.lock().unwrap().as_mut_ptr(), 1);
                    if val == 0 {
                        _handler = Some(
                            ch::CrashHandler::attach(ch::make_crash_event(move |cc: &ch::CrashContext| {
                                ch::CrashEventResult::Jump {
                                    // jmp_buf: jmp_buf.as_ptr().cast_mut(),
                                    jmp_buf: jmp_buf.lock().unwrap().as_mut_ptr(),
                                    value: 22,
                                }
                            }))
                            .unwrap()
                        );
                        // 调用外部函数
                        let result = Some(#extern_symbol(&mut success_flag as *mut i32, #( #extern_args ),* ));
                        if success_flag == 0 {
                            None
                        }else{
                            result
                        }
                        
                    } else {
                        assert_eq!(val, 22);
                        None
                    }
                }
            );
            // 结果序列化处理
            let prepare_rust_output_repr = data.prepare_rust_output_repr();
            let prepare_extern_output_repr = data.prepare_extern_output_repr();
            let prepare_output_reprs = data.prepare_output_reprs();
            
            // ----------- 比较输入和输出 ------------
            let compare_executions = quote::quote!(
                match (output, extern_output) {
                    (None, None) =>  std::panic::panic_any((ExecutionFailure, ExecutionFailure)),
                    (None, Some(extern_output)) => {
                        #prepare_extern_output_repr
                        std::panic::panic_any((ExecutionFailure, ExecutionSuccess(format!("output:{}",extern_output_repr.to_owned()))))
                    }
                    (Some(output), None) => {
                        #prepare_rust_output_repr
                        std::panic::panic_any((ExecutionSuccess(format!("output:{}",output_repr.to_owned())), ExecutionFailure))
                    }
                    (Some(output), Some(extern_output)) => {
                        #prepare_output_reprs
                        #compare_output
                        #extern_output_cleanup
                        #( #compare_input )*
                        return ExecutionSuccess(format!("output:{}",output_repr.to_owned()))
                    }
                }
            );
            // 生成用于记录成功执行结果的代码片段。
            let execution_success = multi_examples.then(|| {
                let record_input = data.record_input(quote::quote!(example.args));
                quote::quote!(
                    let mut examples = POSITIVE_EXAMPLES.lock().unwrap();
                    if examples.len() < MAX_NUM_EXAMPLES {
                        let mut example = PositiveExample {
                            args: vec![],
                            actual: execution_result,
                        };
                        #( #record_input )*
                        if !examples.contains(&example) {
                            examples.push(example);
                        }
                    }
                )
            });

            // 处理异常
            let execution_failure = if multi_examples {
                quote::quote!(
                    match err.downcast::<(ExecutionResult, ExecutionResult)>() {
                        Ok(pair) => {
                            let (actual, expected) = *pair;
                            let mut examples = COUNTER_EXAMPLES.lock().unwrap();
                            #record_counter_example
                            if examples.len() >= MAX_NUM_EXAMPLES {
                                let collected = serde_json::to_string(&*examples).unwrap();
                                std::mem::drop(examples);
                                let mut examples = POSITIVE_EXAMPLES.lock().unwrap();
                                let positive = serde_json::to_string(&*examples).unwrap();
                                std::mem::drop(examples);
                                panic!("counter examples: {collected}\npositive examples: {positive}\n");
                            }
                        }
                        Err(err) => std::panic::resume_unwind(err)
                    }
                )
            } else {
                quote::quote!(std::panic::resume_unwind(err))
            };
            let rust_args = &rust_args;

            // 超时处理
            let dump_examples = multi_examples.then(|| {
                quote::quote!(
                    let positive_examples = POSITIVE_EXAMPLES.lock().unwrap();
                    let positive = serde_json::to_string(&*positive_examples).unwrap();
                    std::mem::drop(positive_examples);
                    let counter_examples = COUNTER_EXAMPLES.lock().unwrap();
                    let negative = serde_json::to_string(&*counter_examples).unwrap();
                    std::mem::drop(counter_examples);
                    panic!("Time out!\ncounter examples: {negative}\npositive examples: {positive}\n");
                )
            }).unwrap_or_else(|| quote::quote!(panic!("Time out!\n")));
            // ------------- 整合所有代码片段 ----------------
            quote::quote!(#[test] fn #fuzz() {
                #create_global_state
                #[cfg(feature = "fuzzing")]
                let now = std::time::Instant::now();
                bolero::check!()
                    .with_type()
                    .cloned()
                    .for_each(|( #( #params ),* ): ( #( #arg_types ),* )| unsafe {

                        #[cfg(feature = "fuzzing")]
                        {
                            #( #input_filter )*
                            #prepare_input_reprs
                            #( #prepare_extern_args )*
                            let execution = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                let mut success_flag: i32 = 0; 
                                let extern_output = #call_extern_function;
                                let output = std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                                    #rust_symbol(#rust_args)#unwrap_result
                                )).ok();
                                #compare_executions
                            }));
                            match execution {
                                Ok(execution_result) => {
                                    #execution_success
                                }
                                Err(err) => {
                                    #execution_failure
                                }
                            }
                            #( #extern_args_cleanup )*
                        }

                        #[cfg(not(feature = "fuzzing"))]
                        {
                            std::panic::catch_unwind(std::panic::AssertUnwindSafe(||
                                #rust_symbol(#rust_args)
                            )).ok();
                        }

                        #reset_global_state

                        #[cfg(feature = "fuzzing")]
                        {
                            let elapsed = now.elapsed();
                            if elapsed.as_secs() > #timeout {
                                #dump_examples
                            }
                        }
                    });
            })
        });

    quote::quote!(#( #iter )*)
}

impl SignatureData {
    // 用于创建函数参数的标识符（如 input0、input1 等），并返回这些标识符的 TokenStream。
    fn params(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.inputs.iter().enumerate().map(|(index, _)| {
            let ident = quote::format_ident!("input{index}");
            quote::quote!(mut #ident)
        })
    }

    // 生成一个迭代器，用于筛选和过滤输入参数，并生成相应的 TokenStream。
    // 对函数的输入参数进行过滤，并根据参数的类型生成相应的代码来进行运行时检查。
    fn input_filter(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.inputs.iter().enumerate().filter_map(|(index, data)| {
            let input = quote::format_ident!("input{index}");
            match data {
                // 处理基础类型
                TypeKind::Primitive(ty_str) => {
                    let ty_str = ty_str.to_string();
                    if ty_str == "f32" || ty_str == "f64" {
                        Some(quote::quote!(
                            if !#input.is_finite() {
                                return;
                            }
                        ))
                    } else {
                        None
                    }
                }
                // 处理引用和复杂类型
                TypeKind::RefMut(_, ty_str)
                | TypeKind::Ref(_, ty_str)
                | TypeKind::Complex(ty_str) => {
                    let ty_str = ty_str.to_string();
                    if ty_str.contains("HashSet") || ty_str.contains("HashMap") || ty_str.contains("BTreeMap") || ty_str.contains("BTreeSet") || ty_str.contains("Vec") {
                        return None
                    }
                    if ty_str.contains("str") || ty_str.contains("String") {
                        Some(quote::quote!(
                            if !#input.is_ascii() || #input.chars().any(|c| c.is_ascii_control()) {
                                return;
                            }
                        ))
                    } else {
                        None
                    }
                }
                // 处理不在作用域的类型
                TypeKind::OutofScope => unreachable!(),
            }
        })
    }

    // 用于生成一个迭代器，通过将输入参数序列化为 repr（表示形式），从而生成相应的 TokenStream 代码。
    fn prepare_input_reprs(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.inputs.iter().enumerate().map(|(index, _)| {
            let ident = quote::format_ident!("input{index}_repr");
            let input = quote::format_ident!("input{index}");
            quote::quote!(let #ident = serialize__Rust(&#input);)
        })
    }

    // 生成用于序列化 Rust 输出的代码片段,对输出结果进行序列化
    fn prepare_rust_output_repr(&self) -> proc_macro2::TokenStream {
        match self
            .output
            .as_ref()
            .unwrap_or_else(|| &TypeKind::Primitive("()"))
        {
            TypeKind::Primitive(_) => {
                quote::quote!(
                    let output_repr = serialize__Rust(&output);
                )
            }
            TypeKind::RefMut(..) | TypeKind::Ref(..) | TypeKind::Complex(_) => {
                quote::quote!(
                    let output_repr = serialize__Rust(&output);
                )
            }
            _ => unreachable!(),
        }
    }

    // 用于生成一个 TokenStream，以序列化外部（例如从 C 或其他语言调用的 Rust 代码）返回的输出值，并将其存储在 extern_output_repr 变量中。
    fn prepare_extern_output_repr(&self) -> proc_macro2::TokenStream {
        match self
            .output
            .as_ref()
            .unwrap_or_else(|| &TypeKind::Primitive("()"))
        {
            TypeKind::Primitive(_) => {
                quote::quote!(
                    let extern_output_repr = serialize__Rust(&extern_output);
                )
            }
            TypeKind::RefMut(..) | TypeKind::Ref(..) | TypeKind::Complex(_) => {
                quote::quote!(
                    let extern_output_repr = std::ffi::CStr::from_ptr(extern_output).to_str().unwrap();
                )
            }
            _ => unreachable!(),
        }
    }

    // 结合了两个不同的输出表示形式的生成逻辑，并返回一个组合的 TokenStream。
    fn prepare_output_reprs(&self) -> proc_macro2::TokenStream {
        let rust_repr = self.prepare_rust_output_repr();
        let extern_repr = self.prepare_extern_output_repr();
        quote::quote!(#rust_repr #extern_repr)
    }

    // 用于生成一系列 TokenStream，将输入参数的表示形式（即 _repr 结尾的变量）添加到 args 中。
    fn record_input(
        &self,
        args: proc_macro2::TokenStream,
    ) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.inputs.iter().enumerate().map(move |(index, _)| {
            let ident = quote::format_ident!("input{index}_repr");
            quote::quote!(#args.push(#ident.clone());)
        })
    }

    // 用于生成记录“计数器示例”所需的 TokenStream。
    // 这个函数返回一个 Iterator，其中包含多个 TokenStream 项
    // 用于记录输入参数和实际/期望输出，以便后续用于测试或调试。
    fn record_counter_example(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        std::iter::once(quote::quote!(
            let mut example = CounterExample {
                args: vec![],
                actual,
                expected,
            };
        ))
        .chain(self.record_input(quote::quote!(example.args)))
        .chain(std::iter::once(quote::quote!(
            if !examples.contains(&example) {
                examples.push(example);
            }
        )))
    }

    // 为函数参数准备外部调用的代码。
    // 具体来说，它将每个输入参数转换为适用于外部语言（如 C 语言）的形式，并生成相应的代码片段。
    fn prepare_extern_args(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        fn prepare_extern_arg<'a>(
            (index, data): (usize, &'a TypeKind),
        ) -> proc_macro2::TokenStream {
            if !data.is_primitive() {
                let ident = quote::format_ident!("input{index}"); // 原始输入变量名（如 input0）
                let extern_ident_owner = quote::format_ident!("extern_input{index}_owner"); // 持有序列化数据的变量名（如 extern_input0_owner）
                let extern_ident = quote::format_ident!("extern_input{index}"); // 转换后的外部变量名（如 extern_input0）
                // let reserve_space = data.is_mutable_ref().then(
                //     || quote::quote!(#extern_ident_owner.reserve(2 * #extern_ident_owner.len());),
                // );
                let reserve_space = quote::quote!(#extern_ident_owner.reserve(#extern_ident_owner.len()););
                // 生成处理输入参数代码
                quote::quote!(
                    let mut #extern_ident_owner = serialize__Rust(&#ident).into_bytes();
                    #extern_ident_owner.push(0);
                    #reserve_space
                    let mut #extern_ident = #extern_ident_owner.as_mut_ptr() as *mut i8;
                )
            } 
            // 基本数据类型
            else {
                let ident = quote::format_ident!("input{index}");
                let extern_ident = quote::format_ident!("extern_input{index}");
                quote::quote!(let #extern_ident = #ident;)
            }
        }
        self.inputs.iter().enumerate().map(prepare_extern_arg)
    }

    // 用于生成用于与外部 C 语言函数交互的参数的 TokenStream。
    // 这个函数返回一个迭代器，生成每个输入参数对应的 TokenStream。
    fn extern_args(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.inputs.iter().enumerate().map(|(index, _)| {
            let ident = quote::format_ident!("extern_input{index}");
            quote::quote!(#ident)
        })
    }

    // 生成清理外部输入参数的代码片段。
    // 它处理在 Rust 与外部 C 函数交互时，为了确保正确释放内存或进行其他清理操作的代码。
    fn extern_args_cleanup(&self) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
        self.inputs
            .iter()
            .enumerate()
            .filter(|(_, data)| !data.is_primitive())
            .map(|(index, _)| {
                let extern_ident_owner = quote::format_ident!("extern_input{index}_owner");
                quote::quote!(let _ = #extern_ident_owner;)
            })
    }

    // 生成清理外部输出参数的代码片段。
    // 它处理在 Rust 与外部 C 函数交互时，如何清理 C 端分配的内存。
    fn extern_output_cleanup(&self) -> Option<proc_macro2::TokenStream> {
        self.output
            .as_ref()
            .filter(|data| !data.is_primitive())
            .map(|_| quote::quote!(libc::free(extern_output as *mut libc::c_void);))
    }
}

struct InstrumentCalls<'a> {
    config: &'a InstrConfig,
    function_symbols: &'a HashMap<String, SignatureData>,
}

struct ApplyLifetime(proc_macro2::TokenStream);

impl VisitMut for ApplyLifetime {
    // 用于处理 Rust 语法树的函数，它主要用来处理 syn::TypeReference 类型的节点。
    fn visit_type_reference_mut(&mut self, ty_ref: &mut syn::TypeReference) {
        if ty_ref.lifetime.is_none() {
            let lifetime = &self.0;
            ty_ref.lifetime = Some(parse_quote!(#lifetime));
            syn::visit_mut::visit_type_reference_mut(self, ty_ref);
        }
    }
}

impl VisitMut for InstrumentCalls<'_> {
    // 对函数项进行修改:
    // 1 添加生命周期参数(如果函数签名中有省略的生命周期参数（elided lifetimes），会插入一个显式的 'elided 生命周期。)
    // 2 调整函数参数的可变性(将函数参数的可变性设置为 mut)
    // 3 修改函数名称等(将函数名称后缀添加 __Rust)
    // 4 全局状态(如果函数需要全局状态，会在函数参数中插入 global_state 和相应的生命周期参数)
    // 如果函数名称是 main，则不进行修改，直接调用基类的 visit_item_fn_mut 方法。
    fn visit_item_fn_mut(&mut self, item_fn: &mut syn::ItemFn) {
        let symbol = item_fn.sig.ident.to_string();
        if symbol == "main" {
            // NOTE currently we only allow pure functions to appear in main
            syn::visit_mut::visit_item_fn_mut(self, item_fn);
            return;
        }
        let signature = self
            .function_symbols
            .get(&symbol)
            .expect("user defined function should exist");
        // Resolve elided lifetimes
        if signature.is_elided() {
            item_fn.sig.generics.params.insert(0, parse_quote!('elided));
            for fn_arg in item_fn.sig.inputs.iter_mut() {
                ApplyLifetime(quote::quote!('elided)).visit_fn_arg_mut(fn_arg);
            }
            ApplyLifetime(quote::quote!('elided)).visit_return_type_mut(&mut item_fn.sig.output);
        }
        for fn_arg in item_fn.sig.inputs.iter_mut() {
            if let syn::FnArg::Typed(pat_type) = fn_arg {
                let pat = &mut *pat_type.pat;
                if let syn::Pat::Ident(ident_pat) = pat {
                    ident_pat.mutability = Some(parse_quote!(mut))
                }
            }
        }
        let symbol = quote::format_ident!("{}__Rust", symbol);
        item_fn.sig.ident = symbol;
        if require_global_state(signature) {
            item_fn
                .sig
                .inputs
                .insert(0, parse_quote!(global_state: &'state GlobalState));
            item_fn.sig.generics.params.insert(0, parse_quote!('state))
        }
        for lifetime in item_fn
            .sig
            .generics
            .lifetimes()
            .skip(1)
            .map(|lifetime| quote::quote!(#lifetime))
            .collect::<Vec<_>>()
        {
            item_fn
                .sig
                .generics
                .make_where_clause()
                .predicates
                .push(parse_quote!('state: #lifetime))
        }
        syn::visit_mut::visit_item_fn_mut(self, item_fn);
    }

    // 修改函数调用的路径，并在需要时插入全局状态。
    // 将函数调用的名称后缀添加 __C__wrapper 或 __Rust，具体取决于配置。
    // 如果函数需要全局状态，会在函数调用的参数中插入 global_state。
    fn visit_expr_call_mut(&mut self, expr_call: &mut syn::ExprCall) {
        match &mut *expr_call.func {
            syn::Expr::Path(ref mut path) => {
                let path = &mut path.path;
                if let Some(symbol) = path.get_ident() {
                    let symbol = symbol.to_string();
                    if let Some(data) = self.function_symbols.get(&symbol) {
                        let symbol = if self.config.modular && require_global_state(data) {
                            quote::format_ident!("{}__C__wrapper", symbol)
                        } else {
                            quote::format_ident!("{}__Rust", symbol)
                        };
                        *path = parse_quote!(#symbol);
                        if require_global_state(data) {
                            expr_call.args.insert(0, parse_quote!(&*global_state));
                        }
                    }
                }
            }
            _ => {}
        }
        syn::visit_mut::visit_expr_call_mut(self, expr_call)
    }

    // 处理宏调用，修改宏路径和参数。
    // 如果宏是已知的并且类似于函数调用，会用 mock_macro 包裹原宏，并根据配置插入全局状态。
    fn visit_macro_mut(&mut self, mac: &mut syn::Macro) {
        let path = &mut mac.path;
        let path_name = quote::quote!(#path).to_string();
        // if the macro is well known and function like
        if handled_macros(&path_name) {
            let tokens = &mac.tokens;
            let mut mock: syn::ExprCall = parse_quote!(mock_macro(#tokens));
            self.visit_expr_call_mut(&mut mock);
            let mut args = mock.args;
            if let (true, Some(replacement)) = (
                self.config.capture_stdout,
                capturing_replacement(&path_name),
            ) {
                args.insert(0, parse_quote!(global_state.captured_stdout.borrow_mut()));
                let new_path = format_ident!("{replacement}");
                *path = parse_quote!(#new_path);
            }
            *mac = parse_quote!(#path!(#args))
        }
        syn::visit_mut::visit_macro_mut(self, mac)
    }
}

/// Capturing replacements for macros
/// 用于根据给定的宏路径返回一个替代的宏路径。
/// 这在处理宏时尤其有用，例如将 print 和 println 替换为 write 和 writeln，以便捕获它们的输出。
fn capturing_replacement(path: &str) -> Option<&'static str> {
    match path {
        "print" => Some("write"),
        "println" => Some("writeln"),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonKind {
    Bultin,
    String,
    Structural,
}

// 根据不同的比较类型 (ComparisonKind) 处理给定的输入 x,y，并返回相应的处理结果
// fn compare_data(
//     x: proc_macro2::TokenStream,
//     y: proc_macro2::TokenStream,
//     compare_kind: ComparisonKind,
// ) -> proc_macro2::TokenStream {
//     let (x, y) = match compare_kind {
//         ComparisonKind::Bultin => (x, y),
//         ComparisonKind::String => (quote::quote!(&#x[..]), quote::quote!(&#y[..])),
//         ComparisonKind::Structural => (
//             quote::quote!(serde_json::from_str::<serde_json::Value>(&#x).unwrap()),
//             quote::quote!(serde_json::from_str::<serde_json::Value>(&#y).unwrap()),
//         ),
//     };
//     quote::quote!(assert_eq!(#x, #y);)
// }

fn compare_input_data(
    index: usize,
    x: proc_macro2::TokenStream,
    y: proc_macro2::TokenStream,
    compare_kind: ComparisonKind,
) -> proc_macro2::TokenStream {
    let index_literal = LitInt::new(&index.to_string(), proc_macro2::Span::call_site());
    let (x, y) = match compare_kind {
        ComparisonKind::Bultin => (x, y),
        ComparisonKind::String => (quote::quote!(&#x[..]), quote::quote!(&#y[..])),
        ComparisonKind::Structural => (
            quote::quote!(serde_json::from_str::<serde_json::Value>(&#x).unwrap()),
            quote::quote!(serde_json::from_str::<serde_json::Value>(&#y).unwrap()),
        ),
    };
    //quote::quote!(assert_eq!(#x, #y);)
    quote::quote!(
        if #x != #y {
            std::panic::panic_any((
                ExecutionSuccess(format!("ret_input{}:{}",#index_literal, serde_json::to_string(&#x).unwrap())),
                ExecutionSuccess(format!("ret_input{}:{}",#index_literal, serde_json::to_string(&#y).unwrap())),
            ))
        }
    )
}



fn compute_repr(
    x: proc_macro2::TokenStream,
    compare_kind: ComparisonKind,
) -> proc_macro2::TokenStream {
    match compare_kind {
        ComparisonKind::Bultin => quote::quote!(#x),
        ComparisonKind::String => quote::quote!(&#x[..]),
        ComparisonKind::Structural => {
            quote::quote!(serde_json::from_str::<serde_json::Value>(&#x).unwrap())
        }
    }
}
