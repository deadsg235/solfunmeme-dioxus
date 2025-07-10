use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use crate::Error;

pub fn process_turn(turn_content: &str) -> Result<(String, String), Error> {
    let (speaker, content) = extract_speaker_and_content(turn_content);
    let processed_message = process_message_content(content)?;
    Ok((speaker.to_string(), processed_message))
}

fn extract_speaker_and_content(turn: &str) -> (&str, &str) {
    let trimmed_turn = turn.trim();
    if trimmed_turn.starts_with("User") {
        ("User", trimmed_turn.trim_start_matches("User").trim())
    } else if trimmed_turn.starts_with("Grok AI") {
        ("Grok AI", trimmed_turn.trim_start_matches("Grok AI").trim())
    } else {
        ("Unknown", trimmed_turn)
    }
}

fn process_message_content(content: &str) -> Result<String, Error> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)
        .map_err(|e| Error::MarkdownError(e.to_string()))?;
    
    // Split content into lines
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Skip empty lines at the start
        if processed.is_empty() && line.is_empty() {
            i += 1;
            continue;
        }
        
        // Handle LaTeX blocks
        if line.starts_with("\\documentclass") {
            let mut latex_block = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() {
                latex_block.push(lines[i]);
                i += 1;
            }
            processed.push_str("```latex\n");
            processed.push_str(&latex_block.join("\n"));
            processed.push_str("\n```\n\n");
            continue;
        }
        
        // Handle regular text
        if !line.is_empty() {
            processed.push_str(line);
            processed.push_str("\n");
        } else if !processed.ends_with("\n\n") {
            processed.push_str("\n");
        }
        
        i += 1;
    }
    
    // Append code snippets if found
    if !code_snippets.is_empty() {
        processed.push_str("\n### Code Snippets\n\n");
        for snippet in code_snippets {
            processed.push_str(&format!("```{}\n{}\n```\n\n", 
                snippet.language, snippet.content));
        }
    }
    
    Ok(processed)
}