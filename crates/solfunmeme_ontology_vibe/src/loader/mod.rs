use anyhow::Result;
use sophia_api::prelude::*;
use sophia_api::parser::TripleParser;
use sophia_api::source::TripleSource;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle::TurtleParser;
use std::path::PathBuf;

pub fn load_graph_internal() -> Result<FastGraph> {
    let index_ttl_path = PathBuf::from("ontologies/index.ttl");
    let zos_v1_ttl_path = PathBuf::from("ontologies/zos/v1.ttl");

    let mut graph = FastGraph::new();

    let index_ttl_content = std::fs::read_to_string(&index_ttl_path)?;
    TurtleParser::default().parse_str(&index_ttl_content).add_to_graph(&mut graph)?;

    let zos_v1_ttl_content = std::fs::read_to_string(&zos_v1_ttl_path)?;
    TurtleParser::default().parse_str(&zos_v1_ttl_content).add_to_graph(&mut graph)?;

    Ok(graph)
}