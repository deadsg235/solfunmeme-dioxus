use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    let mut args = env::args().skip(1);
    let mut target_path: Option<String> = None;
    let mut limit: Option<usize> = None;
    let mut output_dir: Option<String> = None;
    
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--limit" => {
                if let Some(lim) = args.next() {
                    limit = lim.parse().ok();
                }
            },
            "--output" | "-o" => {
                output_dir = args.next();
            },
            "--help" | "-h" => {
                println!("Usage: chat_processor [OPTIONS] [PATH]");
                println!();
                println!("Options:");
                println!("  --limit N           Limit processing to N files");
                println!("  --output, -o DIR    Output directory for processed files");
                println!("  --help, -h          Show this help message");
                println!();
                println!("Examples:");
                println!("  chat_processor .                          # Process current directory");
                println!("  chat_processor --limit 100 .              # Process first 100 files");
                println!("  chat_processor -o processed_chats .       # Save to output directory");
                return Ok(());
            },
            _ => {
                if target_path.is_none() {
                    target_path = Some(arg);
                }
            }
        }
    }

    let target_path = target_path.unwrap_or_else(|| ".".to_string());
    let output_dir = output_dir.unwrap_or_else(|| "processed_chats".to_string());
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Find all markdown files in the target directory
    let mut chat_files: Vec<PathBuf> = WalkDir::new(&target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && 
            e.path().extension().map_or(false, |ext| ext == "md") &&
            e.path().file_name().map_or(false, |name| name.to_string_lossy().contains("grok-chat"))
        })
        .map(|e| e.path().to_owned())
        .collect();

    if let Some(limit) = limit {
        chat_files.truncate(limit);
    }

    println!("[INFO] Processing {} chat files:", chat_files.len());

    for chat_file in chat_files {
        process_chat_file(&chat_file, &output_dir)?;
    }

    Ok(())
}

fn process_chat_file(input_path: &Path, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[INFO] Processing file: {}", input_path.display());
    let content = fs::read_to_string(input_path)?;

    // Remove HTML tags
    let html_tag_regex = Regex::new(r"<[^>]*>")?;
    let cleaned_content = html_tag_regex.replace_all(&content, "");

    // Split into turns and process
    let turns: Vec<&str> = cleaned_content.split("### ").filter(|s| !s.trim().is_empty()).collect();
    let mut processed_content = String::new();

    for turn_content in turns {
        let (speaker, content) = extract_speaker_and_content(turn_content);
        processed_content.push_str("### ");
        processed_content.push_str(speaker);
        processed_content.push_str("\n\n");
        
        // Add the message content, skipping the speaker line
        let message_content = content.lines().skip(1).collect::<Vec<_>>().join("\n");
        processed_content.push_str(&message_content);
        processed_content.push_str("\n\n");
    }

    // Create output file path
    let file_name = input_path.file_name().unwrap().to_string_lossy();
    let output_name = format!("{}_processed.md", file_name.trim_end_matches(".md"));
    let output_path = Path::new(output_dir).join(output_name);

    // Write processed content
    fs::write(&output_path, processed_content)?;
    println!("[INFO] Saved processed file to: {}", output_path.display());

    Ok(())
}

fn extract_speaker_and_content(turn: &str) -> (&str, &str) {
    let trimmed_turn = turn.trim();
    if trimmed_turn.starts_with("User") {
        ("User", trimmed_turn)
    } else if trimmed_turn.starts_with("Grok AI") {
        ("Grok AI", trimmed_turn)
    } else {
        ("Unknown", trimmed_turn)
    }
}