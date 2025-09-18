use crate::CARGO;

use base64::Engine;
use heck::ToPascalCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::fs::read_dir;
use syn::parse::{Parse, ParseStream};
use syn::Result;

pub enum Component {
  Filled,
  Color,
  Light,
  Regular,
  Each(String),
}

impl Component {
  fn generate(&self, name: &str, preview: String) -> TokenStream {
    let mod_name = format!("__{name}__");
    let comp_name = Ident::new(&name.to_pascal_case(), Span::call_site());

    let _mod_name = Ident::new(&mod_name, Span::call_site());
    quote! {
      mod #_mod_name {
        extern crate dioxus;
        extern crate dioxus_fluent_sysicon;

        use ::dioxus::prelude::*;

        #[component]
        #[allow(non_snake_case)]
        #[doc = #preview]
        #[doc = ""]
        pub fn #comp_name(
          #[props(extends = GlobalAttributes, extends = svg)] attributes: Vec<Attribute>
        ) -> Element {
          rsx! {
            {::dioxus_fluent_sysicon::icon!{ name: #name, ..attributes }}
          }
        }
      }

      pub use #_mod_name::*;
    }
  }

  fn generates(&self) -> TokenStream {
    let mut components: Vec<TokenStream> = Vec::with_capacity(9);
    let dirs = read_dir(CARGO.join("assets")).expect("Assets dir");
    for dir in dirs.flatten() {
      let path = dir.path();
      if !path.is_file()
        || path.extension().and_then(|e| e.to_str()) != Some("svg")
        || path
          .file_name()
          .is_none_or(|t| !t.to_string_lossy().contains(self.as_ref()))
      {
        continue;
      }
      let stem = path.file_stem().unwrap().to_string_lossy().to_string();

      components.push(self.generate(stem.as_str(), svg_as_preview(path)));
    }

    quote! { #(#components)* }
  }
}

impl AsRef<str> for Component {
  fn as_ref(&self) -> &str {
    match *self {
      Component::Filled => "_filled",
      Component::Color => "_color",
      Component::Light => "_light",
      Component::Regular => "_regular",
      Component::Each(ref str) => str,
    }
  }
}

impl Parse for Component {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(input.parse::<Ident>()?.to_string().into())
  }
}

impl From<String> for Component {
  fn from(value: String) -> Self {
    match value.as_str() {
      "filled" => Self::Filled,
      "color" => Self::Color,
      "light" => Self::Light,
      "regular" => Self::Regular,
      _ => Self::Each(value),
    }
  }
}

pub fn generate_components(comp: Component) -> TokenStream {
  match comp {
    Component::Filled | Component::Color | Component::Light | Component::Regular => comp.generates(),
    Component::Each(ref s) => {
      let file = CARGO.join("assets").join(format!("{s}.svg"));
      comp.generate(s, svg_as_preview(file))
    }
  }
}

fn svg_as_preview(path: impl AsRef<std::path::Path>) -> String {
  let svg = std::fs::read_to_string(path)
    .unwrap()
    .replace("\n", "")
    .replace("currentColor", "#fff");
  let encoded = base64::prelude::BASE64_STANDARD.encode(svg.as_bytes());
  icon_template().replace("DATA", &encoded)
}

fn icon_template() -> String {
  let template = CARGO.join("ICON_TEMPLATE.md");
  std::fs::read_to_string(template).unwrap()
}
