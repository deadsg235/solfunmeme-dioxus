use dioxus::prelude::*;
use std::collections::HashMap;
use crate::model::lean::style::{Styles, THEME};
use crate::model::lean::emojis::{json_to_emoji, type_to_emoji};
use crate::model::simple_expr::SimpleExprType;

#[component]
pub fn StylingAndEmojis() -> Element {
    let emoji_map = use_signal(|| HashMap::<&str, &str>::new());
    let simple_expr_type = use_signal(|| SimpleExprType::bzero());
    let type_emoji = use_signal(|| type_to_emoji(&simple_expr_type.read(), 0, &emoji_map.read()));
    let json_emoji = use_signal(|| json_to_emoji("{ \"type\": \"bvar\" }").unwrap_or_default());

    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { class: "{Styles::section_title()}", "Styling & Emojis" }
            div {
                class: "{Styles::code_block()}",
                style: "{Styles::text_primary()} {Styles::font_weight_bold()}",
                "Primary Gradient: {THEME.primary_gradient}"
                br {}
                "Secondary Gradient: {THEME.secondary_gradient}"
                br {}
                "Spacing XS: {THEME.spacing_xs}"
                br {}
                "Radius SM: {THEME.radius_sm}"
                br {}
                "Shadow MD: {THEME.shadow_md}"
                br {}
                "Type Emoji: {type_emoji.read()}"
                br {}
                "JSON Emoji: {json_emoji.read()}"
            }
            div {
                class: "{Styles::form_grid()}",
                div { class: "{Styles::radio_group()}", "Radio Group Placeholder" }
                div { class: "{Styles::checkbox_label()}", "Checkbox Label Placeholder" }
            }
            div {
                class: "{Styles::flex_center()}",
                style: "{Styles::margin_bottom(THEME.spacing_lg)}",
                "Centered Content"
            }
        }
    }
} 