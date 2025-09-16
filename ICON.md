# Fluent System Icons — Catalog & Usage

This crate exposes a single macro, `icon!`, that renders Fluent System Icons from the `assets/` folder.

- Icon keys are the SVG filenames without the `.svg` extension.
- Variants include suffixes like `_filled`, `_regular`, `_light`, and `_color`.
- Works with Dioxus v0.6+ and expands to `rsx!` behind the scenes.

## Quick usage

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


Tips
- Default viewport is square; set width/height via `class` or `style`.
- Names are case-sensitive and must exactly match the filename minus `.svg`.

## Find an icon fast

- Browse `assets/` in your editor and copy the filename without `.svg`.
- Or search in this file (Ctrl/Cmd + F) for a keyword (e.g., “calendar”, “arrow”, “person”).

## Regenerate this list (optional)

If you add or remove SVGs, you can regenerate a full list of icon keys with a simple script. Example for Bash or Git Bash on Windows:

```bash
# From the repository root
find assets -maxdepth 1 -type f -name "*.svg" -printf "%f\n" \
  | sort \
  | sed 's/\.svg$//' > ICON_KEYS.txt

# Preview the first 50 keys
head -n 50 ICON_KEYS.txt
```

You can then paste the contents of `ICON_KEYS.txt` into a section below if you prefer an inline, exhaustive list.

> Note: The full catalog contains thousands of icons. To keep this documentation readable in editors and diffs, we include a representative sample below and rely on the `assets/` folder as the source of truth.

---

## Sample icon keys (alphabetical)

<details>
<summary>Click to expand sample (~200 entries)</summary>

```
accessibility_checkmark_filled
accessibility_checkmark_light
accessibility_checkmark_regular
accessibility_error_filled
accessibility_error_regular
accessibility_filled
accessibility_more_filled
accessibility_more_regular
accessibility_question_mark_filled
accessibility_question_mark_regular
accessibility_regular
access_time_filled
access_time_regular
add_circle_color
add_circle_filled
add_circle_regular
add_filled
add_light
add_regular
add_square_filled
add_square_multiple_filled
add_square_multiple_regular
add_square_regular
add_starburst_color
add_starburst_filled
add_starburst_regular
add_subtract_circle_filled
add_subtract_circle_regular
agents_add_filled
agents_add_regular
agents_color
agents_filled
agents_regular
airplane_filled
airplane_landing_filled
airplane_landing_regular
airplane_regular
airplane_take_off_filled
airplane_take_off_regular
album_add_filled
album_add_regular
album_filled
album_regular
alert_badge_color
alert_badge_filled
alert_badge_regular
alert_color
alert_filled
alert_light
alert_off_filled
alert_off_regular
alert_on_filled
alert_on_regular
alert_regular
alert_snooze_filled
alert_snooze_regular
alert_urgent_color
alert_urgent_filled
alert_urgent_regular
align_bottom_filled
align_bottom_regular
align_center_horizontal_filled
align_center_horizontal_regular
align_center_vertical_filled
align_center_vertical_regular
align_distribute_bottom_filled
align_distribute_bottom_regular
align_distribute_left_filled
align_distribute_left_regular
align_distribute_right_filled
align_distribute_right_regular
align_distribute_top_filled
align_distribute_top_regular
align_end_horizontal_filled
align_end_horizontal_regular
align_end_vertical_filled
align_end_vertical_regular
align_left_filled
align_left_regular
align_right_filled
align_right_regular
align_space_around_horizontal_filled
align_space_around_horizontal_regular
align_space_around_vertical_filled
align_space_around_vertical_regular
align_space_between_horizontal_filled
align_space_between_horizontal_regular
align_space_between_vertical_filled
align_space_between_vertical_regular
align_space_evenly_horizontal_filled
align_space_evenly_horizontal_regular
align_space_evenly_vertical_filled
align_space_evenly_vertical_regular
align_space_fit_vertical_filled
align_space_fit_vertical_regular
align_start_horizontal_filled
align_start_horizontal_regular
align_start_vertical_filled
align_start_vertical_regular
align_straighten_filled
align_straighten_regular
align_stretch_horizontal_filled
align_stretch_horizontal_regular
align_stretch_vertical_filled
align_stretch_vertical_regular
align_top_filled
align_top_regular
animal_cat_filled
animal_cat_regular
animal_dog_filled
animal_dog_regular
animal_paw_print_color
animal_paw_print_filled
animal_paw_print_regular
animal_rabbit_filled
animal_rabbit_off_filled
animal_rabbit_off_regular
animal_rabbit_regular
animal_turtle_filled
animal_turtle_regular
approvals_app_color
approvals_app_filled
approvals_app_regular
apps_add_in_filled
apps_add_in_off_filled
apps_add_in_off_regular
apps_add_in_regular
apps_color
apps_filled
apps_list_color
apps_list_detail_color
apps_list_detail_filled
apps_list_detail_regular
apps_list_filled
apps_list_regular
apps_regular
apps_settings_filled
apps_settings_regular
apps_shield_filled
apps_shield_regular
app_folder_filled
app_folder_light
app_folder_regular
app_generic_filled
app_generic_light
app_generic_regular
app_recent_filled
app_recent_regular
app_store_filled
app_store_regular
app_title_filled
app_title_regular
archive_arrow_back_filled
archive_arrow_back_regular
archive_filled
archive_light
archive_multiple_filled
archive_multiple_regular
archive_regular
archive_settings_filled
archive_settings_light
archive_settings_regular
arrows_bidirectional_filled
arrows_bidirectional_regular
arrow_autofit_content_filled
arrow_autofit_content_regular
arrow_autofit_down_filled
arrow_autofit_down_regular
arrow_autofit_height_dotted_filled
arrow_autofit_height_dotted_regular
arrow_autofit_height_filled
arrow_autofit_height_in_filled
arrow_autofit_height_in_regular
arrow_autofit_height_regular
arrow_autofit_up_filled
arrow_autofit_up_regular
arrow_autofit_width_dotted_filled
arrow_autofit_width_dotted_regular
arrow_autofit_width_filled
arrow_autofit_width_regular
arrow_between_down_filled
arrow_between_down_regular
arrow_between_up_filled
arrow_between_up_regular
arrow_bidirectional_left_right_filled
arrow_bidirectional_left_right_regular
arrow_bidirectional_up_down_filled
arrow_bidirectional_up_down_regular
arrow_bounce_filled
arrow_bounce_regular
arrow_circle_down_double_filled
arrow_circle_down_double_regular
arrow_circle_down_filled
arrow_circle_down_regular
arrow_circle_down_right_filled
arrow_circle_down_right_regular
arrow_circle_down_split_filled
arrow_circle_down_split_regular
arrow_circle_down_up_filled
arrow_circle_down_up_regular
arrow_circle_left_filled
arrow_circle_left_regular
arrow_circle_right_filled
arrow_circle_right_regular
arrow_circle_up_filled
arrow_circle_up_left_filled
arrow_circle_up_left_regular
arrow_circle_up_regular
arrow_circle_up_right_filled
arrow_circle_up_right_regular
arrow_circle_up_sparkle_filled
arrow_circle_up_sparkle_regular
arrow_clockwise_dashes_color
arrow_clockwise_dashes_filled
arrow_clockwise_dashes_regular
arrow_clockwise_dashes_settings_color
arrow_clockwise_dashes_settings_filled
arrow_clockwise_dashes_settings_regular
arrow_clockwise_filled
arrow_clockwise_light
arrow_clockwise_regular
arrow_collapse_all_filled
arrow_collapse_all_regular
arrow_counterclockwise_dashes_filled
arrow_counterclockwise_dashes_regular
arrow_counterclockwise_filled
arrow_counterclockwise_info_filled
arrow_counterclockwise_info_regular
arrow_counterclockwise_regular
arrow_curve_down_left_filled
arrow_curve_down_left_regular
arrow_curve_down_right_filled
arrow_curve_down_right_regular
arrow_curve_up_left_filled
arrow_curve_up_left_regular
arrow_curve_up_right_filled
arrow_curve_up_right_regular
arrow_download_filled
arrow_download_light
arrow_download_off_filled
arrow_download_off_regular
arrow_download_regular
arrow_down_exclamation_filled
arrow_down_exclamation_regular
arrow_down_filled
arrow_down_left_filled
arrow_down_left_regular
arrow_down_light
arrow_down_regular
arrow_down_right_filled
arrow_down_right_regular
arrow_eject_filled
arrow_eject_regular
arrow_enter_filled
arrow_enter_left_filled
arrow_enter_left_regular
arrow_enter_regular
arrow_enter_up_filled
arrow_enter_up_regular
arrow_exit_filled
arrow_exit_regular
arrow_expand_all_filled
arrow_expand_all_regular
arrow_expand_filled
arrow_expand_regular
arrow_export_filled
arrow_export_ltr_filled
arrow_export_ltr_regular
arrow_export_regular
arrow_export_rtl_filled
arrow_export_rtl_regular
arrow_export_up_filled
arrow_export_up_regular
arrow_fit_filled
arrow_fit_in_filled
arrow_fit_in_regular
arrow_fit_regular
```

</details>

---

## License

The icon assets are sourced from Microsoft’s Fluent System Icons and follow their licensing. See the upstream repository for details:
https://github.com/microsoft/fluentui-system-icons

Code in this crate is licensed separately per the repository’s LICENSE files. The assets remain under their original license.

