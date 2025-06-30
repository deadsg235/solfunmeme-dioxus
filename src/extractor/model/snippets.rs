//content_hash.rs
use crate::extractor::model::extract::estimate_token_count;
use markdown::{to_mdast, ParseOptions};
use regex::Regex;

use crate::extractor::model::content_hash::create_content_hash;
//use crate::extractor::model::token_count::estimate_token_count;
use crate::extractor::model::walk_ast::walk_ast;
use crate::extractor::types::CodeSnippet;

    /// Extracts code snippets from markdown AST using the markdown crate.  


/// Extracts code snippets from text using regex patterns.
pub fn extract_code_snippets_old(text: &str) -> Vec<CodeSnippet> {
    let mut snippets = Vec::new();
    let excluded_ranges = Vec::new();

    // Pattern for code blocks with language specification
    let code_block_re = Regex::new(r"```(\w+)?\s*\n(.*?)\n```").unwrap();

    // Extract code blocks first
    for cap in code_block_re.captures_iter(text) {
        let full_match = cap.get(0).unwrap();
        let language = cap.get(1).map(|m| m.as_str()).unwrap_or("text").to_string();
        let content = cap.get(2).unwrap().as_str().trim();

        if !content.is_empty() {
            let content_hash = create_content_hash(content);
            let token_count = estimate_token_count(content);
            let line_count = content.lines().count();
            let char_count = content.chars().count();

            snippets.push(CodeSnippet {
                content: content.to_string(),
                content_hash,
                language,
                token_count,
                line_count,
                char_count,
                test_result: None,
                line_start: todo!(),
                line_end: todo!(),
            });

            // Mark the range of the code block as excluded
            excluded_ranges.push(full_match.start()..full_match.end());
        }
    }

    // Pattern for inline code snippets
    let inline_code_re = Regex::new(r"`([^`]+)`").unwrap();

    // Extract inline code snippets, skipping excluded ranges
    for cap in inline_code_re.captures_iter(text) {
        let full_match = cap.get(0).unwrap();
        let content = cap.get(1).unwrap().as_str();

        // Skip if the match is within an excluded (code block) range
        if excluded_ranges
            .iter()
            .any(|range| range.contains(&full_match.start()))
        {
            continue;
        }

        if content.len() > 10 {
            // Only consider substantial inline code
            let content_hash = create_content_hash(content);
            let token_count = estimate_token_count(content);
            let line_count = content.lines().count();
            let char_count = content.chars().count();

            snippets.push(CodeSnippet {
                content: content.to_string(),
                content_hash,
                language: "inline".to_string(),
                token_count,
                line_count,
                char_count,
                test_result: None,
                line_end: 0,
                line_start: 0,
            });
        }
    }

    snippets
}
/// Extracts code snippets from text using regex patterns.
pub fn extract_code_snippets2(text: &str) -> Vec<CodeSnippet> {
    let mut snippets = Vec::new();
    
    // Pattern for code blocks with language specification
    let code_block_re = Regex::new(r"```(\w+)?\s*\n(.*?)\n```").unwrap();
    
    for cap in code_block_re.captures_iter(text) {
        let language = cap.get(1).map(|m| m.as_str()).unwrap_or("text").to_string();
        let content = cap.get(2).unwrap().as_str().trim();
        
        if !content.is_empty() {
            let content_hash = create_content_hash(content);
            let token_count = estimate_token_count(content);
            let line_count = content.lines().count();
            let char_count = content.chars().count();
            
            snippets.push(CodeSnippet {
                content: content.to_string(),
                content_hash,
                language,
                token_count,
                line_count,
                char_count,
                test_result: None,
                line_start: 0,
                line_end: 0,
            });
        }
    }
    
    // Also look for inline code snippets
    let inline_code_re = Regex::new(r"`([^`]+)`").unwrap();
    for cap in inline_code_re.captures_iter(text) {
        let content = cap.get(1).unwrap().as_str();
        if content.len() > 10 { // Only consider substantial inline code
            let content_hash = create_content_hash(content);
            let token_count = estimate_token_count(content);
            let line_count = content.lines().count();
            let char_count = 0;
            snippets.push(CodeSnippet {
                content: content.to_string(),
                content_hash,
                language: "inline".to_string(),
                token_count,
                line_count,
                char_count,
                test_result: None,
                line_start: 0,
                line_end: 0,
                

            });
        }
    }
    
    snippets
}


// pub fn extract_code_snippets(text: &str) -> Result<Vec<CodeSnippet>, markdown::message::Message> {  
//     let mut snippets = Vec::new();  
      
//     // Parse markdown to AST  
//     let ast = to_mdast(text, &ParseOptions::default())?;  
      
//     // Walk the AST to find code nodes  
//     walk_ast(&ast, &mut snippets);  
      
//     Ok(snippets)  
// }  

pub fn extract_code_snippets(text: &str) -> Result<Vec<CodeSnippet>, markdown::message::Message> {
    let mut snippets = Vec::new();
    let ast = to_mdast(text, &ParseOptions::default())?;
    walk_ast(&ast, &mut snippets);
    Ok(snippets)
}

// pub fn extract_code_snippets_old(content: &str) -> Vec<CodeSnippet> {
//         let mut snippets = Vec::new();
//         let lines: Vec<&str> = content.lines().collect();
//         let mut in_code_block = false;
//         let mut current_language = String::new();
//         let mut current_content = String::new();
//         let mut start_line = 0;

//         for (i, line) in lines.iter().enumerate() {
//             if line.starts_with("```") {
//                 if in_code_block {
//                     // End of code block
//                     if !current_content.trim().is_empty() {
//                         snippets.push(CodeSnippet {
//                             language: current_language.clone(),
//                             content: current_content.clone(),
//                             line_start: start_line,
//                             line_end: i,
//                         });
//                     }
//                     current_content.clear();
//                     in_code_block = false;
//                 } else {
//                     // Start of code block
//                     current_language = line[3..].trim().to_string();
//                     if current_language.is_empty() {
//                         current_language = "text".to_string();
//                     }
//                     current_content.clear();
//                     start_line = i + 1;
//                     in_code_block = true;
//                 }
//             } else if in_code_block {
//                 if !current_content.is_empty() {
//                     current_content.push('\n');
//                 }
//                 current_content.push_str(line);
//             }
//         }

//         // Handle unclosed code block
//         if in_code_block && !current_content.trim().is_empty() {
//             snippets.push(CodeSnippet {
//                 language: current_language,
//                 content: current_content,
//                 line_start: start_line,
//                 line_end: lines.len(),
//             });
//         }

//         snippets
//     }
