use regex::Regex;
use std::fs;
use std::path::Path;

pub fn process_content(input_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(input_path)?;
    let html_tag_regex = Regex::new(r"<[^>]*>")?;
    let cleaned_content = html_tag_regex.replace_all(&content, "");
    
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let processed_content = turns
        .into_iter()
        .map(super::turn_processor::process_turn)
        .collect::<Vec<_>>()
        .join("\n\n");

    Ok(processed_content)
}