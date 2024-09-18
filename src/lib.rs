mod auto_debug;
mod auto_deref;
mod enum_macro;
mod enum_macro_daring;

use auto_debug::process_auto_debug;
use auto_deref::process_auto_deref;
use enum_macro::process_enum_from;
use enum_macro_daring::process_enum_from_darling;
// proc-macro
use proc_macro::TokenStream;
// 注册EnumFrom过程宏的实现，自动为 变体 生成From<T> trait实现
#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    // input是proc-macro的输入（代表要处理的代码），返回新生成的代码
    // 使用syn解析input为DeriveInput
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    process_enum_from(input).into()
}

// 注册EnumFromDarling过程宏的实现
#[proc_macro_derive(EnumFromDarling)]
pub fn derive_enum_from_darling(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    process_enum_from_darling(input).into()
}

#[proc_macro_derive(AutoDeref, attributes(deref))]
pub fn derive_auto_deref(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    process_auto_deref(input).into()
}

#[proc_macro_derive(AutoDebug, attributes(debug))]
pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    process_auto_debug(input).into()
}
