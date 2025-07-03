use walkdir::WalkDir;
use std::fs;
use std::collections::HashMap;
use solfunmeme_dioxus::core::code_analyzer::CodeAnalyzer;

/**
idea : lets build a mini compiler right here
first filter out only rust files.
refactor this code into parts.
read the source
parse to ast
embed source into vector
report and identify duplicates

construction of functions to match struture of the code and 
reporting on the matches
analysis of the of the statistics of the matches
construction of new functions based on the statistics
construction of new functions based on the matches
construction of new functions based on the analysis
construction of new functions based on the reporting
construction of new functions based on the extraction
construction of new functions based on the mathematical model
construction of new functions based on the export
a recursive feedback loop using LLMS and statistics to dynamically create new functions and models in real time.


extraction of terms from identifiers
extraction of relcationships between terms
creation of mathematical mmodel

export data for gui (reduce the size)

 */
fn main() {
    // 1. Discover all Rust files
    let mut files = HashMap::new();
    for entry in WalkDir::new("src").into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && entry.path().extension().map_or(false, |e| e == "rs") {
            let path = entry.path().to_string_lossy().to_string();
            let content = fs::read_to_string(entry.path()).unwrap();
            files.insert(path, content);
        }
    }

    // 2. Analyze all files
    let mut analyzer = CodeAnalyzer::new(32, 0.8);
    let analyses = analyzer.analyze_multiple_files(files).unwrap();

    // 3. Collect and merge JSON ASTs (for now, just collect into an array)
    let mut asts = Vec::new();
    for analysis in analyses {
        let ast: serde_json::Value = serde_json::from_str(&analysis.json_ast).unwrap();
        asts.push(ast);
    }
    let merged_graph = serde_json::Value::Array(asts);

    // 4. Export or visualize merged_graph
    println!("{}", serde_json::to_string_pretty(&merged_graph).unwrap());
}
