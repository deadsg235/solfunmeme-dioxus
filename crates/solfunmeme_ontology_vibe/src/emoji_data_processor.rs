use anyhow::Result;
use sophia_api::prelude::*;
use sophia_api::term::SimpleTerm;
use sophia_api::MownStr;
use sophia_inmem::graph::FastGraph;
use sophia_iri::{Iri, AsIriRef};

use solfunmeme_clifford::generate_multivector_from_string;

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
