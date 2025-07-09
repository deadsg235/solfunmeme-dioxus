use std::fs;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("chat_processor: Starting main function.");
    let chat_log_path = "/mnt/c/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/2025/07/08/grok-chat.md";
    let content = fs::read_to_string(chat_log_path)?;

    let part_regex = Regex::new(r"(?s)\[START PART \d+/\d+\](.*?)\[END PART \d+/\d+\]")?;
    let html_tag_regex = Regex::new(r"<[^>]*>")?;

    let mut part_number = 0;
    for cap in part_regex.captures_iter(&content) {
        part_number += 1;
        let raw_part_content = cap.get(1).map_or("", |m| m.as_str());

        // Strip HTML tags from the raw part content
        let cleaned_part_content = html_tag_regex.replace_all(raw_part_content, "").to_string();

        println!("
--- Processing Part {} ---", part_number);
        println!("Cleaned part content length: {}", cleaned_part_content.len());
        process_part(&cleaned_part_content)?;
    }

    Ok(())
}

fn process_part(part_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let turns: Vec<&str> = part_content.split("### ").filter(|s| !s.trim().is_empty()).collect();

    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);

        println!("
Speaker: {}", speaker);
        println!("Content (first 200 chars): \"{}\"", &content.chars().take(200).collect::<String>());

        let code_snippets = extract_markdown_snippets(&content).unwrap();

        if !code_snippets.is_empty() {
            println!("  Markdown Code Snippets Found: {}", code_snippets.len());
            for snippet in code_snippets {
                println!("    - Language: {}, Lines: {}-{}, Hash: {}", snippet.language, snippet.line_start, snippet.line_end, snippet.content_hash);
            }
        }
    }
    Ok(())
}

fn extract_speaker_and_content(turn: &str) -> (&str, String) {
    let content = turn.trim().to_string();
    let speaker = if content.starts_with("User") {
        "User"
    } else if content.starts_with("Grok AI") {
        "Grok AI"
    } else {
        "Unknown"
    };

    (speaker, content)
}
