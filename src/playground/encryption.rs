use dioxus::prelude::*;
use crate::model::lean::style::Styles;
use crate::model::crypto::{CryptoError, SolanaEncryption};
use crate::model::wallet_error::MyWalletError;

#[component]
pub fn Encryption() -> Element {
    let private_key = use_signal(|| "sample_private_key".to_string());
    let public_key = use_signal(|| "sample_public_key".to_string());
    let recipient = use_signal(|| "recipient_public_key".to_string());
    let sender = use_signal(|| "sender_public_key".to_string());

    let priv_key_valid = use_signal(|| SolanaEncryption::validate_private_key(&private_key.read()));
    let pub_key_valid = use_signal(|| SolanaEncryption::validate_public_key(&public_key.read()));
    let wallet_valid = use_signal(|| Ok::<(), MyWalletError>(()));

    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { class: "{Styles::section_title()}", "Encryption" }
            div {
                {format!("Private Key Valid: {:?}", priv_key_valid.read())}
                br {}
                {format!("Public Key Valid: {:?}", pub_key_valid.read())}
                br {}
                {format!("Wallet Valid: {:?}", wallet_valid.read())}
            }
        }
    }
} 