# Fluent System Icons — Catalog & Usage

This crate exposes a single macro, `icon!`, that renders Fluent System Icons from the `assets/` folder.

- Icon keys are the SVG filenames without the `.svg` extension.
- Variants include suffixes like `_filled`, `_regular`, `_light`, and `_color`.
- Works with Dioxus v0.6+ and expands to `rsx!` behind the scenes.

use [define_as_component](`dioxus_fluent_sysicon::define_as_component`) if you want autocomplete to define group/each icon as comnponent