use sophia_api::prelude::IriRef;
use sophia_api::term::SimpleTerm;

pub struct Namespaces {
    pub ex_iri: IriRef<String>,
    pub rdf_iri: IriRef<String>,
    pub rdfs_iri: IriRef<String>,
    pub em_iri: IriRef<String>,

    pub ex: SimpleTerm,
    pub rdf: SimpleTerm,
    pub rdfs: SimpleTerm,
    pub em: SimpleTerm,
}

pub fn define_namespaces() -> Namespaces {
    let ex_iri: IriRef<String> = IriRef::new_unchecked("http://example.org/ontology/".into());
    let rdf_iri: IriRef<String> = IriRef::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#".into());
    let rdfs_iri: IriRef<String> = IriRef::new_unchecked("http://www.w3.org/2000/01/rdf-schema#".into());
    let em_iri: IriRef<String> = IriRef::new_unchecked("http://example.org/emoji#".into());

    let ex = SimpleTerm::Iri(ex_iri.clone());
    let rdf = SimpleTerm::Iri(rdf_iri.clone());
    let rdfs = SimpleTerm::Iri(rdfs_iri.clone());
    let em = SimpleTerm::Iri(em_iri.clone());

    Namespaces {
        ex_iri,
        rdf_iri,
        rdfs_iri,
        em_iri,
        ex,
        rdf,
        rdfs,
        em,
    }
}
