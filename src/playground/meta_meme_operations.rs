use dioxus::prelude::*;
use crate::model::lean::style::Styles;
use crate::state::use_memes;
use crate::model::metameme::{
    context, expand, fetch, interpret, prove, LLMProviders, MemeProviders, MetaMemes, ZOS1,
    ZOS1_ARRAY, META_MEME,
};

#[component]

pub fn MetaMemeOperations() -> Element {
    let state = use_memes::use_memes("app");
    let meta_meme = use_signal(|| META_MEME.clone());
    let zos1 = use_signal(|| ZOS1.clone());
    let zos1_array = use_signal(|| ZOS1_ARRAY.clone());

    interpret(meta_meme.read().clone()); // Use interpret
    prove(ZOS1.clone()); // Use prove
    context(ZOS1.clone(), meta_meme.read().clone()); // Use context
    let fetched_meme = use_signal(|| fetch(MemeProviders::GitHub, "https://example.com/meme".to_string()));
    let expanded_meme = use_signal(|| expand(LLMProviders::OpenAI, fetched_meme.read().clone()));
    
    rsx! {
        div {
            class: "section",
            h2 { class: "section-title", "MetaMeme Operations" }
            div {
                                
                "ZOS1Array"
                 {format!("{:?}", zos1_array.read())}
                br {}
                "MetaMemeContent" 
                {format!("{:?}", meta_meme.read().get_value())}
            }
        }
    }
} 