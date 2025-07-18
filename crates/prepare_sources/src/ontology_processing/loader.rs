use anyhow::Result;
use std::path::PathBuf;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::parser::turtle::TurtleParser;



pub fn load_ontologies() -> anyhow::Result<FastGraph> {
    let index_ttl_path = PathBuf::from("ontologies/index.ttl");
    let zos_v1_ttl_path = PathBuf::from("ontologies/zos/v1.ttl");

    let mut graph = FastGraph::new();

    let index_ttl_content = std::fs::read_to_string(&index_ttl_path)?;
    TurtleParser::new(index_ttl_content.as_bytes(), None).add_to_graph(&mut graph)?;

    // Load zos/v1.ttl
    let zos_v1_ttl_content = std::fs::read_to_string(&zos_v1_ttl_path)?;
    TurtleParser::new(zos_v1_ttl_content.as_bytes(), None).add_to_graph(&mut graph)?;

    Ok(graph)
}
