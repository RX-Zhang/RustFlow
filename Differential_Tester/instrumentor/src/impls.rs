use crate::InstrConfig;
use syn::{parse_quote, visit_mut::VisitMut, File};

// 生成一个包含多种辅助功能的代码片段，并返回一个 proc_macro2::TokenStream。
// 这些功能包括数值比较、结构相等性检查、随机生成器以及用于数组的序列化和反序列化等。
pub fn code(ast: &File, config: &InstrConfig) -> proc_macro2::TokenStream {
    let wrapper_structs = config
        .wrapper_structs
        .then(|| wrapper_structs(ast))
        .into_iter()
        .flatten();
    quote::quote!( // 生成 Rust 代码片段
        use super::*;
        use serde_json::Value;

        // 比较两个 f32 在一定精度范围内是否相等
        pub fn f32_eq(this: f32, that: f32) -> bool {
            return (this - that).abs() < f32::EPSILON
        }

        #[doc="Checking with reduced precision..."]
        // 比较两个 f64 在一定精度范围内是否相等
        pub fn f64_eq(this: f64, that: f64) -> bool {
            return (this - that).abs() < (f32::EPSILON as f64)
        }

        // 比较两个 serde_json::Value 类型的 JSON 对象的结构是否相同。
        pub fn structural_eq(this: &Value, that: &Value) -> bool {
            match (this, that) {
                (Value::Null, Value::Null) => true,
                (Value::Bool(this), Value::Bool(that)) => this == that,
                (Value::Number(this), Value::Number(that)) => {
                    if let (Some(this), Some(that)) = (this.as_f64(), that.as_f64()) {
                        f64_eq(this, that)
                    } else {
                        false
                    }
                }
                (Value::String(this), Value::String(that)) => this == that,
                (Value::Array(this), Value::Array(that)) => {
                    if this.len() != that.len() {
                        return false;
                    }
                    this.iter()
                        .zip(that)
                        .all(|(this, that)| structural_eq(this, that))
                }
                (Value::Object(this), Value::Object(that)) => {
                    this.iter()
                        .zip(that)
                        .all(|((this_key, this_value), (that_key, that_value))| {
                            this_key == that_key && structural_eq(this_value, that_value)
                        })
                }
                _ => false
            }
        }

        // 用于生成不同类型随机值的 Rust 模块
        mod generator {
            use super::*;
            // 生成对随机生成的值的不可变引用
            pub fn ref_generator<'a, T: ?Sized + 'a>() -> impl ValueGenerator<Output = &'a T>
            where
                Box<T>: TypeGenerator,
            {
                bolero::gen::<Box<T>>().map_gen(|data| &*Box::leak(data))
            }
            // 生成对随机生成的值的可变引用。
            pub fn ref_mut_generator<'a, T: ?Sized + 'a>() -> impl ValueGenerator<Output = &'a mut T>
            where
                Box<T>: TypeGenerator,
            {
                bolero::gen::<Box<T>>().map_gen(|data| Box::leak(data))
            }
            // 生成一个仅包含 ASCII 字符且不包含控制字符的随机 String。
            pub fn string_generator() -> impl ValueGenerator<Output = String> {
                bolero::gen::<String>().filter_gen(|data| {
                    data.is_ascii() && !data.chars().any(|c| c.is_ascii_control())
                })
            }
            //生成一个仅包含 ASCII 字符且不包含控制字符的 Box<str> 类型的随机字符串。
            pub fn boxed_str_generator() -> impl ValueGenerator<Output = Box<str>> {
                bolero::gen::<Box<str>>().filter_gen(|data| {
                    data.is_ascii() && !data.chars().any(|c| c.is_ascii_control())
                })
            }
            // 生成一个有限的随机 f32 值（即排除 NaN、正无穷或负无穷）。
            pub fn f32_generator() -> impl ValueGenerator<Output = f32> {
                bolero::gen::<f32>().filter_gen(|data| {
                    data.is_finite()
                })
            }
            // 生成一个有限的随机 f64 值。
            pub fn f64_generator() -> impl ValueGenerator<Output = f64> {
                bolero::gen::<f64>().filter_gen(|data| {
                    data.is_finite()
                })
            }
        }
        pub use generator::*;

        use serde::{Serialize, Deserialize};

        // 用于序列化和反序列化固定大小数组。
        pub mod arrays {
            use std::{convert::TryInto, marker::PhantomData};

            use serde::{
                de::{SeqAccess, Visitor},
                ser::SerializeTuple,
                Deserialize, Deserializer, Serialize, Serializer,
            };
            // 将一个固定大小的数组 [T; N] 序列化为一个元组。
            pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
                data: &[T; N],
                ser: S,
            ) -> Result<S::Ok, S::Error> {
                let mut s = ser.serialize_tuple(N)?;
                for item in data {
                    s.serialize_element(item)?;
                }
                s.end()
            }

            // 用于实现 Visitor 接口，帮助反序列化固定大小的数组。
            struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

            impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
            where
                T: Deserialize<'de>,
            {
                type Value = [T; N];
                // 定义反序列化时的预期描述，这里是一个长度为 N 的数组
                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str(&format!("an array of length {}", N))
                }

                // 从序列中获取元素，并将其存储在一个 Vec 中。如果序列中元素数量不足 N，则返回错误。
                // 使用 Vec::try_into() 尝试将 Vec 转换为固定大小的数组。如果转换成功，则返回该数组，否则返回 unreachable!() 错误。
                #[inline]
                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    // can be optimized using MaybeUninit
                    let mut data = Vec::with_capacity(N);
                    for _ in 0..N {
                        match (seq.next_element())? {
                            Some(val) => data.push(val),
                            None => return Err(serde::de::Error::invalid_length(N, &self)),
                        }
                    }
                    match data.try_into() {
                        Ok(arr) => Ok(arr),
                        Err(_) => unreachable!(),
                    }
                }
            }

            // 序列化的数据 反序列化为一个固定大小的数组 [T; N]。
            pub fn deserialize<'de, D, T, const N: usize>(
                deserializer: D,
            ) -> Result<[T; N], D::Error>
            where
                D: Deserializer<'de>,
                T: Deserialize<'de>,
            {
                deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
            }
        }

        // cannot derive Default as std only derive it for arrays of length
        // le 32...
        #[repr(transparent)]
        #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
        // 包装数组，并且带有 Serialize 和 Deserialize 实现，可以用于序列化和反序列化固定大小的数组
        enum ArrayWrapper<T, const N: usize> {
            #[serde(with = "arrays")]
            #[serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))]
            #[serde(untagged)]
            Arr([T; N])
        }

        #( #wrapper_structs )* // 用于插入 wrapper_structs 的位置。
    )
}

// 对 AST（抽象语法树）中的结构体（struct）进行处理，
// 将数组类型包装成 ArrayWrapper 类型，并生成从原始结构体到包装结构体的转换实现
fn wrapper_structs(ast: &File) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    // 遍历 AST 中的所有项，并过滤出所有的--结构体项。
    ast.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(item_struct) => Some(item_struct),
            _ => None,
        })
        .map(|item_struct| {
            let mut item_struct = item_struct.clone(); // 对每个结构体项进行克隆，以便可以在不修改原始 AST 的情况下操作。
            struct ApplyWrapper;
            impl VisitMut for ApplyWrapper { 
                // 修改结构体中的数组类型，将其包装成 ArrayWrapper 类型。
                fn visit_type_mut(&mut self, ty: &mut syn::Type) {
                    if let syn::Type::Array(type_array) = ty {
                        let elem_type = &type_array.elem;
                        let len = &type_array.len;
                        let len = if let syn::Expr::Lit(..) = len {
                            quote::quote!(#len)
                        } else {
                            quote::quote!({#len})
                        };
                        *ty = parse_quote!(
                            ArrayWrapper<#elem_type, #len>
                        )
                    }
                    syn::visit_mut::visit_type_mut(self, ty);
                }
            }
            ApplyWrapper.visit_item_struct_mut(&mut item_struct);
            // 获取原始结构体的标识符（名字），并为包装结构体生成一个新的标识符，通常是原始名字后面加上 Wrapper
            let ident = &item_struct.ident.clone();
            let wrapper_ident = &quote::format_ident!("{ident}Wrapper");
            item_struct.ident = wrapper_ident.clone();
            // 检查原始结构体是否已经有 repr 属性。如果没有，则为其添加 #[repr(C)] 属性
            let mut repred = false;
            for attr in item_struct.attrs.iter() {
                if attr.path().is_ident("repr") {
                    repred = true;
                }
            }
            if !repred {
                item_struct.attrs.push(parse_quote!(#[repr(C)]));
            }
            // 为包装结构体添加 #[derive(Serialize, Deserialize)] 属性，以使其可以进行序列化和反序列化。
            item_struct
                .attrs
                .push(parse_quote!(#[derive(Serialize, Deserialize)]));
            item_struct.vis = parse_quote!(pub); // 将包装结构体的可见性设置为 pub，使其可以在模块外部使用。
            // 最终生成包装结构体的代码片段，包括包装结构体的定义，
            // 以及从原始结构体到包装结构体和从包装结构体到原始结构体的转换实现
            quote::quote!(
                #item_struct

                impl std::convert::From<#ident> for #wrapper_ident {
                    fn from(value: #ident) -> Self {
                        unsafe {
                            std::mem::transmute::<#ident, #wrapper_ident>(value)
                        }
                    }
                }

                impl std::convert::From<#wrapper_ident> for #ident {
                    fn from(value: #wrapper_ident) -> Self {
                        unsafe {
                            std::mem::transmute::<#wrapper_ident, #ident>(value)
                        }
                    }
                }
            )
        })
}
