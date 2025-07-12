use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "plan_cli")]
#[command(about = "Estimates indexing cost for specified directories.", long_about = None)]
struct Cli {
    #[arg(help = "Paths to directories to analyze")]
    directories: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        println!("Usage: plan_cli <directory1> [directory2 ...]");
        println!("Example: plan_cli crates/ vendor/");
        return Ok(());
    }

    let mut total_files = 0;
    let mut total_lines = 0;
    let mut total_chunks_estimated = 0;
    let chunk_size_lines = 50; // Based on prepare_sources chunking

    println!("\n--- Indexing Cost Estimation ---");

    for dir_path in cli.directories {
        println!("Analyzing directory: {}", dir_path.display());
        for entry in WalkDir::new(&dir_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

                // Only process text/code files for estimation
                if matches!(ext.as_str(), "rs" | "md" | "json" | "ttl" | "toml" | "txt" | "js" | "ts" | "tsx" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp") {
                    total_files += 1;
                    match fs::read(&file_path) {
                        Ok(bytes) => {
                            let content = String::from_utf8_lossy(&bytes);
                            let lines_in_file = content.lines().count();
                            total_lines += lines_in_file;
                            total_chunks_estimated += (lines_in_file as f64 / chunk_size_lines as f64).ceil() as usize;
                        },
                        Err(e) => {
                            eprintln!("Warning: Could not read file {} ({}). Skipping.", file_path.display(), e);
                        }
                    }
                }
            }
        }
    }

    println!("\n--- Estimation Summary ---");
    println!("Total files to process: {}", total_files);
    println!("Total lines to process: {}", total_lines);
    println!("Estimated total chunks: {}", total_chunks_estimated);
    println!("\nNote: This is an estimation. Actual indexing time may vary based on content complexity and system performance.");

    Ok(())
}