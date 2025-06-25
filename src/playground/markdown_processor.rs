use markdown::{to_mdast, ParseOptions};
use markdown::mdast::{Node, Code, InlineCode};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationTurn {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodeSnippet {
    pub content: String,
    pub content_hash: String,
    pub language: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<TestResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TestResult {
    pub success: bool,
    pub error_message: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentSummary {
    pub total_turns: usize,
    pub total_code_snippets: usize,
    pub total_tokens: usize,
    pub languages_found: Vec<String>,
    pub content_hashes: Vec<String>,
}

pub fn create_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn extract_code_snippets(text: &str) -> Result<Vec<CodeSnippet>, markdown::message::Message> {
    let mut snippets = Vec::new();
    let ast = to_mdast(text, &ParseOptions::default())?;
    walk_ast(&ast, &mut snippets);
    Ok(snippets)
}

fn walk_ast(node: &Node, snippets: &mut Vec<CodeSnippet>) {
    match node {
        Node::Code(code) => {
            let trimmed_content = code.value.trim();
            if !trimmed_content.is_empty() {
                let content_hash = create_content_hash(trimmed_content);
                let token_count = estimate_token_count(trimmed_content);
                let line_count = trimmed_content.lines().count();
                let char_count = trimmed_content.chars().count();
                let language = code.lang.as_deref().unwrap_or("text").to_string();

                snippets.push(CodeSnippet {
                    content: trimmed_content.to_string(),
                    content_hash,
                    language,
                    token_count,
                    line_count,
                    char_count,
                    test_result: None,
                });
            }
        }
        Node::InlineCode(inline_code) => {
            let trimmed_content = inline_code.value.trim();
            if trimmed_content.len() >= 5 {
                let content_hash = create_content_hash(trimmed_content);
                let token_count = estimate_token_count(trimmed_content);
                let line_count = trimmed_content.lines().count();
                let char_count = trimmed_content.chars().count();

                snippets.push(CodeSnippet {
                    content: trimmed_content.to_string(),
                    content_hash,
                    language: "inline".to_string(),
                    token_count,
                    line_count,
                    char_count,
                    test_result: None,
                });
            }
        }
        _ => {
            if let Some(children) = node.children() {
                for child in children {
                    walk_ast(child, snippets);
                }
            }
        }
    }
}

pub fn estimate_token_count(text: &str) -> usize {
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    (word_count + char_count / 4) / 2
}

pub fn test_code_snippet(snippet: &mut CodeSnippet) {
    let start_time = std::time::Instant::now();
    match snippet.language.as_str() {
        "rust" => {
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
            snippet.test_result = Some(TestResult {
                success: false,
                error_message: Some("Language not supported for testing".to_string()),
                execution_time_ms: start_time.elapsed().as_millis() as u64,
            });
        }
    }
}
