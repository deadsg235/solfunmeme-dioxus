use anyhow::Result;
use std::path::PathBuf;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use sophia_iri::Iri;
use sophia_api::prelude::*;
use std::path::PathBuf;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use sophia_iri::Iri;

pub fn serialize_updated_ontology(graph: &FastGraph, em_prefix: &Iri<&'static str>, crates_root_prefix: &Iri<&'static str>) -> anyhow::Result<()> {
    let index_ttl_path = PathBuf::from("ontologies/index.ttl");
    let mut serializer = TurtleSerializer::new_stringifier();
    serializer.set_prefix("em", &em_prefix.to_string())?;
    serializer.set_prefix("crates_root", &crates_root_prefix.to_string())?;
    serializer.serialize_graph(&graph)?;
    let updated_ttl = serializer.to_string();
    std::fs::write(&index_ttl_path, updated_ttl)?;
    Ok(())
}
