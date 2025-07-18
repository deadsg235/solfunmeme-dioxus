use anyhow::Result;
use sophia_api::prelude::*;
use sophia_inmem::graph::FastGraph;
use sophia_iri::Iri;
use solfunmeme_clifford::generate_multivector_from_string;

pub fn process_emoji_data(graph: &mut FastGraph, em_emoji_iri: &Iri<&'static str>, has_clifford_vector_iri: &Iri<&'static str>) -> anyhow::Result<()> {
    let mut new_triples = Vec::new();
    for t in graph.triples() {
        let t = t?;
        if t.p().iri() == Some(&Iri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type")) && t.o().iri() == Some(&em_emoji_iri) {
            if let Some(subject_iri) = t.s().iri() {
                let emoji_name = subject_iri.as_str().split('#').last().unwrap_or("").to_string();
                let multivector = generate_multivector_from_string(&emoji_name);
                let multivector_str = format!("{}", multivector);
                new_triples.push(Triple::new(
                    subject_iri.to_owned(),
                    has_clifford_vector_iri.to_owned(),
                    Term::new_literal(&multivector_str),
                ));
            }
        }
    }
    for triple in new_triples {
        graph.insert(&triple.s(), &triple.p(), &triple.o())?;
    }
    Ok(())
}
