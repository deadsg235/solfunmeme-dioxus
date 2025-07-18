use sophia_api::term::{SimpleTerm, Term};
use sophia_iri::Iri;

pub fn iri_term(iri_string: &str) -> anyhow::Result<SimpleTerm> {
    Ok(Term::new_iri(Iri::new(iri_string.to_string())?))
}

pub fn literal_term(value: &str) -> SimpleTerm {
    Term::new_literal_untyped(value.to_string())
}

pub fn literal_term_typed<'a>(value: &'a str, datatype_iri: &'a str) -> anyhow::Result<SimpleTerm<'a>> {
    let iri = Iri::new(datatype_iri)?;
    Ok(Term::new_literal_dt(value.to_string(), iri))
}

pub fn bnode_term(id: &str) -> anyhow::Result<SimpleTerm> {
    Ok(Term::new_bnode(id.to_string())?)
}