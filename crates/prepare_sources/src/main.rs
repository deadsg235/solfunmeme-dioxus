use anyhow::Result;
use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;
use serde_json;
use clap::Parser;

mod cli;
mod code_processing;

use cli::Cli;
use code_processing::process_code_chunks;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let target_path = cli.path.map(|p| p.to_string_lossy().into_owned());
    let limit = cli.limit;

    let mut output_writer: Box<dyn Write> = if let Some(output_path) = cli.output {
        Box::new(File::create(&output_path)?)
    } else {
        Box::new(io::stdout())
    };

    process_code_chunks(target_path, limit, &mut output_writer)?;

    // Ontology processing
    let mut graph = RdfGraph::new();
    graph.namespaces.add_namespace("em", "http://example.org/emoji#")?;
    graph.namespaces.add_namespace("crates_root", "http://example.org/crates_root#")?;
    graph.namespaces.add_namespace("onto", "http://example.org/ontology#")?;

    // The logic from solfunmeme_ontology_vibe needs to be moved here or to a new utility crate.
    // For now, I'll comment out the parts that use it.
    // add_crate_data(&mut graph, &crates_root_prefix, &has_clifford_vector_iri)?;
    // add_emoji_data(&mut graph, &em_prefix, &has_clifford_vector_iri)?;
    // serialize_graph(&graph, &em_prefix, &crates_root_prefix)?;

    Ok(())
}