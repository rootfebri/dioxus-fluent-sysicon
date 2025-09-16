//! parse_input.rs
use proc_macro2::TokenStream;
use quote::__private::ext::RepToTokensExt;
use quote::quote;
use syn::Expr;

pub fn parse_param(input: Expr) -> TokenStream {
  let str = match input.next().unwrap() {
    Expr::Lit(
      syn::ExprLit {
        lit: syn::Lit::Str(str),
        ..
      },
      ..,
    ) => str,
    _ => {
      panic!("TODO: Panic message")
    }
  };

  generate(str.value(), input)
}

fn generate(icon_name: String, expr: Expr) -> TokenStream {
  let manifest = std::env::var("CARGO_MANIFEST_DIR").unwrap();

  let file = format!("{manifest}/assets/{icon_name}.svg");
  let icon = quote! {::core::include_str!(#file)}.to_string();
  let rsx = translate(icon);

  quote! {{
    extern crate dioxus;
    use dioxus::prelude::*;

    #[component]
    #[allow(non_snake_case)]
    /// TODO: `#[doc = define_doc!(#icon_name)]`
    pub fn #icon_name(#[props(extends = GlobalAttributes, extends = svg)] attributes: Vec<Attribute>) {
      rsx! {
        #rsx
      }
    }

    #icon_name {#expr}
  }}
}

fn translate(html: impl AsRef<str>) -> TokenStream {
  let dom = html_parser::Dom::parse(html.as_ref()).unwrap();
  let mut callbody = rsx_rosetta::rsx_from_html(&dom);
  if let Some(dioxus_rsx::BodyNode::Element(element)) = callbody.body.roots.first_mut() {
    let expr: Expr = syn::parse_quote! { attributes };
    let dots: syn::token::DotDot = syn::parse_quote! {..};
    let comma: Option<syn::token::Comma> = Some(syn::parse_quote! {,});
    let global_attribute = dioxus_rsx::Spread {
      dots,
      expr,
      comma,
      dyn_idx: Default::default(),
    };

    element.spreads.push(global_attribute)
  }

  let inden_opts = dioxus_autofmt::IndentOptions::new(dioxus_autofmt::IndentType::Spaces, 2, false);
  let block_out = dioxus_autofmt::write_block_out(&callbody).unwrap_or_default();
  let rsx = dioxus_autofmt::fmt_block(&block_out, 1, inden_opts.clone()).unwrap_or_default();

  quote::quote! {
    #rsx
  }
}
