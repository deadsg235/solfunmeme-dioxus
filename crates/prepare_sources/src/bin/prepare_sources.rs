use serde_json;
use serde::{Deserialize, Serialize};
use solfunmeme_input_fs::read_code_chunks;

use solfunmeme_function_analysis::data_models::CodeChunk;

fn main() {
    use std::env;
    let mut args = env::args().skip(1);
    let mut target_path: Option<String> = None;
    let mut limit: Option<usize> = None;
    
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--limit" => {
                if let Some(lim) = args.next() {
                    limit = lim.parse().ok();
                }
            }
            "--help" | "-h" => {
                println!("Usage: prepare_sources [OPTIONS] [PATH]");
                println!();
                println!("Options:");
                println!("  --limit N           Limit processing to N files");
                println!("  --help, -h          Show this help message");
                println!();
                println!("Examples:");
                println!("  prepare_sources .                    # Process current directory");
                println!("  prepare_sources --limit 100 .        # Process first 100 files");
                return;
            }
            _ => {
                if target_path.is_none() {
                    target_path = Some(arg);
                }
            }
        }
    }

    let code_chunks = match read_code_chunks(target_path, limit) {
        Ok(chunks) => chunks,
        Err(e) => {
            eprintln!("[ERROR] Failed to read code chunks: {}", e);
            return;
        }
    };

    println!("[INFO] Processing {} files:", code_chunks.len());

    for chunk in code_chunks {
        println!("{}", serde_json::to_string(&chunk).unwrap());
    }
}