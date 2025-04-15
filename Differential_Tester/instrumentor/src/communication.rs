use crate::InstrConfig;

impl InstrConfig {
    // 生成一个包含序列化和反序列化功能的 Rust 模块，并将这个模块以 proc_macro2::TokenStream 的形式返回，以便在代码生成过程中使用。
    // 这种生成的代码可以嵌入到其它代码中，实现动态的代码注入。
    pub fn communication(&self) -> proc_macro2::TokenStream {
        quote::quote!(
            mod communication {
                // serialize/deserialize a Rust data
                use serde::{Deserialize, Serialize};

                trait SpecSerialize {
                    fn serialize(&self) -> String; // 将对象序列化为 String。
                }

                impl<T> SpecSerialize for T
                where
                    T: Serialize,
                {
                    default fn serialize(&self) -> String {
                        serde_json::to_string(self).unwrap()
                    }
                }

                trait SpecDeserialize<'a> {
                    fn deserialize(_: &'a str) -> Self; // 通过 serde_json 库从 JSON 字符串反序列化为对象。
                }

                impl<'a, T> SpecDeserialize<'a> for T
                where
                    T: Deserialize<'a>,
                {
                    default fn deserialize(s: &'a str) -> Self {
                        serde_json::from_str(s).unwrap()
                    }
                }

                // 将 Rust 对象序列化为字符串
                pub fn serialize__Rust<T: Serialize>(data: &T) -> String { 
                    <T as SpecSerialize>::serialize(data)
                }

                // 将字符串反序列化为 Rust 对象。
                pub fn deserialize__Rust<'state: 'a, 'a, T: Deserialize<'a>>(
                    global_state: &'state crate::GlobalState,
                    s: *mut i8,
                ) -> T {
                    unsafe {
                        let st = std::ffi::CStr::from_ptr(s).to_str().unwrap();
                        let st = global_state.arena.alloc_str(st);
                        <T as SpecDeserialize>::deserialize(st)
                    }
                }
            }
            use communication::*;
        )
    }
}
