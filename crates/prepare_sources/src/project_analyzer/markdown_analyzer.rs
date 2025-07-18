use anyhow::Result;
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use solfunmeme_function_analysis::{AnalyzedDocument, extract_code_snippets};
use indicatif::ProgressBar;

pub fn analyze_markdown_files(project_root: &Path, _ontology_path: &Path, _use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedDocument>> {
    let mut analyzed_documents = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            if ext == "md" || ext == "markdown" {
                let content = fs::read_to_string(file_path)?;
                let code_snippets = extract_code_snippets(&content);
                let mut text_chunks = Vec::new();
                let mut last_end = 0;
                let lines: Vec<&str> = content.lines().collect();
                for snippet in &code_snippets {
                    let start = snippet.line_start.saturating_sub(1);
                    let end = snippet.line_end;
                    if start > last_end {
                        text_chunks.push(lines[last_end..start].join("\n"));
                    }
                    last_end = end;
                }
                if last_end < lines.len() {
                    text_chunks.push(lines[last_end..].join("\n"));
                }
                analyzed_documents.push(AnalyzedDocument {
                    file_path: file_path.to_string_lossy().into_owned(),
                    code_snippets: code_snippets.clone(),
                    text_chunks,
                    analyzed_snippets: code_snippets,
                });
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_documents)
}
