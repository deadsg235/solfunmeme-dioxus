use anyhow::Result;
use sophia_api::prelude::*;
use sophia_api::parser::TripleParser;
use sophia_api::prefix::{Prefix, PrefixMap, PrefixMapPair};
use sophia_api::source::TripleSource;
use sophia_api::term::SimpleTerm;
use sophia_api::MownStr;
use sophia_inmem::graph::FastGraph;
use sophia_iri::{Iri, AsIriRef};
use sophia_turtle::parser::turtle::TurtleParser;
use sophia_turtle::serializer::turtle::{TurtleSerializer, TurtleConfig};
use std::path::PathBuf;

use solfunmeme_clifford::generate_multivector_from_string;

// Internal function to load graph from TTL files
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

// Internal function to add crate data to the graph
pub fn add_crate_data_internal(graph: &mut FastGraph, crates_root_prefix: &Iri<&'static str>, has_clifford_vector_iri: &Iri<&'static str>) -> Result<()> {
    let mut new_triples = Vec::new();
    for t in graph.triples() {
        let t = t?;
        if t.p().iri() == Some(IriRef::new_unchecked(MownStr::from_str("http://www.w3.org/2000/01/rdf-schema#label"))) {
            if let Some(subject_iri) = t.s().iri() {
                if subject_iri.as_str().starts_with(crates_root_prefix.as_str()) {
                    let crate_name = match t.o() {
                        SimpleTerm::LiteralDatatype(val, _) => val.to_string(),
                        SimpleTerm::LiteralLanguage(val, _) => val.to_string(),
                        _ => continue, // Skip if not a literal
                    };
                    let multivector = generate_multivector_from_string(&crate_name);
                    let multivector_str = format!("{}", multivector);
                    new_triples.push(sophia_api::triple::Triple::new(
                        subject_iri.to_owned(),
                        has_clifford_vector_iri.to_owned(),
                        multivector_str.to_string().into_term(),
                    ));
                }
            }
        }
    }
    for triple in new_triples {
        graph.insert(&triple.s(), &triple.p(), &triple.o())?;
    }
    Ok(())
}

// Internal function to add emoji data to the graph
pub fn add_emoji_data_internal(graph: &mut FastGraph, em_emoji_iri: &Iri<&'static str>, has_clifford_vector_iri: &Iri<&'static str>) -> Result<()> {
    let mut new_triples = Vec::new();
    for t in graph.triples() {
        let t = t?;
        if t.p().iri() == Some(IriRef::new_unchecked(MownStr::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"))) && t.o().iri() == Some(&em_emoji_iri.as_iri_ref().to_owned()) {
            if let Some(subject_iri) = t.s().iri() {
                let emoji_name = subject_iri.as_str().split('#').last().unwrap_or("").to_string();
                let multivector = generate_multivector_from_string(&emoji_name);
                let multivector_str = format!("{}", multivector);
                new_triples.push(sophia_api::triple::Triple::new(
                    subject_iri.to_owned(),
                    has_clifford_vector_iri.to_owned(),
                    multivector_str.to_string().into_term(),
                ));
            }
        }
    }
    for triple in new_triples {
        graph.insert(&triple.s(), &triple.p(), &triple.o())?;
    }
    Ok(())
}

// Internal function to serialize the graph to a TTL file
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
