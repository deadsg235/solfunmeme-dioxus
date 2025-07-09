use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use std::fs;

use crate::function_analyzer::{analyze_rust_file, extract_code_snippets};
use shared_analysis_types::CodeSnippet;
use crate::embedding::embed_text;
use crate::clifford::{BertCliffordEncoder, SolMultivector, BertConfig};
use crate::sieve::get_sieve_address;
use shared_analysis_types::{AnalyzedFunction, AnalyzedDocument, ClosestEmojiInfo};
use candle_core::Device;
use tokenizers::Tokenizer;
use crate::load_emoji_multivectors::load_emoji_multivectors;
use orbital_sim_lib::simulate_orbit;
use tclifford::algebra::ClAlgebraBase;


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
    let mut sum_sq_diff: f32 = 0.0;
    for i in 0..crate::clifford::SolCl::dim() {
        sum_sq_diff += (mv1.get_by_idx(i) - mv2.get_by_idx(i)).powi(2);
    }
    sum_sq_diff.sqrt()
}

pub fn calculate_orbital_path(
    multivector: &SolMultivector,
    mass: f64,
) -> Vec<(f64, f64)> {
    let k = 1.0; 
    let t_span = (0.0, 10.0);
    let n_steps = 1000;

    let initial_position = [
        multivector.get_by_idx(0) as f64 * 10.0,
        multivector.get_by_idx(1) as f64 * 10.0,
        multivector.get_by_idx(2) as f64 * 10.0,
        multivector.get_by_idx(3) as f64 * 10.0,
    ];
    let initial_velocity = [
        multivector.get_by_idx(4) as f64 * 0.1,
        multivector.get_by_idx(5) as f64 * 0.1,
        multivector.get_by_idx(6) as f64 * 0.1,
        multivector.get_by_idx(7) as f64 * 0.1,
    ];

    let initial_state = nalgebra::vector![
        initial_position[0],
        initial_position[1],
        initial_position[2],
        initial_position[3],
        initial_velocity[0],
        initial_velocity[1],
        initial_velocity[2],
        initial_velocity[3],
    ];

    simulate_orbit(t_span, n_steps, initial_state, k, mass)
}


#[cfg(feature = "gpu_backend")]
pub fn process_rust_file(
    file_path: &Path,
    emoji_multivectors: &HashMap<String, (SolMultivector, String)>,
    bert_encoder: &BertCliffordEncoder,
    device: &Device,
) -> Result<Vec<AnalyzedFunction>> {
    let functions_info_in_file = analyze_rust_file(file_path);

    let analyzed_functions = functions_info_in_file
        .into_iter()
        .filter_map(|func_info| {
            let embedding = embed_text(&func_info.semantic_summary, device).ok()?;
            let multivector = bert_encoder.encode_embedding(&embedding).ok()?;
            let sieve_address = get_sieve_address(&multivector);
            let closest_emojis = find_closest_emojis(&multivector, emoji_multivectors);
            let orbital_path = Some(calculate_orbital_path(&multivector, func_info.semantic_summary.len() as f64));

            Some(AnalyzedFunction {
                function_name: func_info.function_name,
                code_snippet: func_info.code_snippet,
                semantic_summary: func_info.semantic_summary,
                file_path: func_info.file_path,
                multivector_str: format!("{:?}", multivector),
                sieve_address,
                closest_emojis,
                orbital_path,
            })
        })
        .collect();
    Ok(analyzed_functions)
}

#[cfg(not(feature = "gpu_backend"))]
pub fn process_rust_file(
    _file_path: &Path,
    _emoji_multivectors: &HashMap<String, (Vec<f32>, String)>,
    _bert_encoder: &BertCliffordEncoder,
    _device: &Device,
) -> Result<Vec<AnalyzedFunction>> {
    Ok(Vec::new())
}

#[cfg(feature = "gpu_backend")]
pub fn process_markdown_file(
    file_path: &Path,
    emoji_multivectors: &HashMap<String, (SolMultivector, String)>,
    bert_encoder: &BertCliffordEncoder,
    device: &Device,
) -> Result<AnalyzedDocument> {
    let content = fs::read_to_string(file_path)?;
    let code_snippets = extract_code_snippets(&content);
    let mut analyzed_snippets = Vec::new();
    let mut text_chunks = Vec::new();

    for snippet in &code_snippets {
        let embedding = embed_text(&snippet.content, &device).ok().unwrap_or_default();
        let multivector = bert_encoder.encode_embedding(&embedding).unwrap();
        let sieve_address = get_sieve_address(&multivector);
        let closest_emojis = find_closest_emojis(&multivector, &emoji_multivectors);
        let orbital_path = Some(calculate_orbital_path(&multivector, snippet.content.len() as f64));

        analyzed_snippets.push(AnalyzedFunction {
            function_name: format!("code_snippet_{}", snippet.content_hash),
            code_snippet: snippet.content.clone(),
            semantic_summary: snippet.content.clone(),
            file_path: file_path.to_string_lossy().replace("\\", "/").to_owned(),
            multivector_str: format!("{:?}", multivector),
            sieve_address,
            closest_emojis,
            orbital_path,
        });
    }

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

    Ok(AnalyzedDocument {
        file_path: file_path.to_string_lossy().into_owned(),
        code_snippets,
        text_chunks,
        analyzed_snippets,
    })
}

#[cfg(not(feature = "gpu_backend"))]
pub fn process_markdown_file(
    _file_path: &Path,
    _emoji_multivectors: &HashMap<String, (Vec<f32>, String)>,
    _bert_encoder: &BertCliffordEncoder,
    _device: &Device,
) -> Result<AnalyzedDocument> {
    Ok(AnalyzedDocument {
        file_path: _file_path.to_string_lossy().into_owned(),
        code_snippets: Vec::new(),
        text_chunks: Vec::new(),
        analyzed_snippets: Vec::new(),
    })
}

pub fn process_file_for_tokens(
    file_path: &Path,
    tokenizer: &Tokenizer,
) -> Result<HashMap<String, usize>> {
    let content = fs::read_to_string(file_path)?;
    let mut token_counts: HashMap<String, usize> = HashMap::new();

    if file_path.extension().map_or(false, |ext| ext == "rs") {
        let functions_info_in_file = analyze_rust_file(file_path);
        for func_info in functions_info_in_file {
            for token in tokenizer.encode(&*func_info.semantic_summary, false).map_err(|e| anyhow::anyhow!(e.to_string()))?.get_tokens() {
                *token_counts.entry(token.to_string()).or_insert(0) += 1;
            }
            for token in tokenizer.encode(&*func_info.code_snippet, false).map_err(|e| anyhow::anyhow!(e.to_string()))?.get_tokens() {
                *token_counts.entry(token.to_string()).or_insert(0) += 1;
            }
        }
    } else if file_path.extension().map_or(false, |ext| ext == "md" || ext == "markdown") {
        let code_snippets = extract_code_snippets(&content);
        for snippet in &code_snippets {
            for token in tokenizer.encode(&*snippet.content, false).map_err(|e| anyhow::anyhow!(e.to_string()))?.get_tokens() {
                *token_counts.entry(token.to_string()).or_insert(0) += 1;
            }
        }
        let mut last_end = 0;
        let lines: Vec<&str> = content.lines().collect();
        for snippet in &code_snippets {
            let start = snippet.line_start.saturating_sub(1);
            let end = snippet.line_end;
            if start > last_end {
                let text_chunk = lines[last_end..start].join("\n");                for token in tokenizer.encode(&*text_chunk, false).map_err(|e| anyhow::anyhow!(e.to_string()))?.get_tokens() {                    *token_counts.entry(token.to_string()).or_insert(0) += 1;                }
            }
            last_end = end;
        }
        if last_end < lines.len() {
            let text_chunk = lines[last_end..].join("\n");
            for token in tokenizer.encode(&*text_chunk, false).map_err(|e| anyhow::anyhow!(e.to_string()))?.get_tokens() {
                *token_counts.entry(token.to_string()).or_insert(0) += 1;
            }
        }
    }

    Ok(token_counts)
}
