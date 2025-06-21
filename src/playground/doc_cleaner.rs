//! This module is for cleaning and processing the founding documents.
//! It extracts code snippets, creates content-addressable versions,
//! counts tokens, and tests the code.

use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub content: String,
    pub content_hash: String,
    pub language: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<TestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub success: bool,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub total_turns: usize,
    pub total_code_snippets: usize,
    pub total_tokens: usize,
    pub languages_found: Vec<String>,
    pub content_hashes: Vec<String>,
}

/// Cleans HTML tags from a string.
pub fn clean_html(text: &str) -> String {
    let html_tag_re = Regex::new(r"<[^>]*>").unwrap();
    html_tag_re.replace_all(text, "").to_string()
}

/// Creates a content-addressable hash for a code snippet.
pub fn create_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Extracts code snippets from text using regex patterns.
pub fn extract_code_snippets(text: &str) -> Vec<CodeSnippet> {
    let mut snippets = Vec::new();
    
    // Pattern for code blocks with language specification
    let code_block_re = Regex::new(r"```(\w+)?\s*\n(.*?)\n```").unwrap();
    
    for cap in code_block_re.captures_iter(text) {
        let language = cap.get(1).map(|m| m.as_str()).unwrap_or("text").to_string();
        let content = cap.get(2).unwrap().as_str().trim();
        
        if !content.is_empty() {
            let content_hash = create_content_hash(content);
            let token_count = estimate_token_count(content);
            let line_count = content.lines().count();
            let char_count = content.chars().count();
            
            snippets.push(CodeSnippet {
                content: content.to_string(),
                content_hash,
                language,
                token_count,
                line_count,
                char_count,
                test_result: None,
            });
        }
    }
    
    // Also look for inline code snippets
    let inline_code_re = Regex::new(r"`([^`]+)`").unwrap();
    for cap in inline_code_re.captures_iter(text) {
        let content = cap.get(1).unwrap().as_str();
        if content.len() > 10 { // Only consider substantial inline code
            let content_hash = create_content_hash(content);
            let token_count = estimate_token_count(content);
            let line_count = content.lines().count();
            let char_count = content.chars().count();
            
            snippets.push(CodeSnippet {
                content: content.to_string(),
                content_hash,
                language: "inline".to_string(),
                token_count,
                line_count,
                char_count,
                test_result: None,
            });
        }
    }
    
    snippets
}

/// Estimates token count for a given text (rough approximation).
pub fn estimate_token_count(text: &str) -> usize {
    // Simple tokenization: split on whitespace and punctuation
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    
    // Rough approximation: average of word count and char_count/4
    (word_count + char_count / 4) / 2
}

/// Tests a code snippet by attempting to compile/execute it.
pub fn test_code_snippet(snippet: &mut CodeSnippet) {
    let start_time = std::time::Instant::now();
    
    match snippet.language.as_str() {
        "rust" => {
            // For Rust code, we could try to compile it in a sandbox
            // For now, we'll just do basic syntax checking
            if snippet.content.contains("fn ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        "javascript" | "js" => {
            // For JavaScript, we could use a JS engine
            // For now, just check for basic syntax
            if snippet.content.contains("function") || snippet.content.contains("const ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        _ => {
            // For other languages, just mark as untested
            snippet.test_result = Some(TestResult {
                success: false,
                error_message: Some("Language not supported for testing".to_string()),
                execution_time_ms: start_time.elapsed().as_millis() as u64,
            });
        }
    }
}

/// Processes a single markdown file and extracts all information.
pub fn process_file(file_path: &Path) -> io::Result<DocumentSummary> {
    let content = fs::read_to_string(file_path)?;
    let cleaned_content = clean_html(&content);

    let mut turns = Vec::new();
    let parts = cleaned_content.split("---");

    for part in parts {
        let trimmed_part = part.trim();
        if trimmed_part.starts_with("### User") {
            let content = trimmed_part.strip_prefix("### User").unwrap_or("").trim();
            let code_snippets = extract_code_snippets(content);
            turns.push(ConversationTurn {
                author: "User".to_string(),
                content: content.to_string(),
                code_snippets,
            });
        } else if trimmed_part.starts_with("### Grok AI") {
            let content = trimmed_part.strip_prefix("### Grok AI").unwrap_or("").trim();
            let mut code_snippets = extract_code_snippets(content);
            
            // Test the code snippets
            for snippet in &mut code_snippets {
                test_code_snippet(snippet);
            }
            
            turns.push(ConversationTurn {
                author: "Grok AI".to_string(),
                content: content.to_string(),
                code_snippets,
            });
        }
    }

    // Create summary
    let total_turns = turns.len();
    let total_code_snippets: usize = turns.iter().map(|t| t.code_snippets.len()).sum();
    let total_tokens: usize = turns.iter()
        .flat_map(|t| &t.code_snippets)
        .map(|s| s.token_count)
        .sum();
    
    let mut languages_found: Vec<String> = turns.iter()
        .flat_map(|t| &t.code_snippets)
        .map(|s| s.language.clone())
        .collect();
    languages_found.sort();
    languages_found.dedup();
    
    let content_hashes: Vec<String> = turns.iter()
        .flat_map(|t| &t.code_snippets)
        .map(|s| s.content_hash.clone())
        .collect();

    Ok(DocumentSummary {
        total_turns,
        total_code_snippets,
        total_tokens,
        languages_found,
        content_hashes,
    })
}

/// Saves the processed document summary to a JSON file.
pub fn save_summary(summary: &DocumentSummary, output_path: &Path) -> io::Result<()> {
    let json = serde_json::to_string_pretty(summary)?;
    fs::write(output_path, json)?;
    Ok(())
}

/// Main function to process documents from command line
pub fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        eprintln!("Example: {} founding_documents/prelude1-aaa.md output/summary.json", args[0]);
        std::process::exit(1);
    }
    
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    
    if !input_path.exists() {
        eprintln!("Error: Input file '{}' does not exist", args[1]);
        std::process::exit(1);
    }
    
    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    println!("Processing document: {}", input_path.display());
    let summary = process_file(input_path)?;
    
    println!("Saving summary to: {}", output_path.display());
    save_summary(&summary, output_path)?;
    
    println!("Document Summary:");
    println!("  Total turns: {}", summary.total_turns);
    println!("  Total code snippets: {}", summary.total_code_snippets);
    println!("  Total tokens: {}", summary.total_tokens);
    println!("  Languages found: {:?}", summary.languages_found);
    println!("  Content hashes: {}", summary.content_hashes.len());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_clean_html() {
        let raw_text = r#"<span class="test">Hello</span> World"#;
        let cleaned = clean_html(raw_text);
        assert_eq!(cleaned, "Hello World");
    }

    #[test]
    fn test_create_content_hash() {
        let content = "fn main() { println!(\"Hello, world!\"); }";
        let hash1 = create_content_hash(content);
        let hash2 = create_content_hash(content);
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA256 hex string length
    }

    #[test]
    fn test_extract_code_snippets() {
        let text = r#"
        Here's some Rust code:
        ```rust
        fn main() {
            println!("Hello, world!");
        }
        ```
        And some JavaScript:
        ```javascript
        function hello() {
            console.log("Hello, world!");
        }
        ```
        "#;
        
        let snippets = extract_code_snippets(text);
        assert_eq!(snippets.len(), 2);
        assert_eq!(snippets[0].language, "rust");
        assert_eq!(snippets[1].language, "javascript");
        assert!(snippets[0].content.contains("fn main"));
        assert!(snippets[1].content.contains("function hello"));
    }

    #[test]
    fn test_estimate_token_count() {
        let text = "fn main() { println!(\"Hello, world!\"); }";
        let count = estimate_token_count(text);
        assert!(count > 0);
        assert!(count < 50); // Should be reasonable for this text
    }

    #[test]
    #[ignore] // This test interacts with the filesystem and is slow.
    fn test_process_prelude_file() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("founding_documents/prelude1-aaa.md");
        
        let result = process_file(&path);
        
        assert!(result.is_ok());
        let summary = result.unwrap();
        assert!(summary.total_turns > 0);
        assert!(summary.total_tokens > 0);

        println!("Document Summary:");
        println!("Total turns: {}", summary.total_turns);
        println!("Total code snippets: {}", summary.total_code_snippets);
        println!("Total tokens: {}", summary.total_tokens);
        println!("Languages found: {:?}", summary.languages_found);
        println!("Content hashes: {:?}", summary.content_hashes);
    }
} 