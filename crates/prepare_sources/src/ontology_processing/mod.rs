use anyhow::Result;
use std::path::PathBuf;

pub mod loader;
pub mod crate_processor;
pub mod emoji_processor;
pub mod serializer;

use loader::load_ontologies;
use crate_processor::process_crate_data;
use emoji_processor::process_emoji_data;
use serializer::serialize_updated_ontology;

pub fn process_ontologies() -> anyhow::Result<()> {
    eprintln!("[INFO] Processing ontologies...");

    let mut graph = load_ontologies()?;

    let em_prefix = sophia_iri::Iri::new_unchecked("http://example.org/emoji#");
    let crates_root_prefix = sophia_iri::Iri::new_unchecked("http://example.org/crates_root#");
    let has_clifford_vector_iri = sophia_iri::Iri::new_unchecked("http://example.org/emoji#hasCliffordVector");

    process_crate_data(&mut graph, &crates_root_prefix, &has_clifford_vector_iri)?;
    process_emoji_data(&mut graph, &em_prefix, &has_clifford_vector_iri)?;

    serialize_updated_ontology(&graph, &em_prefix, &crates_root_prefix)?;

    eprintln!("[INFO] Ontologies processed and updated with Clifford vectors.");

    Ok(())
}
