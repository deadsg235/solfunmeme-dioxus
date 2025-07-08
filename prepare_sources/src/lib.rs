pub mod clifford;
pub mod embedding;
pub mod process_file;
pub mod load_emoji_multivectors;
pub mod sieve;
pub mod function_analyzer;
pub mod project_analyzer;
pub mod ontology_generator;
pub use self::function_analyzer::FunctionInfo;

#[cfg(test)]
pub mod tests;