use std::env;
use std::path::Path;

use prepare_sources::project_analyzer::analyze_project;
use prepare_sources::ontology_generator::generate_ontology;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <project_root_path>");
        return;
    }

    let project_root = Path::new(&args[1]);
    let ontology_path = Path::new("../ontologies/zos/v1.ttl"); // Path to your ontology
    let output_ontology_path = Path::new("project_ontology.ttl");

    println!("Analyzing project: {}", project_root.display());
    match analyze_project(project_root, ontology_path) {
        Ok(analyzed_functions) => {
            println!("\n--- Analysis Results ---");
            if analyzed_functions.is_empty() {
                println!("No functions found or analyzed in the project.");
            } else {
                for func in &analyzed_functions {
                    println!("\nFile: {}", func.file_path);
                    println!("  Function: {}", func.function_name);
                    println!("  Code Snippet (first 100 chars): '{:.100}'\n", func.code_snippet);
                    println!("  Semantic Summary (first 100 chars): '{:.100}'\n", func.semantic_summary);
                    println!("  Multivector: {}", func.multivector_str);
                    println!("  Sieve Address: {}", func.sieve_address);
                    println!("  Closest Emoji: {} ({}) with distance: {}", func.closest_emoji, func.emoji_category, func.emoji_distance);
                }

                // Generate and save ontology
                match generate_ontology(analyzed_functions, output_ontology_path) {
                    Ok(_) => println!("\nSuccessfully generated ontology to {}.", output_ontology_path.display()),
                    Err(e) => eprintln!("Error generating ontology: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Error analyzing project: {}", e);
        }
    }
}
