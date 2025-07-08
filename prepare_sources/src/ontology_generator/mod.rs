use anyhow::Result;
use sophia::inmem::graph::LightGraph;
use std::path::Path;

mod namespaces;
mod process_function;
mod serialize;

pub fn generate_ontology(analyzed_functions: Vec<crate::project_analyzer::AnalyzedFunction>, output_path: &Path) -> Result<()> {
    let mut graph = LightGraph::new();
    let ns = namespaces::define_namespaces();

    for func in analyzed_functions {
        process_function::process_analyzed_function(&mut graph, func, &ns)?;
    }

    serialize::serialize_graph_to_file(&graph, output_path, &ns.ex_iri, &ns.rdf_iri, &ns.rdfs_iri, &ns.em_iri)?;

    Ok(())
}
