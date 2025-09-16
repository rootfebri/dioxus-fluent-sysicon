//! lib.rs
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod parse_input;
#[proc_macro]
pub fn icon(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as syn::Expr);
  parse_input::parse_param(input).into()
}
