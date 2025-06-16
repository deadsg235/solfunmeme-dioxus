use dioxus::prelude::*;
use crate::model::lean::style::Styles;

#[component]
pub fn ExpressionParsing() -> Element {
    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { class: "{Styles::h2()}", "Expression Parsing" }
            p { class: "{Styles::p()}", "This section demonstrates expression parsing functionality." }
        }
    }
} 