pub mod process_file;
pub mod project_analyzer;

pub use shared_analysis_types::{AnalyzedFunction, AnalyzedDocument, AnalyzedToken, ClosestEmojiInfo};

#[cfg(test)]
pub mod tests;
