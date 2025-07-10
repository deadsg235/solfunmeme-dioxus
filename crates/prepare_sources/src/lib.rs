pub mod process_file;
pub mod project_analyzer;

pub use solfunmeme_function_analysis::{CodeChunk, FunctionInfo, AnalyzedDocument, AnalyzedToken, Multivector, ExtractedFile, ProcessingFile, TestResult, DocumentSummary, ConversationTurn, UploadedFile, AnnotatedWord, ProcessingStats, ProcessingError, LanguageConfig, extract_code_snippets, create_code_snippet, analyze_rust_file, find_rust_files, AnalyzedFunction, ClosestEmojiInfo};
