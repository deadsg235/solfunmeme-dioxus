use regex::Regex;
use solfunmeme_extractor::model::clean_html::clean_html;

pub fn process_content(content: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cleaned_content = clean_html(content);
    
    // Split into conversation turns
    let turns: Vec<String> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(turns)
}
