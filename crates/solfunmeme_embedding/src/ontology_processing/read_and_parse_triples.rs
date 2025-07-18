use anyhow::Result;
use std::fs;
use sophia_turtle::parser::turtle::TurtleParser;
use sophia_api::term::Term;
use sophia_api::source::TripleSource;
use sophia_inmem::graph::FastGraph;
use sophia_api::graph::Graph;

pub fn read_and_parse_triples(ontology_path: &str) -> anyhow::Result<FastGraph> {
    let turtle_data = fs::read_to_string(ontology_path)?;
    let mut graph = FastGraph::new();
    TurtleParser::new().parse_str(&turtle_data).add_to_graph(&mut graph)?;
    Ok(graph)
}
