use sophia_api::term::SimpleTerm;
use sophia_iri::Iri;
use std::collections::HashMap;
use crate::term_factory;

pub struct NamespaceManager<'a> {
    namespaces: HashMap<String, String>, // Store IRI as String
    terms: HashMap<String, SimpleTerm<'a>>,
}

impl<'a> NamespaceManager<'a> {
    pub fn new() -> Self {
        NamespaceManager {
            namespaces: HashMap::new(),
            terms: HashMap::new(),
        }
    }

    pub fn add_namespace(&mut self, prefix: &str, iri: &str) -> anyhow::Result<()> {
        self.namespaces.insert(prefix.to_string(), iri.to_string());
        self.terms
            .insert(prefix.to_string(), term_factory::iri_term(iri)?);
        Ok(())
    }

    pub fn get_iri(&self, prefix: &str, local_name: &str) -> anyhow::Result<Iri<String>> {
        let base_iri = self
            .namespaces
            .get(prefix)
            .ok_or_else(|| anyhow::anyhow!("Namespace prefix not found: {}", prefix))?;
        Ok(Iri::new(format!("{}{}", base_iri, local_name))?)
    }

    pub fn get_term(&self, prefix: &str, local_name: &str) -> anyhow::Result<SimpleTerm<'a>> {
        let iri = self.get_iri(prefix, local_name)?;
        Ok(iri.into_term())
    }

    pub fn get_base_iri(&self, prefix: &str) -> Option<&String> { // Return &String instead of &IriRef
        self.namespaces.get(prefix)
    }
}
