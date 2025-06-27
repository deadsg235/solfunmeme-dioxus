use std::sync::Arc;
use dioxus::prelude::*;

use dioxus::html::FileEngine;
use gloo_timers::future::TimeoutFuture;

use crate::extractor::types::{CodeSnippet, ExtractedFile, ProcessingFile};

pub fn extract_code_snippets(content: &str) -> Vec<CodeSnippet> {
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
                        content_hash: todo!(),
                        token_count: todo!(),
                        line_count: todo!(),
                        char_count: todo!(),
                        test_result: todo!(),
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
            content_hash: todo!(),
            token_count: todo!(),
            line_count: todo!(),
            char_count: todo!(),
            test_result: todo!(),
        });
    }
    
    snippets
}

fn extract_code_snippets_from_content(content: &str) -> Vec<CodeSnippet> {
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
                        content_hash: todo!(),
                        token_count: todo!(),
                        line_count: todo!(),
                        char_count: todo!(),
                        test_result: todo!(),
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
            content_hash: todo!(),
                        token_count: todo!(),
                        line_count: todo!(),
                        char_count: todo!(),
                        test_result: todo!(),
        });
    }

    snippets
}



// ### 5. File Processing Function
// ```rust
pub async fn process_file_engine(
    file_engine: Arc<dyn FileEngine>,
    mut files: Signal<Vec<ExtractedFile>>,
    mut processing_file: Signal<Option<ProcessingFile>>
) {
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

            let snippets = extract_code_snippets_from_content(&content);
            
            files.write().push(ExtractedFile {
                name: file_name.clone(),
                snippets,
                total_lines,
            });
        }
    }
    
    processing_file.set(None);
}
// ```

// ### 6. Utility Function
// ```rust
fn generate_snippet_id(file_name: &str, snippet_idx: usize) -> String {
    format!("{}_{}", file_name, snippet_idx)
}
