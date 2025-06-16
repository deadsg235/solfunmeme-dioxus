use dioxus::prelude::*;
use crate::model::lean::style::Styles;

#[component]
pub fn StylingAndEmojis() -> Element {
    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { class: "{Styles::h2()}", "Styling & Emojis" }
            p { class: "{Styles::p()}", "This section demonstrates styling and emoji support." }
        }
    }
} 