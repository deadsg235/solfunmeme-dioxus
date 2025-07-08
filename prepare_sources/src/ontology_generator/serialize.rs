use sophia_api::serializer::{TripleSerializer, StreamSerializer, PrefixSink};
use sophia_turtle::serializer::turtle::TurtleSerializer;
use std::io::{Write, BufWriter};
use std::fs::File;
use sophia::api::graph::Graph;
use sophia_api::prelude::IriRef;
use std::path::Path;

use crate::ontology_generator::namespaces::Namespaces;

pub fn serialize_graph_to_file<G>(
    graph: &G,
    output_path: &Path,
    ex_iri: &IriRef<String>,
    rdf_iri: &IriRef<String>,
    rdfs_iri: &IriRef<String>,
    em_iri: &IriRef<String>,
) -> anyhow::Result<()>
where
    G: Graph,
    <G as Graph>::Error: Send + Sync + 'static,
{
    let mut writer = TurtleSerializer::new(BufWriter::new(File::create(output_path)?));

    writer.set_prefix("ex", ex_iri.as_str())?;
    writer.set_prefix("rdf", rdf_iri.as_str())?;
    writer.set_prefix("rdfs", rdfs_iri.as_str())?;
    writer.set_prefix("em", em_iri.as_str())?;

    writer.serialize_graph(&graph)?;
    writer.flush()?;

    Ok(())
}