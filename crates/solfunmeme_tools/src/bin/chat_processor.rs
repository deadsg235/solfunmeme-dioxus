use solfunmeme_tools::chat_processing::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ChatProcessorArgs::from_env()?;
    
    let chat_files = find_chat_files(&args.target_path, args.limit);
    println!("[INFO] Processing {} chat files:", chat_files.len());

    for chat_file in chat_files {
        println!("[INFO] Processing file: {}", chat_file.display());
        
        let processed_content = process_content(&chat_file)?;
        let output_path = save_processed_content(&processed_content, &chat_file, &args.output_dir)?;
        
        println!("[INFO] Saved processed file to: {}", output_path.display());
    }

    Ok(())


fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat = String::new();
    let mut current_speaker = String::new();
    
    for turn in turns {
        let (speaker, content) = extract_speaker_and_content(turn);
        
        // Add speaker header
        if speaker != current_speaker {
            processed_chat.push_str(&format!("\n## {speaker}\n\n"));
            current_speaker = speaker.to_string();
        }
        
        // Process message content
        let message = process_message_content(content)?;
        processed_chat.push_str(&message);
        processed_chat.push_str("\n\n");
    }
    
    // Write processed chat to new file
    let output_path = path.with_file_name(
        format!("{}_processed.md", 
            path.file_stem().unwrap().to_string_lossy())
    );
    fs::write(&output_path, processed_chat)?;
    
    println!("Processed chat saved to: {}", output_path.display());
    Ok(())
}

fn process_message_content(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut processed = String::new();
    
    // Extract code snippets
    let code_snippets = extract_markdown_snippets(content)?;
    
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
use std::fs;
use std::path::Path;
use regex::Regex;
use solfunmeme_extractor::model::snippets::extract_markdown_snippets;
use solfunmeme_extractor::model::clean_html::clean_html;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chat_dir = Path::new("c:/Users/gentd/OneDrive/Documents/GitHub/solfunmeme-dioxus/founding_documents/internal/solfunmeme/july/10");
    process_chat_directory(chat_dir)?;
    Ok(())
}

fn process_chat_directory(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") 
           && path.file_name().unwrap().to_string_lossy().contains("grok-chat") {
            println!("Processing chat file: {}", path.display());
            process_chat_file(&path)?;
        }
    }
    Ok(())
}

fn process_chat_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let cleaned_content = clean_html(&content);
    
    // Split into conversation turns
    let turns: Vec<&str> = cleaned_content
        .split("### ")
        .filter(|s| !s.trim().is_empty())
        .collect();

    let mut processed_chat
