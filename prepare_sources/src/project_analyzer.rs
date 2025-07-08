use anyhow::Result;
use walkdir::WalkDir;
use std::path::Path;

use crate::function_analyzer::{analyze_rust_file, FunctionInfo};
use crate::process_file::process_code;
use crate::sieve::get_sieve_address;
use crate::load_emoji_multivectors::load_emoji_multivectors;

pub struct AnalyzedFunction {
    pub file_path: String,
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emoji: String,
    pub emoji_category: String,
    pub emoji_distance: f32,
}

pub fn analyze_project(project_root: &Path, ontology_path: &Path) -> Result<Vec<AnalyzedFunction>> {
    let mut all_analyzed_functions = Vec::new();

    let emoji_multivectors = load_emoji_multivectors(ontology_path.to_str().unwrap())?;

    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
            println!("Analyzing Rust file: {}", path.display());
            match analyze_rust_file(path) {
                Ok(functions) => {
                    for func_info in functions {
                        match process_code(&func_info.semantic_summary) {
                            Ok(func_multivector) => {
                                let sieve_address = get_sieve_address(&func_multivector);

                                let mut closest_emoji: Option<(String, String, f32)> = None;
                                for (emoji_char, (emoji_mv, category)) in emoji_multivectors.iter() {
                                    let distance = (0..8).map(|i| {
                                        let diff = func_multivector.get_by_idx(1 << i) - emoji_mv.get_by_idx(1 << i);
                                        diff * diff
                                    }).sum::<f32>().sqrt();

                                    if closest_emoji.is_none() || distance < closest_emoji.as_ref().unwrap().2 {
                                        closest_emoji = Some((emoji_char.clone(), category.clone(), distance));
                                    }
                                }

                                if let Some((emoji, category, distance)) = closest_emoji {
                                    all_analyzed_functions.push(AnalyzedFunction {
                                        file_path: path.to_string_lossy().to_string(),
                                        function_name: func_info.name,
                                        code_snippet: func_info.code,
                                        semantic_summary: func_info.semantic_summary,
                                        multivector_str: format!("{:?}", func_multivector),
                                        sieve_address,
                                        closest_emoji: emoji,
                                        emoji_category: category,
                                        emoji_distance: distance,
                                    });
                                }
                            }
                            Err(e) => eprintln!("Error processing function {}: {}", func_info.name, e),
                        }
                    }
                }
                Err(e) => eprintln!("Error analyzing file {}: {}", path.display(), e),
            }
        }
    }
    Ok(all_analyzed_functions)
}