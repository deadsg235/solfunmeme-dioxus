use dioxus::prelude::*;
use crate::model::lean::style::Styles;
use crate::playground::MenuOption;

#[component]
pub fn ManagementButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Dashboard),
            "Dashboard"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ClustersManagement),
            "Clusters Management"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Clusters),
            "Clusters"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Airdrop),
            "Airdrop"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::Accounts),
            "Accounts"
        }
    }
} 