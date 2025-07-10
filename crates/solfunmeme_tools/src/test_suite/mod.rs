pub mod args;
pub mod vectorization_tests;
pub mod declaration_splitting_tests;
pub mod duplicate_detection_tests;
pub mod code_analysis_tests;
pub mod wallet_integration_tests;
pub mod self_analysis_tests;

pub use args::TestSuiteArgs;
pub use vectorization_tests::run_vectorization_tests;
pub use declaration_splitting_tests::run_declaration_splitting_tests;
pub use duplicate_detection_tests::run_duplicate_detection_tests;
pub use code_analysis_tests::run_code_analysis_tests;
pub use wallet_integration_tests::run_wallet_integration_tests;
pub use self_analysis_tests::run_self_analysis_tests;
