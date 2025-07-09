use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use crate::process_file::{process_rust_file, process_markdown_file, process_file_for_tokens};
use crate::load_emoji_multivectors::load_emoji_multivectors;
use crate::clifford::{BertCliffordEncoder, SolMultivector, BertConfig};
use rayon::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use candle_core::Device;
use walkdir::WalkDir;
use tokenizers::Tokenizer;
use indicatif::ProgressBar;
use tracing::instrument;

use shared_analysis_types::{AnalyzedFunction, AnalyzedDocument, AnalyzedToken, ClosestEmojiInfo, CodeSnippet};

#[derive(Debug, Clone)]
pub struct CodeChunk {
    pub path: String,
    pub content: String,
    pub emoji: String,
    pub line_start: u32,
    pub line_end: u32,
    pub chunk_type: String,
}

#[instrument(skip(pb, ontology_path))]
pub fn analyze_project(project_root: &Path, ontology_path: &Path, use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedFunction>> {
    let emoji_multivectors = load_emoji_multivectors(ontology_path.to_str().unwrap())?;
    let bert_encoder = BertCliffordEncoder::new(BertConfig::default());

        let device = if use_gpu && Device::cuda_if_available(0).is_ok() {
        Device::cuda_if_available(0)?
    } else {
        Device::Cpu
    };

    let rust_files = crate::function_analyzer::find_rust_files(project_root);
    let analyzed_functions: Vec<AnalyzedFunction> = rust_files.par_iter().flat_map(|file_path_str| {
        pb.set_message(format!("Analyzing Rust file: {}", file_path_str));
        let file_path = Path::new(&file_path_str);
        process_rust_file(file_path, &emoji_multivectors, &bert_encoder, &device).unwrap_or_default()
    }).collect();

    Ok(analyzed_functions)
}

#[instrument(skip(pb, ontology_path))]
pub fn analyze_markdown_files(project_root: &Path, ontology_path: &Path, use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedDocument>> {
    let emoji_multivectors = load_emoji_multivectors(ontology_path.to_str().unwrap())?;
    let bert_encoder = BertCliffordEncoder::new(BertConfig::default());

        let device = if use_gpu && Device::cuda_if_available(0).is_ok() {
        Device::cuda_if_available(0)?
    } else {
        Device::Cpu
    };

    let mut analyzed_documents = Vec::new();

    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "md" || extension == "markdown" {
                    let file_path = entry.path();
                    pb.set_message(format!("Analyzing Markdown file: {}", file_path.display()));
                    let analyzed_document = process_markdown_file(file_path, &emoji_multivectors, &bert_encoder, &device)?;
                    analyzed_documents.push(analyzed_document);
                }
            }
        }
    }

    Ok(analyzed_documents)
}

#[instrument(skip(pb, ontology_path))]
pub fn analyze_project_tokens(project_root: &Path, ontology_path: &Path, use_gpu: bool, pb: &ProgressBar) -> Result<HashMap<String, AnalyzedToken>> {
    let mut token_counts: HashMap<String, usize> = HashMap::new();
    let bert_encoder = BertCliffordEncoder::new(BertConfig::default());

        let device = if use_gpu && Device::cuda_if_available(0).is_ok() {
        Device::cuda_if_available(0)?
    } else {
        Device::Cpu
    };

    let tokenizer = Tokenizer::from_file(
        Path::new("../vendor/candle/candle-transformers/tokenizer.json").to_str().unwrap()
    ).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            pb.set_message(format!("Analyzing file for tokens: {}", entry.path().display()));
            let file_token_counts = process_file_for_tokens(entry.path(), &tokenizer)?;
            for (token, count) in file_token_counts {
                *token_counts.entry(token).or_insert(0) += count;
            }
        }
    }

    let mut analyzed_tokens = HashMap::new();
    for (token, count) in token_counts {
        let embedding = crate::embedding::embed_text(&token, &device).ok().unwrap_or_default();
        let multivector = bert_encoder.encode_embedding(&embedding).unwrap();
        let orbital_path = Some(crate::process_file::calculate_orbital_path(&multivector, token.len() as f64));
        analyzed_tokens.insert(token.clone(), AnalyzedToken {
            token,
            count,
            multivector_str: format!("{:?}", multivector),
            orbital_path,
        });
    }

    Ok(analyzed_tokens)
}
