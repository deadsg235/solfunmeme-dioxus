use dioxus::prelude::*;
//use crate::extractor::ExtractedFile;
use std::collections::HashSet;
use crate::extractor::types::ExtractedFile;

#[component]
pub fn ClearButton(files: Signal<Vec<ExtractedFile>>, copied_snippets: Signal<HashSet<String>>) -> Element {
    rsx! {
        button { 
            class: "clear-btn",
            onclick: move |_| {
                files.write().clear();
                copied_snippets.write().clear();
            },
            "🗑️ Clear All Files" 
        }
    }
}
