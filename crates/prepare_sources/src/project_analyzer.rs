use anyhow::Result;
use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use solfunmeme_function_analysis::{AnalyzedFunction, AnalyzedDocument, AnalyzedToken, ClosestEmojiInfo, analyze_rust_file, extract_code_snippets};
use indicatif::ProgressBar;
use std::collections::HashMap;

#[cfg(feature = "with-embedding")]
use solfunmeme_embedding::{embed_text, load_emoji_multivectors, find_closest_emojis};
#[cfg(feature = "with-embedding")]
use solfunmeme_clifford::{BertCliffordEncoder, BertConfig, get_sieve_address};
#[cfg(feature = "with-embedding")]
use candle_core::Device;
#[cfg(feature = "with-embedding")]
use tokenizers::Tokenizer;

use solfunmeme_clifford::{SolMultivector, SolCl};

pub fn analyze_project(project_root: &Path, ontology_path: &Path, use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedFunction>> {
    let mut analyzed_functions = Vec::new();

    #[cfg(feature = "with-embedding")]
    let (bert_encoder, emoji_multivectors, device) = {
        let config = BertConfig::default();
        let encoder = BertCliffordEncoder::new(config);
        let emoji_mvs = solfunmeme_embedding::load_emoji_multivectors(ontology_path.to_str().unwrap())?;
        let device = if use_gpu { Device::cuda_if_available(0).unwrap() } else { Device::Cpu };
        (encoder, emoji_mvs, device)
    };

    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

            if ext == "rs" {
                let functions_info_in_file = analyze_rust_file(file_path);

                for mut func_info in functions_info_in_file {
                    #[cfg(feature = "with-embedding")]
                    {
                        let embedding = embed_text(&func_info.semantic_summary, &device)?;
                        let multivector = bert_encoder.encode_embedding(&embedding)?;
                        let sieve_address = get_sieve_address(&multivector);
                        let closest_emojis = find_closest_emojis(&multivector, &emoji_multivectors);

                        func_info.multivector_str = format!("{:?}", multivector);
                        func_info.sieve_address = sieve_address;
                        func_info.closest_emojis = closest_emojis;
                    }
                    analyzed_functions.push(func_info);
                }
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_functions)
}

pub fn analyze_markdown_files(project_root: &Path, ontology_path: &Path, use_gpu: bool, pb: &ProgressBar) -> Result<Vec<AnalyzedDocument>> {
    let mut analyzed_documents = Vec::new();

    #[cfg(feature = "with-embedding")]
    let (bert_encoder, emoji_multivectors, device) = {
        let config = BertConfig::default();
        let encoder = BertCliffordEncoder::new(config);
        let emoji_mvs = solfunmeme_embedding::load_emoji_multivectors(ontology_path.to_str().unwrap())?;
        let device = if use_gpu { Device::cuda_if_available(0).unwrap() } else { Device::Cpu };
        (encoder, emoji_mvs, device)
    };

    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

            if ext == "md" || ext == "markdown" {
                let content = fs::read_to_string(file_path)?;
                let code_snippets = extract_code_snippets(&content);
                let mut analyzed_snippets = Vec::new();
                let mut text_chunks = Vec::new();

                for mut snippet in code_snippets.clone() {
                    #[cfg(feature = "with-embedding")]
                    {
                        let embedding = embed_text(&snippet.content, &device).ok().unwrap_or_default();
                        let multivector = bert_encoder.encode_embedding(&embedding).unwrap();
                        snippet.multivector_str = format!("{:?}", multivector);
                        snippet.sieve_address = get_sieve_address(&multivector);
                        snippet.closest_emojis = find_closest_emojis(&multivector, &emoji_multivectors);
                    }
                    analyzed_snippets.push(AnalyzedFunction {
                        function_name: format!("code_snippet_{}", snippet.content_hash),
                        code_snippet: snippet.content.clone(),
                        semantic_summary: snippet.content.clone(),
                        file_path: file_path.to_string_lossy().replace("\\", "/").to_owned(),
                        multivector_str: snippet.multivector_str,
                        sieve_address: snippet.sieve_address,
                        closest_emojis: snippet.closest_emojis,
                        orbital_path: None, // Placeholder
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

                analyzed_documents.push(AnalyzedDocument {
                    file_path: file_path.to_string_lossy().into_owned(),
                    code_snippets,
                    text_chunks,
                    analyzed_snippets,
                });
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_documents)
}

pub fn analyze_project_tokens(project_root: &Path, ontology_path: &Path, use_gpu: bool, pb: &ProgressBar) -> Result<HashMap<String, AnalyzedToken>> {
    let mut analyzed_tokens: HashMap<String, AnalyzedToken> = HashMap::new();

    #[cfg(feature = "with-embedding")]
    let (bert_encoder, emoji_multivectors, device) = {
        let config = BertConfig::default();
        let encoder = BertCliffordEncoder::new(config);
        let emoji_mvs = solfunmeme_embedding::load_emoji_multivectors(ontology_path.to_str().unwrap())?;
        let device = if use_gpu { Device::cuda_if_available(0).unwrap() } else { Device::Cpu };
        (encoder, emoji_mvs, device)
    };

    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();

            if matches!(ext.as_str(), "rs" | "md" | "json" | "ttl" | "toml" | "txt" | "js" | "ts" | "tsx" | "py" | "go" | "java" | "c" | "cpp" | "h" | "hpp") {
                let content = fs::read_to_string(file_path)?;
                for word in content.split_whitespace() {
                    let mut multivector_str = String::new();
                    #[cfg(feature = "with-embedding")]
                    {
                        let embedding = embed_text(word, &device)?;
                        let multivector = bert_encoder.encode_embedding(&embedding)?;
                        multivector_str = format!("{:?}", multivector);
                    }
                    let entry = analyzed_tokens.entry(word.to_string()).or_insert_with(|| AnalyzedToken {
                        token: word.to_string(),
                        count: 0,
                        multivector_str,
                        orbital_path: None,
                    });
                    entry.count += 1;
                }
            }
            pb.inc(entry.metadata()?.len());
        }
    }
    Ok(analyzed_tokens)
}