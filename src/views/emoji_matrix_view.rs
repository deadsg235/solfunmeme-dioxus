use dioxus::prelude::*;
use crate::models::emoji_matrix::{self, EmojiMatrix, EmojiMatrixEntry};

#[component]
pub fn EmojiMatrixView() -> Element {
    let mut emoji_matrix = use_signal::<Option<EmojiMatrix>>(|| None);

    use_future(move || async move {
        let client = reqwest::Client::new();

        // Fetch summary_total.txt
        let total_res = client.get("http://localhost:8080/reports/summary_total.txt").send().await;
        let mut combined_entries: Vec<EmojiMatrixEntry> = Vec::new();

        if let Ok(res) = total_res {
            if let Ok(text) = res.text().await {
                let total_matrix = emoji_matrix::parse_summary_total(&text);
                combined_entries.extend(total_matrix.entries);
            }
        }

        // Fetch summary_root.txt
        let root_res = client.get("http://localhost:8080/reports/summary_root.txt").send().await;
        if let Ok(res) = root_res {
            if let Ok(text) = res.text().await {
                let root_matrix = emoji_matrix::parse_summary_root(&text);
                combined_entries.extend(root_matrix.entries);
            }
        }

        let mut final_matrix = EmojiMatrix { entries: combined_entries };
        final_matrix = emoji_matrix::rollup_emoji_matrix(final_matrix);

        emoji_matrix.set(Some(final_matrix));
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
