use anyhow::Result;
use sophia_api::prelude::*;
use sophia_api::term::SimpleTerm;
use sophia_api::MownStr;
use sophia_inmem::graph::FastGraph;
use sophia_iri::Iri;

use solfunmeme_clifford::generate_multivector_from_string;
use crate::util::create_literal_simple_term;

pub fn add_crate_data_internal(graph: &mut FastGraph, crates_root_prefix: &Iri<&'static str>, has_clifford_vector_iri: &Iri<&'static str>) -> Result<()> {
    let all_triples: Vec<_> = graph.triples().filter_map(Result::ok).map(|t| {
        (t.s().to_owned(), t.p().to_owned(), t.o().to_owned())
    }).collect();

    let triples_to_add: Vec<_> = all_triples.into_iter().filter_map(|(s, p, o)| {
        if p.iri() == Some(IriRef::new_unchecked(MownStr::from_ref("http://www.w3.org/2000/01/rdf-schema#label"))) {
            if let Some(subject_iri) = s.iri().map(|iri_ref| IriRef::<MownStr<'static>>::new_unchecked(iri_ref.as_str().to_string().into())) {
                if subject_iri.as_str().starts_with(crates_root_prefix.as_str()) {
                    let crate_name = match o {
                        SimpleTerm::LiteralDatatype(val, _) => val.to_string(),
                        SimpleTerm::LiteralLanguage(val, _) => val.to_string(),
                        _ => return None,
                    };
                    let multivector = generate_multivector_from_string(&crate_name);
                    let multivector_str = format!("{}", multivector);
                    return Some((
                        subject_iri.to_owned(),
                        has_clifford_vector_iri.to_owned(),
                        create_literal_simple_term(&multivector_str),
                    ));
                }
            }
        }
        None
    }).collect();

    for (s, p, o) in triples_to_add {
        graph.insert(&s, &p, &o)?;
    }
    Ok(())
}