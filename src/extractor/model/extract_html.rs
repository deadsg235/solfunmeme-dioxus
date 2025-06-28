use scraper::{Html, Selector};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::extractor::types::{CodeSnippet, TestResult};

/// Extract code snippets from HTML content containing markdown code blocks
pub fn extract_code_snippets_from_html(content: &str) -> Vec<CodeSnippet> {
    let mut snippets = Vec::new();
    let document = Html::parse_fragment(content);

    // Select all markdown code blocks
    let code_block_selector = Selector::parse("div[data-testid='markdown-code-block']").unwrap();
    
    for (block_index, code_block) in document.select(&code_block_selector).enumerate() {
        // Get language (from the span containing the language identifier)
        let lang_selector = Selector::parse("div > div > span").unwrap();
        //let language = code_block
        //.select(&lang_selector)
        //.map(|span| span.inner_html().trim().to_string())
        //.unwrap_or_else(|| "text".to_string());
	let language = code_block
	    .select(&lang_selector)
	    .next() // Get the first matching element
	    .map(|span| span.inner_html().trim().to_string()) // Map to the desired string
	    .unwrap_or_else(|| "text".to_string()); // Provide default if None
	
        // Get code content (from pre > code)
        let code_selector = Selector::parse("pre > code").unwrap();
        if let codec  = code_block.select(&code_selector) {
            // Collect text, removing any HTML artifacts
            //let code_content = codec.text().collect::<Vec<_>>().join("");

	    let code_content = codec
		.flat_map(|element| element.text()) // Get text from each ElementRef
		.collect::<Vec<_>>()
		.join("");
	    
            if !code_content.trim().is_empty() {
                // Approximate line numbers (since HTML doesn't preserve them)
                let line_count = code_content.lines().count();
                let line_start = block_index * 100 + 1; // Arbitrary offset to avoid overlap
                let line_end = line_start + line_count - 1;

                let snippet = create_code_snippet(
                    language,
                    code_content,
                    line_start,
                    line_end,
                );
                snippets.push(snippet);
            }
        }
    }

    snippets
}

/// Create a complete CodeSnippet with all fields populated (reusing your existing function)
fn create_code_snippet(
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

/// Generate a simple hash for content deduplication (reusing your existing function)
fn generate_content_hash(content: &str) -> String {
    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Estimate token count (reusing your existing function)
fn estimate_token_count(content: &str) -> usize {
    content
        .split_whitespace()
        .map(|word| {
            let punctuation_count = word.chars()
                .filter(|c| c.is_ascii_punctuation())
                .count();
            1 + punctuation_count
        })
        .sum()
}

/// Create a default test result (reusing your existing function)
fn create_default_test_result() -> TestResult {
    TestResult {
        passed: false,
        error_message: None,
        execution_time: None,
        output: None,
    }
}
