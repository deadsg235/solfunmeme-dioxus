use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
use gloo_timers::future::TimeoutFuture;
use log::error;
use web_sys::{window, HtmlTextAreaElement};
use wasm_bindgen::JsCast;

const STYLE: &str = r#"
.code-extractor {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    font-family: system-ui, -apple-system, sans-serif;
}

.upload-area {
    border: 2px dashed #ccc;
    border-radius: 8px;
    padding: 40px;
    text-align: center;
    margin: 20px 0;
    transition: all 0.3s ease;
}

.upload-area.drag-over {
    border-color: #007bff;
    background-color: #f8f9fa;
}

.file-input {
    margin: 10px 0;
}

.processing-indicator {
    border: 2px solid #007bff;
    border-radius: 8px;
    padding: 20px;
    margin: 20px 0;
    background: #f8f9fa;
}

.progress-bar {
    width: 100%;
    height: 20px;
    background: #e9ecef;
    border-radius: 10px;
    overflow: hidden;
    margin: 10px 0;
}

.progress-fill {
    height: 100%;
    background: #007bff;
    transition: width 0.3s ease;
}

.snippet-container {
    border: 1px solid #dee2e6;
    border-radius: 8px;
    margin: 15px 0;
    overflow: hidden;
}

.snippet-header {
    background: #f8f9fa;
    padding: 10px 15px;
    border-bottom: 1px solid #dee2e6;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.snippet-content {
    max-height: 400px;
    overflow-y: auto;
}

.snippet-code {
    margin: 0;
    padding: 15px;
    background: #f6f8fa;
    font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
    font-size: 14px;
    line-height: 1.4;
    white-space: pre-wrap;
    word-wrap: break-word;
}

.copy-btn {
    background: #007bff;
    color: white;
    border: none;
    padding: 5px 12px;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    transition: background 0.2s;
}

.copy-btn:hover {
    background: #0056b3;
}

.copy-btn.copied {
    background: #28a745;
}

.summary-stats {
    background: #e9ecef;
    padding: 15px;
    border-radius: 8px;
    margin: 20px 0;
}

.clear-btn {
    background: #dc3545;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    margin: 10px 0;
}

.clear-btn:hover {
    background: #c82333;
}
"#;

#[derive(Clone, Debug, PartialEq)]
pub struct CodeSnippet {
    pub language: String,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtractedFile {
    pub name: String,
    pub snippets: Vec<CodeSnippet>,
    pub total_lines: usize,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ProcessingFile {
    pub name: String,
    pub progress: usize,
    pub total_lines: usize,
    pub current_content: String,
}

pub fn MarkdownCodeExtractor() -> Element {
    let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    
    let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let mut hovered = use_signal(|| false);
    let mut copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

    let extract_code_snippets = |content: &str| -> Vec<CodeSnippet> {
        let mut snippets = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_code_block = false;
        let mut current_language = String::new();
        let mut current_content = String::new();
        let mut start_line = 0;

        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("```") {
                if in_code_block {
                    // End of code block
                    if !current_content.trim().is_empty() {
                        snippets.push(CodeSnippet {
                            language: current_language.clone(),
                            content: current_content.clone(),
                            line_start: start_line,
                            line_end: i,
                        });
                    }
                    current_content.clear();
                    in_code_block = false;
                } else {
                    // Start of code block
                    current_language = line[3..].trim().to_string();
                    if current_language.is_empty() {
                        current_language = "text".to_string();
                    }
                    current_content.clear();
                    start_line = i + 1;
                    in_code_block = true;
                }
            } else if in_code_block {
                if !current_content.is_empty() {
                    current_content.push('\n');
                }
                current_content.push_str(line);
            }
        }

        // Handle unclosed code block
        if in_code_block && !current_content.trim().is_empty() {
            snippets.push(CodeSnippet {
                language: current_language,
                content: current_content,
                line_start: start_line,
                line_end: lines.len(),
            });
        }

        snippets
    };

    let copy_to_clipboard = move |text: String, snippet_id: String| {
        let navigator = window().unwrap().navigator();
        let clipboard = navigator.clipboard();
        
        wasm_bindgen_futures::spawn_local(async move {
            let promise = clipboard.write_text(&text);
            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            
            copied_snippets.write().insert(snippet_id.clone());
            
            // Remove the "copied" state after 2 seconds
            gloo_timers::future::TimeoutFuture::new(2000).await;
            copied_snippets.write().remove(&snippet_id);
        });
    };

    let copy_all_snippets = move |snippets: &[CodeSnippet]| {
        let combined = snippets.iter()
            .enumerate()
            .map(|(i, snippet)| {
                format!("// Snippet {} - Language: {}\n// Lines: {}-{}\n{}\n\n", 
                       i + 1, snippet.language, snippet.line_start, snippet.line_end, snippet.content)
            })
            .collect::<Vec<_>>()
            .join("---\n\n");
        
        copy_to_clipboard(combined, "all_snippets".to_string());
    };

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        let file_names = file_engine.files();
        
        for file_name in &file_names {
            processing_file.set(Some(ProcessingFile {
                name: file_name.clone(),
                ..Default::default()
            }));
            
            TimeoutFuture::new(50).await;

            if let Some(content) = file_engine.read_file_to_string(file_name).await {
                let lines: Vec<&str> = content.lines().collect();
                let total_lines = lines.len();

                // Update processing status
                if let Some(mut pf) = processing_file.write().as_mut() {
                    pf.total_lines = total_lines;
                    pf.current_content = content.clone();
                }

                // Simulate progress for visual feedback
                for i in 0..=total_lines {
                    if let Some(mut pf) = processing_file.write().as_mut() {
                        pf.progress = i;
                    }
                    if i % 100 == 0 || i == total_lines {
                        TimeoutFuture::new(10).await;
                    }
                }

                let snippets = extract_code_snippets(&content);
                
                files.write().push(ExtractedFile {
                    name: file_name.clone(),
                    snippets,
                    total_lines,
                });
            }
        }
        
        processing_file.set(None);
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    let get_snippet_id = |file_name: &str, snippet_idx: usize| -> String {
        format!("{}_{}", file_name, snippet_idx)
    };
    
    rsx! {
        style { "{STYLE}" }
        
        div { class: "code-extractor",
            h1 { "📄 Markdown Code Extractor" }
            p { "Upload markdown files to extract and copy code snippets with ease" }
            
            button { 
                class: "clear-btn",
                onclick: move |_| {
                    files.write().clear();
                    copied_snippets.write().clear();
                },
                "🗑️ Clear All Files" 
            }

            div { class: "file-input",
                label { r#for: "file-upload", "📁 Select Markdown Files:" }
                input {
                    id: "file-upload",
                    r#type: "file",
                    accept: ".md,.txt",
                    multiple: true,
                    onchange: upload_files,
                }
            }

            div {
                class: if hovered() { "upload-area drag-over" } else { "upload-area" },
                ondragover: move |evt| {
                    evt.prevent_default();
                    hovered.set(true);
                },
                ondragleave: move |_| hovered.set(false),
                ondrop: move |evt| async move {
                    evt.prevent_default();
                    hovered.set(false);
                    if let Some(file_engine) = evt.files() {
                        read_files(file_engine).await;
                    }
                },
                "🎯 Drop markdown files here or click above to select"
            }

            // Processing indicator
            if let Some(pf) = processing_file() {
                div { class: "processing-indicator",
                    h3 { "🔄 Processing: {pf.name}" }
                    if pf.total_lines > 0 {
                        div {
                            "Progress: {pf.progress} / {pf.total_lines} lines ({((pf.progress as f32 / pf.total_lines as f32) * 100.0) as i32}%)"
                        }
                        div { class: "progress-bar",
                            div { 
                                class: "progress-fill",
                                style: "width: {((pf.progress as f32 / pf.total_lines as f32) * 100.0)}%"
                            }
                        }
                    }
                }
            }

              // Results
	      
              for file in files.read().iter() {
                div { key: "{file.name}",
                    h2 { "📝 {file.name}" }
                    
                    if !file.snippets.is_empty() {
                        div { class: "summary-stats",
                            p { "📊 Found {file.snippets.len()} code snippets in {file.total_lines} lines" }
                            p { 
                                "🔤 Languages: {file.snippets.iter().map(|s| s.language.as_str()).collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>().join(\", \")}"
                            }
                            button {
                                class: if copied_snippets.read().contains("all_snippets") { "copy-btn copied" } else { "copy-btn" },
                                onclick: {
				    let snippets = file.snippets.clone();
				    move |_| copy_all_snippets(&snippets)
				},
                                if copied_snippets.read().contains("all_snippets") { "✅ Copied All!" } else { "📋 Copy All Snippets" }
                            }
                        }

                        for (idx, snippet) in file.snippets.iter().enumerate() {
                            div { class: "snippet-container", key: "{idx}",
                                div { class: "snippet-header",
                                    span { 
                                        "🏷️ {snippet.language} (lines {snippet.line_start}-{snippet.line_end})"
                                    }
                                    button {
                                        class: if copied_snippets.read().contains(&get_snippet_id(&file.name, idx)) { "copy-btn copied" } else { "copy-btn" },
                                        onclick: {
                                            let content = snippet.content.clone();
                                            let snippet_id = get_snippet_id(&file.name, idx);
                                            move |_| copy_to_clipboard(content.clone(), snippet_id.clone())
                                        },
                                        if copied_snippets.read().contains(&get_snippet_id(&file.name, idx)) { "✅ Copied!" } else { "📋 Copy" }
                                    }
                                }
                                div { class: "snippet-content",
                                    pre { class: "snippet-code", "{snippet.content}" }
                                }
                            }
                        }
                    } else {
                        p { "ℹ️ No code snippets found in this file" }
                    }
                }
            }

            if files.read().is_empty() && processing_file().is_none() {
                div { class: "summary-stats",
                    p { "👋 Welcome! Upload some markdown files to get started extracting code snippets." }
                    p { "💡 This tool will find all code blocks wrapped in ``` and make them easy to copy." }
                }
            }
        }
    }
}
