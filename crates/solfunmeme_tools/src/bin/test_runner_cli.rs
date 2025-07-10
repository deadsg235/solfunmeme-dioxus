use crate::test_suite::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = TestSuiteArgs::from_env()?;

    println!("🚀 Solfunmeme Test Runner - Comprehensive Coverage Tests");

    match args.category {
        TestCategory::All => {
            run_vectorization_tests()?;
            run_declaration_splitting_tests()?;
            run_duplicate_detection_tests()?;
            run_code_analysis_tests()?;
            run_wallet_integration_tests()?;
            run_self_analysis_tests()?;
        },
        TestCategory::Vectorization => {
            run_vectorization_tests()?;
        },
        TestCategory::DeclarationSplitting => {
            run_declaration_splitting_tests()?;
        },
        TestCategory::DuplicateDetection => {
            run_duplicate_detection_tests()?;
        },
        TestCategory::CodeAnalysis => {
            run_code_analysis_tests()?;
        },
        TestCategory::WalletIntegration => {
            run_wallet_integration_tests()?;
        },
        TestCategory::SelfAnalysis => {
            run_self_analysis_tests()?;
        },
    }

    println!("\n✅ All selected tests completed successfully!");
    Ok(())
}
