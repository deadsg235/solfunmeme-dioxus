//! Duplicate Checker CLI Tool
//!
//! This program is one of the 42 canonical tools in the Solfunmeme suite.
//! It scans a directory for duplicate or highly similar code snippets (Rust, TOML, Markdown),
//! reports exact and near-duplicate code, and can output results in text or JSON format.
//!
//! Usage:
//!   cargo run --bin duplicate_check -- <directory> [--threshold <float>] [--format <json|text>]
//!
//! Example:
//!   cargo run --bin duplicate_check -- src --threshold 0.8 --format json

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct CodeSnippet {
    file_path: String,
    line_start: usize,
    line_end: usize,
    content: String,
    normalized_content: String,
    hash: String,
}

#[derive(Debug, Serialize)]
struct DuplicateGroup {
    similarity_score: f64,
    snippets: Vec<CodeSnippet>,
}

struct DuplicateChecker {
    snippets: Vec<CodeSnippet>,
    hash_map: HashMap<String, Vec<usize>>,
}

impl DuplicateChecker {
    fn new() -> Self {
        Self {
            snippets: Vec::new(),
            hash_map: HashMap::new(),
        }
    }

    fn analyze_directory(&mut self, dir_path: &Path) -> Result<(), Box<dyn Error>> {
        println!("🔍 Analyzing directory: {:?}", dir_path);
        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "toml" || ext == "md" {
                    self.analyze_file(path)?;
                }
            }
        }
        Ok(())
    }

    fn analyze_file(&mut self, file_path: &Path) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();
        let snippets = self.extract_snippets(&lines, file_path);
        for mut snippet in snippets {
            let hash = self.compute_hash(&snippet.normalized_content);
            snippet.hash = hash.clone();
            self.hash_map.entry(hash).or_insert_with(Vec::new).push(self.snippets.len());
            self.snippets.push(snippet);
        }
        Ok(())
    }

    fn extract_snippets(&self, lines: &[&str], file_path: &Path) -> Vec<CodeSnippet> {
        let mut snippets = Vec::new();
        let mut current_snippet = String::new();
        let mut start_line = 0;
        let mut in_code_block = false;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                if in_code_block {
                    if !current_snippet.trim().is_empty() {
                        snippets.push(CodeSnippet {
                            file_path: file_path.to_string_lossy().to_string(),
                            line_start: start_line + 1,
                            line_end: i + 1,
                            content: current_snippet.clone(),
                            normalized_content: self.normalize_content(&current_snippet),
                            hash: String::new(),
                        });
                    }
                    current_snippet.clear();
                    in_code_block = false;
                } else {
                    in_code_block = true;
                    start_line = i;
                }
            } else if in_code_block {
                current_snippet.push_str(line);
                current_snippet.push('\n');
            } else {
                if self.is_code_start(trimmed) {
                    if !current_snippet.trim().is_empty() {
                        snippets.push(CodeSnippet {
                            file_path: file_path.to_string_lossy().to_string(),
                            line_start: start_line + 1,
                            line_end: i + 1,
                            content: current_snippet.clone(),
                            normalized_content: self.normalize_content(&current_snippet),
                            hash: String::new(),
                        });
                    }
                    current_snippet = line.to_string();
                    current_snippet.push('\n');
                    start_line = i;
                } else if !current_snippet.is_empty() {
                    current_snippet.push_str(line);
                    current_snippet.push('\n');
                }
            }
        }
        if !current_snippet.trim().is_empty() {
            snippets.push(CodeSnippet {
                file_path: file_path.to_string_lossy().to_string(),
                line_start: start_line + 1,
                line_end: lines.len(),
                content: current_snippet,
                normalized_content: self.normalize_content(&current_snippet),
                hash: String::new(),
            });
        }
        snippets
    }

    fn is_code_start(&self, line: &str) -> bool {
        line.starts_with("pub fn ") || 
        line.starts_with("fn ") || 
        line.starts_with("pub struct ") || 
        line.starts_with("struct ") || 
        line.starts_with("pub enum ") || 
        line.starts_with("enum ") || 
        line.starts_with("impl ") || 
        line.starts_with("pub trait ") || 
        line.starts_with("trait ") ||
        line.starts_with("mod ") ||
        line.starts_with("pub mod ")
    }

    fn normalize_content(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty() && !line.starts_with("//"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn compute_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn find_duplicates(&self, threshold: f64) -> Vec<DuplicateGroup> {
        let mut groups = Vec::new();
        let mut processed = std::collections::HashSet::new();
        // Exact duplicates
        for (hash, indices) in &self.hash_map {
            if indices.len() > 1 && !processed.contains(hash) {
                let snippets = indices.iter().map(|&i| self.snippets[i].clone()).collect();
                groups.push(DuplicateGroup { similarity_score: 1.0, snippets });
                processed.insert(hash.clone());
            }
        }
        // Near-duplicates
        for i in 0..self.snippets.len() {
            for j in (i + 1)..self.snippets.len() {
                let sim = self.calculate_similarity(&self.snippets[i], &self.snippets[j]);
                if sim >= threshold && sim < 1.0 {
                    groups.push(DuplicateGroup {
                        similarity_score: sim,
                        snippets: vec![self.snippets[i].clone(), self.snippets[j].clone()],
                    });
                }
            }
        }
        groups
    }

    fn prove_no_duplicates(&self, module_path: &Path, threshold: f64) -> bool {
        let module_snippets: Vec<&CodeSnippet> = self.snippets.iter()
            .filter(|s| s.file_path.contains(module_path.to_string_lossy().as_ref()))
            .collect();
        if module_snippets.is_empty() {
            println!("✅ No code found in module: {:?}", module_path);
            return true;
        }
        let mut hashes = std::collections::HashSet::new();
        for snippet in &module_snippets {
            if !hashes.insert(&snippet.hash) {
                println!("❌ Found duplicate code in module: {:?}", module_path);
                println!("   Duplicate: {}", snippet.content.lines().next().unwrap_or(""));
                return false;
            }
        }
        for i in 0..module_snippets.len() {
            for j in (i + 1)..module_snippets.len() {
                let similarity = self.calculate_similarity(module_snippets[i], module_snippets[j]);
                if similarity > threshold {
                    println!("⚠️  Found similar code in module: {:?} (similarity: {:.2})", module_path, similarity);
                    println!("   File 1: {}:{}", module_snippets[i].file_path, module_snippets[i].line_start);
                    println!("   File 2: {}:{}", module_snippets[j].file_path, module_snippets[j].line_start);
                    return false;
                }
            }
        }
        println!("✅ No duplicates found in module: {:?}", module_path);
        true
    }

    fn calculate_similarity(&self, snippet1: &CodeSnippet, snippet2: &CodeSnippet) -> f64 {
        let words1: Vec<&str> = snippet1.normalized_content.split_whitespace().collect();
        let words2: Vec<&str> = snippet2.normalized_content.split_whitespace().collect();
        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }
        if words1.is_empty() || words2.is_empty() {
            return 0.0;
        }
        let common_words: Vec<&&str> = words1.iter().filter(|word| words2.contains(word)).collect();
        let union_size = words1.len() + words2.len() - common_words.len();
        if union_size == 0 {
            1.0
        } else {
            common_words.len() as f64 / union_size as f64
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    use clap::{Arg, Command};
    let matches = Command::new("duplicate_check")
        .about("Duplicate Checker CLI Tool (one of the 42 programs): Scans for duplicate or similar code snippets in a directory.")
        .arg(Arg::new("directory")
            .help("Directory to scan for duplicates")
            .required(true))
        .arg(Arg::new("threshold")
            .long("threshold")
            .short('t')
            .help("Similarity threshold for near-duplicates (0.0-1.0)")
            .default_value("0.8"))
        .arg(Arg::new("format")
            .long("format")
            .short('f')
            .help("Output format: json or text")
            .default_value("text"))
        .get_matches();
    let dir_path = PathBuf::from(matches.get_one::<String>("directory").unwrap());
    let threshold: f64 = matches.get_one::<String>("threshold").unwrap().parse().unwrap_or(0.8);
    let format = matches.get_one::<String>("format").unwrap().to_lowercase();
    if !dir_path.exists() {
        eprintln!("❌ Directory does not exist: {:?}", dir_path);
        return Ok(());
    }
    let mut checker = DuplicateChecker::new();
    checker.analyze_directory(&dir_path)?;
    let groups = checker.find_duplicates(threshold);
    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&groups).unwrap());
    } else {
        println!("\n📊 Analysis Results:");
        println!("Total snippets found: {}", checker.snippets.len());
        println!("Duplicate groups found: {}", groups.len());
        for (i, group) in groups.iter().enumerate() {
            println!("\nGroup {}: Similarity {:.2}", i + 1, group.similarity_score);
            for snippet in &group.snippets {
                println!("  File: {} (lines {}-{})", snippet.file_path, snippet.line_start, snippet.line_end);
                println!("  ---\n{}\n---", snippet.content.trim());
            }
        }
    }
    Ok(())
} 