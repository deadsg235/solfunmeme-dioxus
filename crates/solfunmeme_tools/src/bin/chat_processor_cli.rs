use solfunmeme_tools::chat_processing::*;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ChatProcessorArgs::from_env()?;
    
    let chat_files = find_chat_files(&args.target_path, args.limit);
    println!("[INFO] Processing {} chat files:", chat_files.len());

    for chat_file in chat_files {
        println!("[INFO] Processing file: {}", chat_file.display());
        
        let content = std::fs::read_to_string(&chat_file)?;
        let turns = process_content(&content)?;

        let mut processed_chat = String::new();
        let mut current_speaker = String::new();
        
        for turn_content in turns {
            let (speaker, message) = process_turn(&turn_content)?;
            
            if speaker != current_speaker {
                processed_chat.push_str(&format!("\n## {}\n\n", speaker));
                current_speaker = speaker.to_string();
            }
            processed_chat.push_str(&message);
            processed_chat.push_str("\n\n");
        }

        let output_path = save_processed_content(&processed_chat, &chat_file, &args.output_dir)?;
        
        println!("[INFO] Saved processed file to: {}", output_path.display());
    }

    Ok(())
}
