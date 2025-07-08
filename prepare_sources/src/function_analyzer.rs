use anyhow::{Result, anyhow};
use syn::{File, Item, ItemFn, ItemMod, Ident, Lit};
use quote::ToTokens;
use std::fs;
use std::path::{Path, PathBuf};

pub struct FunctionInfo {
    pub name: String,
    pub code: String,
    pub semantic_summary: String,
}

pub fn analyze_rust_file(file_path: &Path) -> Result<Vec<FunctionInfo>> {
    let source_code = fs::read_to_string(file_path)?;
    
    // Pre-process: remove lines with #[cfg(...)] attributes
    let processed_code: String = source_code
        .lines()
        .filter(|line| !line.trim_start().starts_with("#[cfg("))
        .collect::<Vec<&str>>()
        .join("\n");

    let syntax_tree: File = syn::parse_file(&processed_code)
        .map_err(|e| anyhow!("Failed to parse Rust file {}: {}", file_path.display(), e))?;

    let mut functions = Vec::new();
    println!("DEBUG: Analyzing file: {}", file_path.display());
    println!("DEBUG: Top-level items found: {}", syntax_tree.items.len());
    extract_functions_from_items(syntax_tree.items.into_iter(), file_path.parent().unwrap_or(Path::new(".")), &mut functions);
    Ok(functions)
}

fn extract_functions_from_items(items: impl Iterator<Item = Item>, base_dir: &Path, functions: &mut Vec<FunctionInfo>) {
    for item in items {
        match item {
            Item::Fn(func) => {
                println!("DEBUG: Found function: {}", func.sig.ident);
                let mut semantic_tokens = Vec::new();
                semantic_tokens.push(func.sig.ident.to_string()); // Function name

                // Extract identifiers from signature
                for input in &func.sig.inputs {
                    if let syn::FnArg::Typed(pat_type) = input {
                        if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                            semantic_tokens.push(pat_ident.ident.to_string());
                        }
                    }
                }

                // Extract identifiers and literals from function body
                for stmt in &func.block.stmts {
                    match stmt {
                        syn::Stmt::Local(local) => {
                            if let Some(init) = &local.init {
                                extract_tokens_from_expr(&init.expr, &mut semantic_tokens);
                            }
                        }
                        syn::Stmt::Expr(expr, _) => {
                            extract_tokens_from_expr(expr, &mut semantic_tokens);
                        }
                        _ => {},
                    }
                }

                let semantic_summary = semantic_tokens.join(" ");

                functions.push(FunctionInfo {
                    name: func.sig.ident.to_string(),
                    code: func.to_token_stream().to_string(),
                    semantic_summary,
                });
            }
            Item::Mod(module) => {
                println!("DEBUG: Found module: {}", module.ident);
                if let Some((_, module_items)) = module.content {
                    // Inline module
                    extract_functions_from_items(module_items.into_iter(), base_dir, functions);
                } else {
                    // External module (e.g., `mod my_module;`)
                    let module_name_str = module.ident.to_string();
                    let module_file_path = base_dir.join(format!("{}.rs", module_name_str));
                    let module_dir_path = base_dir.join(&module_name_str);

                    if module_file_path.exists() {
                        println!("DEBUG: Recursively analyzing external module file: {}", module_file_path.display());
                        match analyze_rust_file(&module_file_path) {
                            Ok(mut found_functions) => functions.append(&mut found_functions),
                            Err(e) => eprintln!("ERROR: Failed to analyze module file {}: {}", module_file_path.display(), e),
                        }
                    } else if module_dir_path.exists() && module_dir_path.join("mod.rs").exists() {
                        println!("DEBUG: Recursively analyzing external module directory: {}", module_dir_path.display());
                        match analyze_rust_file(&module_dir_path.join("mod.rs")) {
                            Ok(mut found_functions) => functions.append(&mut found_functions),
                            Err(e) => eprintln!("ERROR: Failed to analyze module directory {}: {}", module_dir_path.display(), e),
                        }
                    } else {
                        eprintln!("WARNING: Could not find module file or directory for: {}", module.ident);
                    }
                }
            }
            _ => {
                println!("DEBUG: Found other item type.");
            },
        }
    }
}

fn extract_tokens_from_expr(expr: &syn::Expr, tokens: &mut Vec<String>) {
    match expr {
        syn::Expr::Lit(expr_lit) => {
            if let Lit::Str(lit_str) = &expr_lit.lit {
                tokens.push(lit_str.value());
            } else if let Lit::Int(lit_int) = &expr_lit.lit {
                tokens.push(lit_int.to_string());
            } else if let Lit::Bool(lit_bool) = &expr_lit.lit {
                tokens.push(lit_bool.value.to_string()); // Access the bool value directly
            }
        }
        syn::Expr::Path(expr_path) => {
            if let Some(ident) = expr_path.path.get_ident() {
                tokens.push(ident.to_string());
            }
        }
        syn::Expr::Call(expr_call) => {
            extract_tokens_from_expr(&expr_call.func, tokens);
            for arg in &expr_call.args {
                extract_tokens_from_expr(arg, tokens);
            }
        }
        syn::Expr::MethodCall(expr_method_call) => {
            tokens.push(expr_method_call.method.to_string());
            extract_tokens_from_expr(&expr_method_call.receiver, tokens);
            for arg in &expr_method_call.args {
                extract_tokens_from_expr(arg, tokens);
            }
        }
        syn::Expr::Binary(expr_binary) => {
            extract_tokens_from_expr(&expr_binary.left, tokens);
            tokens.push(expr_binary.op.to_token_stream().to_string());
            extract_tokens_from_expr(&expr_binary.right, tokens);
        }
        syn::Expr::Unary(expr_unary) => {
            tokens.push(expr_unary.op.to_token_stream().to_string());
            extract_tokens_from_expr(&expr_unary.expr, tokens);
        }
        syn::Expr::Assign(expr_assign) => {
            extract_tokens_from_expr(&expr_assign.left, tokens);
            extract_tokens_from_expr(&expr_assign.right, tokens);
        }
        syn::Expr::Block(expr_block) => {
            for stmt in &expr_block.block.stmts {
                match stmt {
                    syn::Stmt::Local(local) => {
                        if let Some(init) = &local.init {
                            extract_tokens_from_expr(&init.expr, tokens);
                        }
                    }
                    syn::Stmt::Expr(expr, _) => {
                        extract_tokens_from_expr(expr, tokens);
                    }
                    _ => {},
                }
            }
        }
        // Add more Expr variants as needed for deeper semantic extraction
        _ => {},
    }
}