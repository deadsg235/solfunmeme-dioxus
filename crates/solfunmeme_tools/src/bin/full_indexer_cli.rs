use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use solfunmeme_indexer::index_directory;

#[derive(Parser)]
#[command(name = "full_indexer")]
#[command(about = "Indexes specified directories into the Tantivy search index.", long_about = None)]
struct Cli {
    #[arg(help = "Paths to directories to index")]
    directories: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.directories.is_empty() {
        println!("Usage: full_indexer <directory1> [directory2 ...]");
        println!("Example: full_indexer crates/ vendor/");
        return Ok(());
    }

    let index_path = PathBuf::from("codebase_index");

    for dir_path in cli.directories {
        println!("Indexing directory: {}", dir_path.display());
        index_directory(&dir_path, &index_path)?;
        println!("Finished indexing: {}", dir_path.display());
    }

    println!("Indexing complete.");
    Ok(())
}
