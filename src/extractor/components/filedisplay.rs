use dioxus::prelude::*;
//use crate::extractor::FileEngine;
//use crate::extractor::ExtractedFile;
use std::collections::HashSet;
//use crate::extractor::CodeSnippet;
use crate::extractor::types::CodeSnippet;
use crate::extractor::components::filesummary::FileSummary;
use crate::extractor::components::codesnippet::CodeSnippetComponent;
use crate::extractor::types::ExtractedFile;
#[component]
pub fn FileDisplay(
    file: ExtractedFile,
    copied_snippets: Signal<HashSet<String>>,
    copy_all_snippets: EventHandler<Vec<CodeSnippet>>,
    copy_to_clipboard: EventHandler<(String, String)>
) -> Element {
//    let get_snippet_id = |file_name: &str, snippet_idx: usize| -> String {
//        format!("{}_{}", file_name, snippet_idx)
//    };

    rsx! {
        div { key: "{file.name}",
            h2 { "📝 {file.name}" }
            
            if !file.snippets.is_empty() {
                FileSummary { 
                    file: file.clone(), 
                    copied_snippets, 
                    copy_all_snippets 
                }

                for (idx, snippet) in file.snippets.iter().enumerate() {
                    CodeSnippetComponent {
                        snippet: snippet.clone(),
                        file_name: file.name.clone(),
                        idx,
                        copied_snippets,
                        copy_to_clipboard
                    }
                }
            } else {
                p { "ℹ️ No code snippets found in this file" }
            }
        }
    }
}
