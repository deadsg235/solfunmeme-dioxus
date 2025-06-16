use dioxus::prelude::*;
use crate::model::lean::style::Styles;
use crate::playground::MenuOption;

#[component]
pub fn TransactionButtons(on_menu_change: EventHandler<MenuOption>) -> Element {
    rsx! {
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::SendSol),
            "Send SOL"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::ReceiveSol),
            "Receive SOL"
        }
        button {
            class: "{Styles::primary_button()}",
            onclick: move |_| on_menu_change.call(MenuOption::QueryAccounts),
            "Query Accounts"
        }
    }
} 