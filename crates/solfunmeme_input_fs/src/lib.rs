use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use anyhow::Result;
use shared_analysis_types::CodeChunk;

pub fn read_code_chunks(target_path: Option<String>, limit: Option<usize>) -> Result<Vec<CodeChunk>> {
    let mut discovered_files = Vec::new();
    if let Some(ref path) = target_path {
        let path = Path::new(path);
        if path.is_file() {
            discovered_files.push(path.to_path_buf());
        } else if path.is_dir() {
            for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    discovered_files.push(entry.path().to_path_buf());
                }
            }
        } else {
            anyhow::bail!("Path not found: {}", path.display());
        }
    } else {
        for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                discovered_files.push(entry.path().to_path_buf());
            }
        }
    }
    if let Some(lim) = limit {
        discovered_files.truncate(lim);
    }

    let mut code_chunks = Vec::new();
    for path in discovered_files {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if !matches!(ext.as_str(), "rs" | "md") {
            // Only process .rs and .md files for now
            continue;
        }
        match fs::read_to_string(&path) {
            Ok(content) => {
                let chunk = CodeChunk {
                    path: path.to_string_lossy().to_string(),
                    content: content.clone(),
                    emoji: "❓".to_string(), // Placeholder emoji
                    line_start: 1,
                    line_end: content.lines().count() as u32,
                    chunk_type: "file".to_string(),
                };
                code_chunks.push(chunk);
            },
            Err(e) => {
                eprintln!("[ERROR: could not read file: {}]", e);
            }
        }
    }
    Ok(code_chunks)
}