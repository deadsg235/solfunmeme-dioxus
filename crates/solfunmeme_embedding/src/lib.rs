use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use serde_json;

use sophia_turtle::parser::turtle::parse_str;
use sophia_api::term::SimpleTerm;
use sophia_api::triple::Triple;
use sophia_api::source::TripleSource;

use solfunmeme_clifford::{SolMultivector};

mod ontology_processing;

pub fn load_emoji_multivectors(ontology_path: &str) -> Result<HashMap<String, (SolMultivector, String)>> {
    let triples = ontology_processing::read_and_parse_triples::read_and_parse_triples(ontology_path)?;
    let (emoji_data, concept_descriptions) = ontology_processing::extract_emoji_data::extract_emoji_data(&triples);
    let emoji_multivectors = ontology_processing::process_emoji_multivectors::process_emoji_multivectors(emoji_data, concept_descriptions)?;

    Ok(emoji_multivectors)
}

// Helper function to convert a SimpleTerm to a String
fn term_to_string(term: &SimpleTerm) -> String {
    match term {
        SimpleTerm::Iri(iri) => iri.to_string(),
        SimpleTerm::LiteralDatatype(value, _) => value.to_string(),
        SimpleTerm::LiteralLanguage(value, _) => value.to_string(),
        _ => "".to_string(), // Handle other term types as needed
    }
}
