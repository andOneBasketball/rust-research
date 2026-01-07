use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 把 struct / enum 解析成 AST
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // 生成 Rust 代码
    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello from {}!", stringify!(#name));
            }
        }
    };

    expanded.into()
}
