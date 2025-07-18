use anyhow::Result;
use sophia_api::prelude::*;
use sophia_api::prefix::{Prefix, PrefixMap, PrefixMapPair};
use sophia_iri::Iri;
use sophia_turtle::serializer::turtle::{TurtleSerializer, TurtleConfig};
use std::path::PathBuf;

pub fn serialize_graph_internal(graph: &FastGraph, em_prefix: &Iri<&'static str>, crates_root_prefix: &Iri<&'static str>) -> Result<()> {
    let index_ttl_path = PathBuf::from("ontologies/index.ttl");
    let mut config = TurtleConfig::new();
    let prefix_map_vec = vec![(Prefix::new_unchecked("em").to_owned(), em_prefix.to_owned().into()), (Prefix::new_unchecked("crates_root").to_owned(), crates_root_prefix.to_owned().into())];
    config = config.with_own_prefix_map(prefix_map_vec);
    let mut serializer = TurtleSerializer::new_stringifier_with_config(config);
    serializer.serialize_graph(&graph)?;
    let updated_ttl = serializer.to_string();
    std::fs::write(&index_ttl_path, updated_ttl)?;
    Ok(())
}
