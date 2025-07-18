use sophia_api::prelude::{IriRef, Term};
use sophia_api::term::SimpleTerm;
use sophia_iri::Iri;
use std::collections::HashMap;

pub struct NamespaceManager<'a> {
    namespaces: HashMap<String, IriRef<String>>,
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
        let iri_ref = IriRef::new(iri.to_string())?;
        self.namespaces.insert(prefix.to_string(), iri_ref.clone());
        self.terms
            .insert(prefix.to_string(), iri_ref.into_term());
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

    pub fn get_base_iri(&self, prefix: &str) -> Option<&IriRef<String>> {
        self.namespaces.get(prefix)
    }
}
