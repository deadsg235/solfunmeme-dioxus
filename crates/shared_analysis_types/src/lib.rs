use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClosestEmojiInfo {
    pub emoji: String,
    pub category: String,
    pub distance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedFunction {
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub file_path: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emojis: Vec<ClosestEmojiInfo>,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedDocument {
    pub file_path: String,
    pub code_snippets: Vec<CodeSnippet>,
    pub text_chunks: Vec<String>,
    pub analyzed_snippets: Vec<AnalyzedFunction>, // Reusing AnalyzedFunction for snippets
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnalyzedToken {
    pub token: String,
    pub count: usize,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

// Simplified CodeSnippet struct for prepare_sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub content: String,
    pub line_start: usize,
    pub line_end: usize,
    pub content_hash: String,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<String>, // Simplified from original
}

// Placeholder for generate_content_hash
pub fn generate_content_hash(content: &str) -> String {
    // Simple placeholder hash for now
    format!("{:x}", md5::compute(content))
}

// Placeholder for estimate_token_count
pub fn estimate_token_count(content: &str) -> usize {
    content.split_whitespace().count()
}

// Placeholder for create_default_test_result
pub fn create_default_test_result() -> String {
    "Untested".to_string()
}

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
                    let snippet = create_code_snippet(
                        current_language.clone(),
                        current_content.clone(),
                        start_line,
                        i,
                    );
                    snippets.push(snippet);
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
        let snippet =
            create_code_snippet(current_language, current_content, start_line, lines.len());
        snippets.push(snippet);
    }

    snippets
}

pub fn create_code_snippet(
    language: String,
    content: String,
    line_start: usize,
    line_end: usize,
) -> CodeSnippet {
    let content_hash = generate_content_hash(&content);
    let token_count = estimate_token_count(&content);
    let line_count = content.lines().count();
    let char_count = content.chars().count();
    let test_result = Some(create_default_test_result());

    CodeSnippet {
        language,
        content,
        line_start,
        line_end,
        content_hash,
        token_count,
        line_count,
        char_count,
        test_result,
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Multivector {
    pub scalar: f32,
    pub vector: [f32; 3],
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExtractedFile {
    pub name: String,
    pub snippets: Vec<CodeSnippet>,
    pub total_lines: usize,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessingFile {
    pub name: String,
    pub progress: usize,
    pub total_lines: usize,
    pub current_content: String,
    pub summary: Option<DocumentSummary>,
}

// Test execution results
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub error_message: Option<String>,
    pub execution_time: Option<std::time::Duration>,
    pub output: Option<String>,
}

// Document analysis and summary
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentSummary {
    pub total_turns: usize,
    pub total_code_snippets: usize,
    pub total_tokens: usize,
    pub languages_found: Vec<String>,
    pub content_hashes: Vec<String>,
}

// Conversation processing
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub author: String,
    pub content: String,
    pub code_snippets: Vec<CodeSnippet>,
}

// File upload and processing
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UploadedFile {
    pub name: String,
    pub contents: String,
    pub generated_program: String,
    pub summary: Option<DocumentSummary>,
    pub zip_url: Option<String>,
}

// Advanced text processing with embeddings
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnnotatedWord {
    pub word: String,
    pub primary_emoji: String,
    pub secondary_emoji: String,
    pub wikidata: Option<String>,
    pub embedding: Vec<f32>,
    pub multivector: Multivector,
}

// Processing states and statistics
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProcessingStats {
    pub files_processed: usize,
    pub total_snippets_extracted: usize,
    pub total_lines_processed: usize,
    pub processing_time_ms: u64,
    pub languages_detected: Vec<String>,
}

// Error handling
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProcessingError {
    FileReadError(String),
    ParseError(String),
    TestExecutionError(String),
    InvalidFormat(String),
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessingError::FileReadError(msg) => write!(f, "File read error: {}", msg),
            ProcessingError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ProcessingError::TestExecutionError(msg) => write!(f, "Test execution error: {}", msg),
            ProcessingError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
        }
    }
}

impl std::error::Error for ProcessingError {}

// Language-specific configuration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub file_extension: String,
    pub comment_prefix: String,
    pub supports_testing: bool,
    pub syntax_highlighter: Option<String>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            name: "text".to_string(),
            file_extension: "txt".to_string(),
            comment_prefix: "#".to_string(),
            supports_testing: false,
            syntax_highlighter: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeChunk {
    pub path: String,
    pub content: String,
    pub emoji: String, // Placeholder for now
    pub line_start: u32,
    pub line_end: u32,
    pub chunk_type: String,
}
