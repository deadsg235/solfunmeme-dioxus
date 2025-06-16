use dioxus::prelude::*;
use crate::model::lean::style::Styles;
use crate::state::use_memes;
use crate::model::memes::{Controller, LiftedExpression, Meme, Quine};

fn processmeme(id: &String, expr: &LiftedExpression) -> Element {
    let similarity = Controller::get_vector_similarity(expr, expr);
    rsx! {
        div {
            class: {Styles::card()},
            onmouseover: move |_| {}, // Trigger card_hover
            style: {Styles::card_hover()},
            "ID: {id}"
            br {}
            br {}
            "Similarity: {similarity}"
        }
    }
}

#[component]
pub fn MemeManagement() -> Element {
    let state = use_memes::use_memes("app");
    let meme = use_signal(|| Meme::new("Meme content".to_string(), vec!["tag1".to_string(), "tag2".to_string()]));
    let quine = use_signal(|| Quine::new("Self-referential meme".to_string()));
    let expr = use_signal(|| LiftedExpression::from_meme(meme.read().clone()));

    rsx! {
        div {
            class: "{Styles::section()}",
            h2 { "Meme Management" }
            div {
                "Meme content placeholder"
            }
        }
    }
} 