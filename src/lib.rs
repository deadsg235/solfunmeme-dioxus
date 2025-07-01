//! # Solfunmeme Dioxus
//! 
//! A comprehensive code analysis and vectorization platform that combines:
//! - Code vectorization and analysis
//! - Declaration splitting and meme generation  
//! - Duplicate detection and canonical directories
//! - Wallet integration with encrypted secret management
//! - Biosemiotic properties analysis
//! 
//! ## Core Modules
//! 
//! - `core::vectorization` - Transform code into executable vector representations
//! - `core::declaration_splitter` - Split code declarations into individual files
//! - `core::duplicate_detector` - Identify and manage code duplicates
//! - `core::code_analyzer` - Comprehensive code analysis and metrics
//! - `core::meme_generator` - Generate memes with biosemiotic properties
//! - `core::wallet_integration` - Secure wallet and secret management

pub mod core;
pub mod model;
pub mod views;
pub mod state;
pub mod playground;
pub mod extractor;
pub mod embedself;

// Re-export core functionality for easy access
pub use core::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        // Test that we can create core components
        let vectorizer = CodeVectorizer::new(64);
        let vector = vectorizer.vectorize("fn test() {}");
        assert_eq!(vector.dimensions.len(), 64);
        
        let mut splitter = DeclarationSplitter::new();
        assert_eq!(splitter.declarations.len(), 0);
        
        let detector = DuplicateDetector::new(64, 0.8);
        let empty_report = detector.detect_duplicates(&[]);
        assert_eq!(empty_report.total_duplicates, 0);
        
        let mut analyzer = CodeAnalyzer::new(64, 0.8);
        let result = analyzer.analyze_file("fn empty() {}", "test.rs".to_string());
        assert!(result.is_ok());
        
        let generator = MemeGenerator::new(64);
        let analysis = result.unwrap();
        let ecosystem = generator.create_meme_ecosystem(&[analysis]);
        assert!(ecosystem.memes.len() > 0);

        
        let mut wallet = WalletManager::new();
        assert!(wallet.initialize_with_password("test").is_ok());
    }
}