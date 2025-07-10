use anyhow::Result;
use solfunmeme_function_analysis::CodeChunk;
use solfunmeme_search_tantivy::SearchIndex;
use solfunmeme_tantivy_report::{get_top_entries, ReportType};
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn index_directory(path: &Path, index_path: &Path) -> Result<()> {
    let mut search_index = SearchIndex::new(index_path)?;

    let child = Command::new("cargo")
        .args([
            "run",
            "-p",
            "prepare_sources",
            "--bin",
            "prepare_sources",
            "--",
            path.to_str().unwrap(),
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.expect("Failed to get stdout");
    let reader = io::BufReader::new(stdout);

    for line in reader.lines() {
        let line = line?;
        let chunk: CodeChunk = serde_json::from_str(&line)?;
        search_index.add_chunk(&chunk)?;
    }

    search_index.commit()
}

pub fn report_top_entries(index_path: &Path, report_type: &str, limit: usize) -> Result<()> {
    let report_type = ReportType::Field(report_type.to_string());
    let top_entries = get_top_entries(index_path, report_type, limit)?;

    for (entry, count) in top_entries {
        println!("{}: {}", entry, count);
    }

    Ok(())
}
