use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
use crate::extractor::types::{ProcessingFile, ExtractedFile, CodeSnippet};


// #[component]
// pub fn FileUploadArea<
// 	F: 'static + std::cmp::PartialEq,
//     G: 'static + std::cmp::PartialEq
// 	>(
//     hovered: bool,
//     on_hover: F,
//     on_upload: G,
// ) -> Element 
// where
//     F: Fn(bool) + 'static,
//     G: Fn(FormEvent) + 'static,



    #[component]
    pub fn WelcomeMessage(show: bool) -> Element {
        if show {
            rsx! {
                div { class: "summary-stats",
                    p { "👋 Welcome! Upload some markdown files to get started extracting code snippets." }
                    p { "💡 This tool will find all code blocks wrapped in ``` and make them easy to copy." }
                }
            }
        } else {
            rsx! { div {} }
        }
    }
//}
