use dioxus::prelude::*;
use crate::models::emoji_matrix::{self, EmojiMatrix, EmojiMatrixEntry};
use log::info;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "reports/"]
struct EmbeddedReports;

#[component]
pub fn EmojiMatrixView() -> Element {
    let mut emoji_matrix = use_signal::<Option<EmojiMatrix>>(|| None);

    use_future(move || async move {
        info!("Starting EmojiMatrixView data fetch.");
        let mut combined_entries: Vec<EmojiMatrixEntry> = Vec::new();

        // Read summary_total.txt
        if let Some(file) = EmbeddedReports::get("summary_total.txt") {
            if let Ok(text) = String::from_utf8(file.data.into_owned()) {
                info!("Successfully read summary_total.txt.");
                let total_matrix = emoji_matrix::parse_summary_total(&text);
                combined_entries.extend(total_matrix.entries);
            } else {
                log::error!("Failed to convert summary_total.txt to UTF-8.");
            }
        } else {
            log::error!("Failed to find summary_total.txt in embedded reports.");
        }

        // Read summary_root.txt
        if let Some(file) = EmbeddedReports::get("summary_root.txt") {
            if let Ok(text) = String::from_utf8(file.data.into_owned()) {
                info!("Successfully read summary_root.txt.");
                let root_matrix = emoji_matrix::parse_summary_root(&text);
                combined_entries.extend(root_matrix.entries);
            } else {
                log::error!("Failed to convert summary_root.txt to UTF-8.");
            }
        } else {
            log::error!("Failed to find summary_root.txt in embedded reports.");
        }

        let mut final_matrix = EmojiMatrix { entries: combined_entries };
        info!("Before rollup: {} entries", final_matrix.entries.len());
        final_matrix = emoji_matrix::rollup_emoji_matrix(final_matrix);
        info!("After rollup: {} entries", final_matrix.entries.len());

        emoji_matrix.set(Some(final_matrix));
        info!("Emoji matrix set.");
    });

    rsx! {
        div {
            h1 { "Emoji Matrix" }
            if let Some(matrix) = emoji_matrix() {
                table {
                    class: "min-w-full divide-y divide-gray-200",
                    thead {
                        tr {
                            class: "bg-gray-50",
                            th {
                                class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Path"
                            }
                            th {
                                class: "px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider",
                                "Emojis"
                            }
                        }
                    }
                    tbody {
                        class: "bg-white divide-y divide-gray-200",
                        for entry in &matrix.entries {
                            tr {
                                td {
                                    class: "px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900",
                                    "{entry.path.clone()}"
                                }
                                td {
                                    class: "px-6 py-4 whitespace-nowrap text-sm text-gray-500",
                                    for (emoji, count) in &entry.emoji_counts {
                                        span {
                                            class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 mr-2 mb-2",
                                            "{emoji} ({count})"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                p { "Loading..." }
            }
        }
    }
}
