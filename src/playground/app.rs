use dioxus::prelude::*;
use crate::model::lean::style::{Styles, THEME};
use crate::app::{FAVICON, TAILWIND_CSS};
use crate::model::NotificationInfo;
//use crate::model::notificationinfo::NotificationInfo;
use crate::password_manager::DecryptedEntry;
use crate::playground::{Encryption, ExpressionParsing, MemeManagement, MetaMemeOperations, StylingAndEmojis};

#[derive(PartialEq, Clone)]
pub enum MenuOption {
    MemeManagement,
    ExpressionParsing,
    Encryption,
    MetaMemeOperations,
    StylingAndEmojis,
}

#[component]
pub fn App() -> Element {
    let mut menu_option = use_signal(|| MenuOption::MemeManagement);
    let notifications = use_signal(|| vec![NotificationInfo {
        key: 1,
        secs: 5,
        message: "Welcome to Meme App!".to_string(),
    }]);

    rsx! {
        link { rel: "stylesheet", href: "{TAILWIND_CSS}" }
        link { rel: "icon", href: "{FAVICON}" }
        div {
            style: "background: {THEME.background_color}; padding: {THEME.spacing_md}; font-family: {THEME.font_family_sans}",
            nav {
                class: "{Styles::header()}",
                ul {
                    class: "{Styles::flex_between()}",
                    li {
                        button {
                            class: "{Styles::primary_button()}",
                            onclick: move |_| menu_option.set(MenuOption::MemeManagement),
                            "Meme Management"
                        }
                    }
                    li {
                        button {
                            class: "{Styles::primary_button()}",
                            onclick: move |_| menu_option.set(MenuOption::ExpressionParsing),
                            "Expression Parsing"
                        }
                    }
                    li {
                        button {
                            class: "{Styles::primary_button()}",
                            onclick: move |_| menu_option.set(MenuOption::Encryption),
                            "Encryption"
                        }
                    }
                    li {
                        button {
                            class: "{Styles::primary_button()}",
                            onclick: move |_| menu_option.set(MenuOption::MetaMemeOperations),
                            "MetaMeme Operations"
                        }
                    }
                    li {
                        button {
                            class: "{Styles::primary_button()}",
                            onclick: move |_| menu_option.set(MenuOption::StylingAndEmojis),
                            "Styling & Emojis"
                        }
                    }
                }
            }
            div {
                class: "{Styles::section()}",
                {
                    notifications.read().iter().map(|notif| rsx! {
                        div {
                            key: "{notif.key}",
                            style: "color: {THEME.text_primary}; margin-bottom: {THEME.spacing_sm}",
                            "{notif.message} (Visible for {notif.secs} seconds)"
                        }
                    })
                }
            }
            div {
                class: "{Styles::app_container()}",
                match *menu_option.read() {
                    MenuOption::MemeManagement => rsx! { MemeManagement {} },
                    MenuOption::ExpressionParsing => rsx! { ExpressionParsing {} },
                    MenuOption::Encryption => rsx! { Encryption {} },
                    MenuOption::MetaMemeOperations => rsx! { MetaMemeOperations {} },
                    MenuOption::StylingAndEmojis => rsx! { StylingAndEmojis {} },
                }
            }
        }
    }
} 