#![doc = include_str!("../README.md")]

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use crate::generator::Component;

const SELF_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
lazy_static! {
  static ref CARGO: std::path::PathBuf = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
}

mod parse_input;
mod generator;

#[proc_macro]
#[doc = include_str!("../ICON.md")]
pub fn use_icon(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as parse_input::InputTree);
  parse_input::generate_token_stream(input).into()
}

/// # Use this macro in icons.rs (create the module anywhere you like)
///
/// ## There are 5 type of identifier this macro accept as input: filled, color, light, regular and the name of the icon as fallback.
///
/// ### filled, color, light and regular will define icon as dioxus component based on the types
/// ### fallback will search input as icon name and define it as component
///
/// # example
/// ```no_run
///   /// You will get AddFilled component
///   dioxus_fluent_sysicon::define_as_component!(add_filled);
///    /// You will get all colored type icons as component
///    dioxus_fluent_sysicon::define_as_component!(color);
/// ```
#[proc_macro]
pub fn define_as_component(input: TokenStream) -> TokenStream {
  let comp = parse_macro_input!(input as Component);
  generator::generate_components(comp).into()
}
