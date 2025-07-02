use serde::{Deserialize, Serialize};

// Core types for code extraction
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub language: String,
    pub content: String,
    pub content_hash: String,
    pub line_start: usize,
    pub line_end: usize,
    pub token_count: usize,
    pub line_count: usize,
    pub char_count: usize,
    pub test_result: Option<TestResult>,
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

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Multivector {
    pub scalar: f32,
    pub vector: [f32; 3],
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

// Utility functions for type conversions and validation
impl CodeSnippet {
    pub fn new(language: String, content: String, line_start: usize, line_end: usize) -> Self {
        Self {
            language,
            content,
            line_start,
            line_end,
            ..Default::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }

    pub fn estimated_reading_time_seconds(&self) -> u32 {
        // Rough estimate: 200 words per minute reading speed
        let word_count = self.content.split_whitespace().count();
        ((word_count as f32 / 200.0) * 60.0) as u32
    }
}

impl ExtractedFile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            snippets: Vec::new(),
            total_lines: 0,
        }
    }

    pub fn add_snippet(&mut self, snippet: CodeSnippet) {
        self.snippets.push(snippet);
    }

    pub fn get_languages(&self) -> Vec<String> {
        let mut languages: Vec<String> = self.snippets.iter().map(|s| s.language.clone()).collect();
        languages.sort();
        languages.dedup();
        languages
    }

    pub fn total_code_lines(&self) -> usize {
        self.snippets.iter().map(|s| s.line_count).sum()
    }
}

impl ProcessingFile {
    pub fn new(name: String) -> Self {
        Self {
            name,
            progress: 0,
            total_lines: 0,
            current_content: String::new(),
            summary: None,
        }
    }

    pub fn progress_percentage(&self) -> f32 {
        if self.total_lines == 0 {
            0.0
        } else {
            (self.progress as f32 / self.total_lines as f32) * 100.0
        }
    }
}

// Constants for common languages
pub const SUPPORTED_LANGUAGES: &[&str] = &[
    "rust",
    "javascript",
    "typescript",
    "python",
    "java",
    "cpp",
    "c",
    "html",
    "css",
    "json",
    "xml",
    "yaml",
    "toml",
    "sql",
    "bash",
    "powershell",
];

pub fn get_language_config(language: &str) -> LanguageConfig {
    match language.to_lowercase().as_str() {
        "rust" | "rs" => LanguageConfig {
            name: "Rust".to_string(),
            file_extension: "rs".to_string(),
            comment_prefix: "//".to_string(),
            supports_testing: true,
            syntax_highlighter: Some("rust".to_string()),
        },
        "javascript" | "js" => LanguageConfig {
            name: "JavaScript".to_string(),
            file_extension: "js".to_string(),
            comment_prefix: "//".to_string(),
            supports_testing: true,
            syntax_highlighter: Some("javascript".to_string()),
        },
        "python" | "py" => LanguageConfig {
            name: "Python".to_string(),
            file_extension: "py".to_string(),
            comment_prefix: "#".to_string(),
            supports_testing: true,
            syntax_highlighter: Some("python".to_string()),
        },
        _ => LanguageConfig::default(),
    }
}
