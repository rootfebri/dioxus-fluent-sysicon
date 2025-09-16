# dioxus-fluent-sysicon

Small procedural macro crate that exposes a single macro: `icon!`.

It generates a reusable Dioxus component (React-like) using `rsx!`, so you can drop Fluent System Icons directly into your UI.

- Requires Dioxus v0.6+ (uses `dioxus-rsx` 0.6 under the hood)
- Ships SVG sources under `assets/`
- Icon data is from Microsoft Fluent System Icons

## Usage

```rust
use dioxus::prelude::*;

#[component]
#[allow(non_snake_case)]
fn App() -> Element {
  rsx! {
    div { class: "flex justify-center items-center", {
      dioxus_fluent_sysicon::icon!{
        name: "building_home_color",  // default size is 16px
        class: "size-12",             // e.g. Tailwind size utility (~96px)
      }
    }}
  }
}
```

Notes
- The `icon!` macro expands into `rsx!` markup; any additional tokens inside the block are forwarded as attributes/props to the generated node.
- Default size is 16px; pass a `class` (or width/height) to style the SVG.
- Icon names map to SVG filenames in `assets/` without the `.svg` extension (see `ICON.md`).

## Icon catalog

See `ICON.md` for the full list of available icon names and quick tips.

## License

The SVG assets come from Microsoft Fluent System Icons and retain their original license: https://github.com/microsoft/fluentui-system-icons

This crate adopts that licensing for those assets. Code in this crate is provided under the crate’s own license; see `LICENSE*` files.
