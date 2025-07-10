use std::fs;
use std::path::{Path, PathBuf};

pub fn save_processed_content(
    content: &str,
    input_path: &Path,
    output_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;
    
    let file_name = input_path.file_name().unwrap().to_string_lossy();
    let output_name = format!("{}_processed.md", file_name.trim_end_matches(".md"));
    let output_path = output_dir.join(output_name);
    
    fs::write(&output_path, content)?;
    Ok(output_path)
}