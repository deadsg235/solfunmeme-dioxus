use anyhow::Result;
use sophia_api::prelude::*;
use sophia_api::term::SimpleTerm;
use sophia_api::MownStr;
use sophia_inmem::graph::FastGraph;
use sophia_iri::{Iri, AsIriRef};

use solfunmeme_clifford::generate_multivector_from_string;
use crate::util::create_literal_simple_term;

pub fn add_emoji_data_internal(graph: &mut FastGraph, em_emoji_iri: &Iri<&'static str>, has_clifford_vector_iri: &Iri<&'static str>) -> Result<()> {
    let all_triples: Vec<_> = graph.triples().filter_map(Result::ok).map(|t| {
        (t.s().to_owned(), t.p().to_owned(), t.o().to_owned())
    }).collect();

    let triples_to_add: Vec<_> = all_triples.into_iter().filter_map(|(s, p, o)| {
        if p.iri() == Some(IriRef::new_unchecked(MownStr::from_ref("http://www.w3.org/1999/02/22-rdf-syntax-ns#type"))) && o.iri().map(|iri_ref| iri_ref.as_str().to_string()) == Some(em_emoji_iri.as_str().to_string()) {
            if let Some(subject_iri) = s.iri().map(|iri_ref| IriRef::<MownStr<'static>>::new_unchecked(iri_ref.as_str().to_string().into())) {
                let emoji_name = subject_iri.as_str().split('#').last().unwrap_or("").to_string();
                let multivector = generate_multivector_from_string(&emoji_name);
                let multivector_str = format!("{}", multivector);
                return Some((
                    subject_iri.to_owned(),
                    has_clifford_vector_iri.to_owned(),
                    create_literal_simple_term(&multivector_str),
                ));
            }
        }
        None
    }).collect();

    for (s, p, o) in triples_to_add {
        graph.insert(s, p, o)?;
    }
    Ok(())
}