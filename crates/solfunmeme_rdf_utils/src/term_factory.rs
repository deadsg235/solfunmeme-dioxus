use sophia_api::term::SimpleTerm;
use sophia_iri::Iri;

pub fn iri_term(iri_string: &str) -> anyhow::Result<SimpleTerm<'static>> {
    Ok(Iri::new(iri_string.to_string())?.into_term())
}

pub fn literal_term(value: &str) -> SimpleTerm<'static> {
    SimpleTerm::new_literal_untyped(value.to_string())
}

pub fn literal_term_typed(value: &str, datatype_iri: &str) -> anyhow::Result<SimpleTerm<'static>> {
    Ok(SimpleTerm::new_literal_dt(value.to_string(), Iri::new(datatype_iri.to_string())?.into_term())?)
}

pub fn bnode_term(id: &str) -> SimpleTerm<'static> {
    SimpleTerm::new_bnode(id.to_string())
}
