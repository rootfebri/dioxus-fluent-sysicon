use dioxus_rsx::{BodyNode, CallBody, TemplateBody};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse, Error, Token};

struct Icon {
  key: syn::Ident,
  _colon: Token![:],
  value: syn::LitStr,
  _comma: Option<Token![,]>,
  svg: String,
}

pub(crate) struct InputTree {
  icon: Icon,
  props: TokenStream, // ← sisa token (RSX props), bebas
}

impl Icon {
  fn check_key(self) -> syn::Result<Self> {
    if self.key == "name" {
      Ok(self)
    } else {
      Err(Error::new(self.key.span(), "expected `name: \"...\"`"))
    }
  }

  fn load_svg(mut self) -> syn::Result<Self> {
    self.svg = std::fs::read_to_string(format!(
      "{}/assets/{}.svg",
      crate::SELF_MANIFEST_DIR,
      self.value.value()
    ))
    .map_err(|_| Error::new(Span::call_site(), format!("Cannot find icon `{}`", self.value.value())))?;
    Ok(self)
  }

  fn translate(&self, props: TokenStream) -> TokenStream {
    let dom = html_parser::Dom::parse(&self.svg).unwrap();
    let mut body = rsx_rosetta::rsx_from_html(&dom);

    if let Some(BodyNode::Element(el)) = body.body.roots.first_mut() {
      let hollow: TokenStream = quote::quote! { div { #props } };
      let CallBody {
        body: TemplateBody { roots, .. },
        ..
      } = parse::<CallBody>(hollow.into()).unwrap();

      if let Some(BodyNode::Element(dummy_element)) = roots.into_iter().next() {
        el.raw_attributes.extend(dummy_element.raw_attributes);
        el.merged_attributes.extend(dummy_element.merged_attributes);
        el.spreads.extend(dummy_element.spreads);
      }
    }

    let block_out = dioxus_autofmt::write_block_out(&body).unwrap();
    let rsx_str = dioxus_autofmt::fmt_block(
      &block_out,
      1,
      dioxus_autofmt::IndentOptions::new(dioxus_autofmt::IndentType::Spaces, 2, false),
    )
    .unwrap();

    rsx_str.parse::<TokenStream>().expect("RSX to tokens failed")
  }
}

impl Parse for Icon {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Icon {
      key: input.parse()?,
      _colon: input.parse()?,
      value: input.parse()?,
      _comma: input.parse()?,
      svg: Default::default(),
    }
    .check_key()?
    .load_svg()
  }
}

impl Parse for InputTree {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let icon: Icon = input.parse()?;
    let props: TokenStream = input.parse()?;
    Ok(InputTree { icon, props })
  }
}

pub(crate) fn generate_token_stream(InputTree { icon, props }: InputTree) -> TokenStream {
  let rsx = icon.translate(props);

  quote! {
    rsx! {
      #rsx
    }
  }
}
