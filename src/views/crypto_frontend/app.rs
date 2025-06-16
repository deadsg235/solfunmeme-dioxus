use dioxus::prelude::*;
use crate::views::crypto_style::*;
use super::forms::{EncryptionForm, DecryptionForm};

#[component]
pub fn CryptoFrontendApp() -> Element {
    rsx! {
        div { class: APP_CONTAINER,
            div { class: MAIN_WRAPPER,
                AppHeader {}
                div { class: GRID_LAYOUT,
                    EncryptionForm {}
                    DecryptionForm {}
                }
            }
        }
    }
}

#[component]
pub fn AppHeader() -> Element {
    rsx! {
        h1 { class: MAIN_TITLE,
            "Solana Wallet Encryption Demo"
        }
    }
} 