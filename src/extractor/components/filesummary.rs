use dioxus::prelude::*;
//use crate::extractor::components::filesummary::FileSummary;
//use crate::extractor::ExtractedFile;
use std::collections::HashSet;
//use crate::extractor::CodeSnippet;
use crate::extractor::types::{CodeSnippet, ExtractedFile};
#[component]
pub fn FileSummary(
    file: ExtractedFile,
    copied_snippets: Signal<HashSet<String>>,
    copy_all_snippets: EventHandler<Vec<CodeSnippet>>, // Changed from String to Vec<CodeSnippet>
) -> Element {
    rsx! {
        div { class: "summary-stats",
            p { "📊 Found {file.snippets.len()} code snippets in {file.total_lines} lines" }
            p {
                "🔤 Languages: {file.snippets.iter().map(|s| s.language.as_str()).collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>().join(\", \")}"
            }
            button {
                class: if copied_snippets.read().contains("all_snippets") { "copy-btn copied" } else { "copy-btn" },
                onclick: move |_| copy_all_snippets.call(file.snippets.clone()),  // Pass the actual snippets
                if copied_snippets.read().contains("all_snippets") { "✅ Copied All!" } else { "📋 Copy All Snippets" }
            }
        }
    }
}
