use syn::{ItemFn, ReturnType};
use std::path::Path;
use walkdir::WalkDir;
use shared_analysis_types::CodeSnippet;
use md5;

pub struct FunctionInfo {
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

pub fn extract_code_snippets(content: &str) -> Vec<CodeSnippet> {
    let mut snippets = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut in_code_block = false;
    let mut current_language = String::new();
    let mut current_content = String::new();
    let mut start_line = 0;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("```") {
            if in_code_block {
                // End of code block
                if !current_content.trim().is_empty() {
                    let snippet = shared_analysis_types::create_code_snippet(
                        current_language.clone(),
                        current_content.clone(),
                        start_line,
                        i,
                    );
                    snippets.push(snippet);
                }
                current_content.clear();
                in_code_block = false;
            } else {
                // Start of code block
                current_language = line[3..].trim().to_string();
                if current_language.is_empty() {
                    current_language = "text".to_string();
                }
                current_content.clear();
                start_line = i + 1;
                in_code_block = true;
            }
        } else if in_code_block {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }

    // Handle unclosed code block
    if in_code_block && !current_content.trim().is_empty() {
        let snippet =
            shared_analysis_types::create_code_snippet(current_language, current_content, start_line, lines.len());
        snippets.push(snippet);
    }

    snippets
}

pub fn analyze_rust_file(file_path: &Path) -> Vec<FunctionInfo> {
    let mut functions_info = Vec::new();
    let code = match std::fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read file {}: {}", file_path.display(), e);
            return functions_info;
        }
    };

    let syntax = match syn::parse_file(&code) {
        Ok(syntax) => syntax,
        Err(e) => {
            eprintln!("Failed to parse file {}: {}", file_path.display(), e);
            return functions_info;
        }
    };

    for item in syntax.items {
        if let syn::Item::Fn(item_fn) = item {
            let function_name = item_fn.sig.ident.to_string();
            let code_snippet = quote::quote! { #item_fn }.to_string();
            let semantic_summary = extract_semantic_summary(&item_fn);
            functions_info.push(FunctionInfo {
                function_name,
                code_snippet,
                semantic_summary,
                file_path: file_path.to_string_lossy().replace("\\", "/").to_owned(),
                multivector_str: String::new(), // Placeholder
                sieve_address: String::new(),   // Placeholder
                closest_emoji: String::new(),   // Placeholder
                emoji_category: String::new(),  // Placeholder
                emoji_distance: 0.0,            // Placeholder
            });
        }
    }
    functions_info
}

fn extract_semantic_summary(item_fn: &syn::ItemFn) -> String {
    let mut summary = String::new();
    // Extract identifiers and literals from the function to form a semantic summary
    // This is a simplified example; a real implementation would traverse the AST more thoroughly.
    summary.push_str(&item_fn.sig.ident.to_string());
    for input in &item_fn.sig.inputs {
        summary.push_str(&format!("{:?}", input));
    }
    if let ReturnType::Type(_, ty) = &item_fn.sig.output {
        summary.push_str(&format!("{:?}", ty));
    }
    // Add more AST traversal logic here to extract meaningful information
    summary
}

pub fn find_rust_files(project_root: &Path) -> Vec<String> {
    let mut rust_files = Vec::new();
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "rs" {
                    rust_files.push(entry.path().to_string_lossy().into_owned());
                }
            }
        }
    }
    rust_files
}