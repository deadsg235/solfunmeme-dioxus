use anyhow::Result;
use std::fs;
use sophia_turtle::parser::turtle::parse_str;
use sophia_api::term::SimpleTerm;
use sophia_api::source::TripleSource;

pub fn read_and_parse_triples(ontology_path: &str) -> Result<Vec<[SimpleTerm; 3]>> {
    let turtle_data = fs::read_to_string(ontology_path)?;
    let triples: Vec<[SimpleTerm; 3]> = parse_str(&turtle_data).collect_triples().map_err(|e| anyhow::anyhow!("Failed to parse Turtle: {:?}" , e))?;
    Ok(triples)
}
