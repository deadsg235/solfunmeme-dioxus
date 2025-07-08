use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use tclifford::algebra::ClAlgebraBase;
use crate::function_analyzer::analyze_rust_file;
use crate::load_emoji_multivectors::load_emoji_multivectors;
use crate::embedding::embed_text;
use crate::clifford::{BertCliffordEncoder, SolMultivector, SolCl};
use crate::sieve::get_sieve_address;
use crate::clifford::BertConfig;
use crate::function_analyzer::FunctionInfo;

#[derive(Debug, Clone)]
pub struct ClosestEmojiInfo {
    pub emoji: String,
    pub category: String,
    pub distance: f32,
}

pub struct AnalyzedFunction {
    pub function_name: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub file_path: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emojis: Vec<ClosestEmojiInfo>,
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

            let closest_emojis = 
                find_closest_emojis(&multivector, &emoji_multivectors);

            analyzed_functions.push(AnalyzedFunction {
                function_name: func_info.function_name,
                code_snippet: func_info.code_snippet,
                semantic_summary: func_info.semantic_summary,
                file_path: func_info.file_path,
                multivector_str: format!("{:?}", multivector),
                sieve_address,
                closest_emojis,
            });
        }
    }

    Ok(analyzed_functions)
}

fn find_closest_emojis(
    multivector: &SolMultivector,
    emoji_multivectors: &HashMap<String, (SolMultivector, String)>,
) -> Vec<ClosestEmojiInfo> {
    let mut distances: Vec<(f32, String, String)> = Vec::new();

    for (emoji_char, (emoji_mv, category)) in emoji_multivectors {
        let distance = calculate_distance(multivector, emoji_mv);
        distances.push((distance, emoji_char.clone(), category.clone()));
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    distances.into_iter().take(3).map(|(distance, emoji, category)| {
        ClosestEmojiInfo {
            emoji,
            category,
            distance,
        }
    }).collect()
}

fn calculate_distance(mv1: &SolMultivector, mv2: &SolMultivector) -> f32 {
    // Simple Euclidean distance for demonstration. 
    // In a real scenario, you might use a more sophisticated distance metric 
    // appropriate for multivectors or the specific embedding space.
    let mut sum_sq_diff = 0.0;
    for i in 0..SolCl::dim() {
        sum_sq_diff += (mv1.get_by_idx(i) - mv2.get_by_idx(i)).powi(2);
    }
    sum_sq_diff.sqrt()
}
