use anyhow::Result;
use sophia_api::prelude::*;
use sophia_api::term::SimpleTerm;
use sophia_api::MownStr;
use sophia_inmem::graph::FastGraph;
use sophia_iri::Iri;

use solfunmeme_clifford::generate_multivector_from_string;
use crate::create_literal_simple_term;

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
                        create_literal_simple_term(&multivector_str),
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