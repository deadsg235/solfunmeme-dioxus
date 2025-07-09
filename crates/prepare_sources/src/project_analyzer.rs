use anyhow::Result;
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use std::collections::HashMap;
use shared_analysis_types::CodeChunk;
use serde::{Deserialize, Serialize};



pub fn analyze_project(project_root: &Path) -> Result<Vec<CodeChunk>> {
    let mut code_chunks = Vec::new();

    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

            // Only process text/code files for now
            if matches!(ext.as_str(), "rs" | "md" | "json" | "ttl" | "toml" | "txt" | "js" | "ts" | "tsx" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp") {
                match fs::read_to_string(file_path) {
                    Ok(content) => {
                        // Simple chunking for now
                        let lines: Vec<&str> = content.lines().collect();
                        let chunk_size = 50; // Lines per chunk

                        for (i, chunk_lines) in lines.chunks(chunk_size).enumerate() {
                            let start_line = i * chunk_size + 1;
                            let end_line = start_line + chunk_lines.len() - 1;

                            let chunk = CodeChunk {
                                path: file_path.to_string_lossy().to_string(),
                                content: chunk_lines.join("\n"),
                                emoji: "📄".to_string(), // Generic document emoji
                                line_start: start_line as u32,
                                line_end: end_line as u32,
                                chunk_type: "code".to_string(),
                            };
                            code_chunks.push(chunk);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading file {}: {}", file_path.display(), e);
                    }
                }
            }
        }
    }
    Ok(code_chunks)
}