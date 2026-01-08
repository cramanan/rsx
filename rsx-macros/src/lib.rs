// mod codegen;
mod ir;
mod parse;
mod tokens;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::ir::Root;

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let node = parse_macro_input!(input as Root);
    quote! {{ rsx::Element::from(#node) }}.into()
}
