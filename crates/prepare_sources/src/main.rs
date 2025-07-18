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
use solfunmeme_ontology_vibe::{load_graph, add_crate_data, add_emoji_data, serialize_graph};
use sophia_iri::Iri;
use sophia_api::prelude::*;

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
    let mut graph = load_graph()?;

    let em_prefix = Iri::new("http://example.org/emoji#").unwrap();
    let crates_root_prefix = Iri::new("http://example.org/crates_root#").unwrap();
    let has_clifford_vector_iri = Iri::new("http://example.org/ontology#hasCliffordVector").unwrap();

    add_crate_data(&mut graph, &crates_root_prefix, &has_clifford_vector_iri)?;
    add_emoji_data(&mut graph, &em_prefix, &has_clifford_vector_iri)?;
    serialize_graph(&graph, &em_prefix, &crates_root_prefix)?;

    Ok(())
}
