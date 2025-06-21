use dioxus::prelude::*;
use crate::model::lean::style::Styles;
use crate::playground::MenuOption;

#[component]
pub fn CoreButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
	button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Embedding),
            "Embedding Operations"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::PerformanceCharts),
            "Performance Charts"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MemeManagement),
            "Meme Management"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ExpressionParsing),
            "Expression Parsing"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Encryption),
            "Encryption"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::MetaMemeOperations),
            "MetaMeme Operations"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::StylingAndEmojis),
            "Styling & Emojis"
        }
    }
} 
