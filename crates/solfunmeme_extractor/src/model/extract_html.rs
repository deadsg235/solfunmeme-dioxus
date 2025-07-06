use crate::types::{CodeSnippet};
use scraper::{Html, Selector};
use crate::model::extract::create_code_snippet::create_code_snippet;


/// Extract code snippets from HTML content containing markdown code blocks
pub fn extract_code_snippets_from_html(content: &str) -> Vec<CodeSnippet> {
    let mut snippets = Vec::new();
    let document = Html::parse_fragment(content);

    // Select all markdown code blocks
    let code_block_selector = Selector::parse("div[data-testid='markdown-code-block']").unwrap();

    for (block_index, code_block) in document.select(&code_block_selector).enumerate() {
        // Get language (from the span containing the language identifier)
        let lang_selector = Selector::parse("div > div > span").unwrap();
        let language = code_block
            .select(&lang_selector)
            .next() // Get the first matching element
            .map(|span| span.inner_html().trim().to_string()) // Map to the desired string
            .unwrap_or_else(|| "text".to_string()); // Provide default if None

        // Get code content (from pre > code)
        let code_selector = Selector::parse("pre > code").unwrap();
        let code_content = code_block.select(&code_selector)
            .flat_map(|element| element.text()) // Get text from each ElementRef
            .collect::<Vec<_>>()
            .join("");

        if !code_content.trim().is_empty() {
            // Approximate line numbers (since HTML doesn't preserve them)
            let line_count = code_content.lines().count();
            let line_start = block_index * 100 + 1; // Arbitrary offset to avoid overlap
            let line_end = line_start + line_count - 1;

            let snippet = create_code_snippet(language, code_content, line_start, line_end);
            snippets.push(snippet);
        }
    }

    snippets
}
