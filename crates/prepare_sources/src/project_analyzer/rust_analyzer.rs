use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;
use solfunmeme_function_analysis::{AnalyzedFunction, analyze_rust_file};
use indicatif::ProgressBar;

pub fn analyze_project(project_root: &Path, _ontology_path: &Path, _use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedFunction>> {
    let mut analyzed_functions = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
            if ext == "rs" {
                let functions_info_in_file = analyze_rust_file(file_path);
                for func_info in functions_info_in_file {
                    analyzed_functions.push(func_info);
                }
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_functions)
}
