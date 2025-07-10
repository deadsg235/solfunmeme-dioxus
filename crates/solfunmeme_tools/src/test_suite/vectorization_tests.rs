use solfunmeme_core_logic::core::CodeVectorizer;

pub fn run_vectorization_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔬 Testing Code Vectorization...");

    let vectorizer = CodeVectorizer::new(128);
    let test_codes = vec![
        include_str!("../../tests/fixtures/vectorization_test_code_1.rs.fixture"),
        include_str!("../../tests/fixtures/vectorization_test_code_2.rs.fixture"),
        include_str!("../../tests/fixtures/vectorization_test_code_3.rs.fixture"),
    ];

    let mut vectors = Vec::new();
    for code in &test_codes {
        let vector = vectorizer.vectorize(code);
        assert_eq!(vector.dimensions.len(), 128);

        // Check normalization
        let sum: f32 = vector.dimensions.iter().sum();
        assert!(
            (sum - 1.0).abs() < 1e-5,
            "Vector not normalized: sum = {}",
            sum
        );

        vectors.push(vector);
    }

    // Test similarity
    let similarity = vectors[0].similarity(&vectors[0]);
    assert!(
        (similarity - 1.0).abs() < 1e-6,
        "Self-similarity should be 1.0"
    );

    println!("   ✅ Vectorization tests passed");
    Ok(())
}