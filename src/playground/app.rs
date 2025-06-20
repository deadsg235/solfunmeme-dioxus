use dioxus::prelude::*;
use crate::model::lean::style::{Styles, THEME};
use crate::app::{FAVICON, TAILWIND_CSS};
use crate::model::NotificationInfo;
//use crate::password_manager::DecryptedEntry;
use crate::views::{
    send_sol::SendSol,
    receive_sol::ReceiveSol,
    dashboard::Dashboard,
    connections::Connections,
    clusters::Clusters,
    airdrop::Airdrop,
    accounts::Accounts,
    core_buttons::CoreButtons,
    crypto_buttons::CryptoButtons,
    connection_buttons::ConnectionButtons,
    transaction_buttons::TransactionButtons,
    management_buttons::ManagementButtons,
    styling_and_emojis::StylingAndEmojis,
    meta_meme_operations::MetaMemeOperations,
    meme_management::MemeManagement,
    expression_parsing::ExpressionParsing,
    encryption::Encryption,
    component_memes::ComponentMemeExplorer,
};
//pub mod embedding;
use crate::playground::embedding::EmbeddingApp;

#[derive(PartialEq, Clone)]
pub enum MenuOption {
    Embedding,
    #[allow(dead_code)]
    MemeManagement,
    #[allow(dead_code)]
    ExpressionParsing,
    #[allow(dead_code)]
    Encryption,
    #[allow(dead_code)]
    MetaMemeOperations,
    #[allow(dead_code)]
    StylingAndEmojis,
    #[allow(dead_code)]
    CryptoFrontend,
    #[allow(dead_code)]
    Memes,
    #[allow(dead_code)]
    Lean,
    #[allow(dead_code)]
    ConnectionManagement,
    #[allow(dead_code)]
    ConnectionList,
    #[allow(dead_code)]
    SendSol,
    #[allow(dead_code)]
    ReceiveSol,
    #[allow(dead_code)]
    QueryAccounts,
    #[allow(dead_code)]
    Dashboard,
    #[allow(dead_code)]
    Connections,
    #[allow(dead_code)]
    ClustersManagement,
    #[allow(dead_code)]
    Clusters,
    #[allow(dead_code)]
    Airdrop,
    #[allow(dead_code)]
    Accounts,
    #[allow(dead_code)]
    ComponentMemes,
}

#[component]
pub fn PlaygroundApp() -> Element {
    //    let mut menu_option = use_signal(|| MenuOption::MemeManagement);
        let mut menu_option = use_signal(|| MenuOption::Embedding);
    let notifications = use_signal(|| vec![NotificationInfo {
        key: 1,
        secs: 5,
        message: "Welcome to SOLFUNMEME App!".to_string(),
    }]);

    rsx! {
        link { rel: "stylesheet", href: TAILWIND_CSS }
        link { rel: "icon", href: FAVICON }
        div {
            style: format!(
                "background: {}; padding: {}; font-family: {}",
                THEME.background_color, THEME.spacing_md, THEME.font_family_sans
            ),
            nav {
                // The change here is to fix the code block so that the `div` element is properly closed.
                // Previously, the `div` was not closed before the next sibling element, which would cause a syntax or rendering error.
                // Now, the `div` wraps only the intended button components and is closed before the next elements in the parent `nav`.
                class: "{Styles::header()}",
                div {
                    class: "{Styles::flex_between()}",
                    style: "flex-wrap: wrap; gap: 0.5rem;",
                    CoreButtons { on_menu_change: move |opt| menu_option.set(opt) }
                    CryptoButtons { on_menu_change: move |opt| menu_option.set(opt) }
                    ConnectionButtons { on_menu_change: move |opt| menu_option.set(opt) }                
                }
                div {
                    TransactionButtons { on_menu_change: move |opt| menu_option.set(opt) }
                    ManagementButtons { on_menu_change: move |opt| menu_option.set(opt) }
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
                    MenuOption::Memes => rsx! { MemeManagement {} },
                    MenuOption::ExpressionParsing => rsx! { ExpressionParsing {} },
                    MenuOption::Encryption => rsx! { Encryption {} },
                    MenuOption::MetaMemeOperations => rsx! { MetaMemeOperations {} },
                    MenuOption::StylingAndEmojis => rsx! { StylingAndEmojis {} },
                    //MenuOption::CryptoFrontend => rsx! { CryptoFrontendApp {} },
                    //MenuOption::Lean => rsx! { LeanEditor {} },
                    //MenuOption::ConnectionManagement => rsx! { ConnectionManagement {} },
                    //MenuOption::ConnectionList => rsx! { ConnectionList {} },
                    MenuOption::SendSol => rsx! { SendSol { show_send_modal: use_signal(|| true) } },
                    MenuOption::ReceiveSol => rsx! { ReceiveSol { show_receive_modal: use_signal(|| true) } },
                    //MenuOption::QueryAccounts => rsx! { QueryAccounts {} },
                    MenuOption::Dashboard => rsx! { Dashboard {} },
                    MenuOption::Connections => rsx! { Connections {} },
                    //MenuOption::ClustersManagement => rsx! { ClustersManagement {} },
                    MenuOption::Clusters => rsx! { Clusters {} },
                    MenuOption::Airdrop => rsx! { Airdrop { show_airdrop_modal: use_signal(|| true) } },
                    MenuOption::Accounts => rsx! { Accounts {} },
                    MenuOption::ComponentMemes => rsx! { ComponentMemeExplorer {} },
		    MenuOption::Embedding => rsx! { EmbeddingApp {} },
                    _ => rsx! { div { "TODO"}}
                }
            }
        }
    }
} 
