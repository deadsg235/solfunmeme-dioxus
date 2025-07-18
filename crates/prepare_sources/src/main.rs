use anyhow::Result;
use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;
use serde_json;
use clap::Parser;

mod cli;
mod ontology_processing;
mod code_processing;

use cli::Cli;
use ontology_processing::process_ontologies;
use code_processing::process_code_chunks;

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

    process_ontologies()?;

    Ok(())
}
