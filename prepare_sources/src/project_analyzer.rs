use anyhow::Result;
use std::path::Path;

use crate::function_analyzer::analyze_rust_file;
use crate::load_emoji_multivectors::load_emoji_multivectors;
use crate::embedding::embed_text;
use crate::clifford::{BertCliffordEncoder, SolMultivector};
use crate::sieve::get_sieve_address;
use crate::clifford::BertConfig;
use crate::function_analyzer::FunctionInfo;

pub struct AnalyzedFunction {
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub file_path: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emoji: String,
    pub emoji_category: String,
    pub emoji_distance: f32,
}

pub fn analyze_project(project_root: &Path, ontology_path: &Path) -> Result<Vec<AnalyzedFunction>> {
    let mut analyzed_functions = Vec::new();
    let emoji_multivectors = load_emoji_multivectors(ontology_path.to_str().unwrap())?;
    let bert_encoder = BertCliffordEncoder::new(BertConfig::default());

    let rust_files = crate::function_analyzer::find_rust_files(project_root);

    for file_path_str in rust_files {
        let file_path = Path::new(&file_path_str);
        let functions_info_in_file = analyze_rust_file(file_path);

        for func_info in functions_info_in_file {
            let embedding = embed_text(&func_info.semantic_summary)?;
            let multivector = bert_encoder.encode_embedding(&embedding)?;
            let sieve_address = get_sieve_address(&multivector);

            let (closest_emoji, emoji_category, emoji_distance) = 
                find_closest_emoji(&multivector, &emoji_multivectors);

            analyzed_functions.push(AnalyzedFunction {
                function_name: func_info.function_name,
                code_snippet: func_info.code_snippet,
                semantic_summary: func_info.semantic_summary,
                file_path: func_info.file_path,
                multivector_str: format!("{:?}", multivector),
                sieve_address,
                closest_emoji,
                emoji_category,
                emoji_distance,
            });
        }
    }

    Ok(analyzed_functions)
}

fn find_closest_emoji(
    _multivector: &SolMultivector,
    _emoji_multivectors: &std::collections::HashMap<String, (SolMultivector, String)>,
) -> (String, String, f32) {
    // Placeholder for actual emoji matching logic
    // This would involve calculating distances between multivectors
    // and finding the closest emoji based on the ontology.
    ("❓".to_string(), "Unknown".to_string(), 0.0)
}