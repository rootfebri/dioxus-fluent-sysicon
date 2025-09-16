//! lib.rs
use proc_macro::TokenStream;
use syn::parse_macro_input;

const SELF_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

mod parse_input;
#[proc_macro]
pub fn icon(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as parse_input::InputTree);
  parse_input::generate_token_stream(input).into()
}
