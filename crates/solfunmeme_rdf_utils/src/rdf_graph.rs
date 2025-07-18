use crate::namespace_manager::NamespaceManager;
use sophia_api::graph::MutableGraph;
use sophia_api::prelude::{IriRef, Term};
use sophia_api::serializer::TripleSerializer;
use sophia_api::term::SimpleTerm;
use sophia_inmem::graph::FastGraph;
use sophia_iri::Iri;
use sophia_turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use crate::term_factory; // Import the new term_factory

pub struct RdfGraph<'a> {
    graph: FastGraph,
    pub namespaces: NamespaceManager<'a>,
}

use sophia_api::parser::TripleParser;
use sophia_turtle::parser::TurtleParser;

use sophia_jsonld::JsonLdParser;
use sophia_jsonld::loader::FsLoader;
use serde_json::Value;

impl<'a> RdfGraph<'a> {
    pub fn from_jsonld_file(path: &Path) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let jsonld_data: Value = serde_json::from_reader(reader)?;
        let mut parser = JsonLdParser::new(FsLoader::new());
        let graph: FastGraph = parser.parse_json_ld(jsonld_data)?;
        Ok(RdfGraph {
            graph,
            namespaces: NamespaceManager::new(),
        })
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let graph: FastGraph = TurtleParser::new()
            .parse_str(&content)
            .collect_graph()?;
        Ok(RdfGraph {
            graph,
            namespaces: NamespaceManager::new(),
        })
    }

    pub fn new() -> Self {
        RdfGraph {
            graph: FastGraph::new(),
            namespaces: NamespaceManager::new(),
        }
    }

    // Modified add_triple to accept &str for IRIs
    pub fn add_triple(
        &mut self,
        subject_iri: &str,
        predicate_iri: &str,
        object_iri: &str,
    ) -> anyhow::Result<()> {
        let subject = term_factory::iri_term(subject_iri)?;
        let predicate = term_factory::iri_term(predicate_iri)?;
        let object = term_factory::iri_term(object_iri)?;
        self.graph.insert(&subject, &predicate, &object)?;
        Ok(())
    }

    // Modified add_literal_triple to accept &str for literal_type
    pub fn add_literal_triple(
        &mut self,
        subject_iri: &str,
        predicate_iri: &str,
        literal_value: &str,
        literal_type_iri: &str,
    ) -> anyhow::Result<()> {
        let subject = term_factory::iri_term(subject_iri)?;
        let predicate = term_factory::iri_term(predicate_iri)?;
        let literal = term_factory::literal_term_typed(literal_value, literal_type_iri)?;
        self.graph.insert(&subject, &predicate, &literal)?;
        Ok(())
    }

    pub fn get_object_literal(
        &self,
        subject: &SimpleTerm<'a>,
        predicate: &SimpleTerm<'a>,
    ) -> anyhow::Result<Option<String>> {
        if let Some(t) = self.graph.triples_with_sp(subject, predicate).next() {
            let t = t?;
            if let Some(literal) = t.o().as_literal() {
                return Ok(Some(literal.value().to_string()));
            }
        }
        Ok(None)
    }

    pub fn get_subjects_with_property(
        &self,
        predicate: &SimpleTerm<'a>,
        object: &SimpleTerm<'a>,
    ) -> anyhow::Result<Vec<SimpleTerm<'a>>> {
        Ok(self
            .graph
            .triples_with_po(predicate, object)
            .map(|t| t.map(|t| t.s().clone()))
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn get_property_value(
        &self,
        subject: &SimpleTerm<'a>,
        predicate: &SimpleTerm<'a>,
    ) -> anyhow::Result<Option<String>> {
        if let Some(t) = self.graph.triples_with_sp(subject, predicate).next() {
            let t = t?;
            let o = t.o();
            if let Some(iri) = o.as_iri() {
                return Ok(Some(iri.as_str().to_string()));
            } else if let Some(literal) = o.as_literal() {
                return Ok(Some(literal.value().to_string()));
            }
        }
        Ok(None)
    }

    pub fn new_bnode(&mut self) -> anyhow::Result<SimpleTerm<'a>> {
        self.graph.new_bnode()
    }

    pub fn serialize_to_turtle(&self, output_path: &Path) -> anyhow::Result<()> {
        let mut config = TurtleConfig::new().with_pretty(true);
        let mut prefix_map = config.prefix_map().to_vec();

        for (prefix, iri) in self.namespaces.namespaces.iter() {
            prefix_map.push((
                sophia_api::prefix::Prefix::new_unchecked(prefix.clone().into()),
                Iri::new_unchecked(iri.as_str().into()),
            ));
        }

        config = config.with_own_prefix_map(prefix_map);

        let mut writer =
            TurtleSerializer::new_with_config(BufWriter::new(File::create(output_path)?), config);

        writer.serialize_graph(&self.graph)?;

        Ok(())
    }
}

pub struct GraphBuilder<'a> {
    graph: RdfGraph<'a>,
}

impl<'a> GraphBuilder<'a> {
    pub fn new() -> Self {
        GraphBuilder {
            graph: RdfGraph::new(),
        }
    }

    pub fn with_namespace(mut self, prefix: &str, iri: &str) -> anyhow::Result<Self> {
        self.graph.namespaces.add_namespace(prefix, iri)?;
        Ok(self)
    }

    pub fn add_triple(mut self, subject_iri: &str, predicate_iri: &str, object_iri: &str) -> anyhow::Result<Self> {
        self.graph.add_triple(subject_iri, predicate_iri, object_iri)?;
        Ok(self)
    }

    pub fn add_literal_triple(
        mut self,
        subject_iri: &str,
        predicate_iri: &str,
        literal_value: &str,
        literal_type_iri: &str,
    ) -> anyhow::Result<Self> {
        self.graph.add_literal_triple(subject_iri, predicate_iri, literal_value, literal_type_iri)?;
        Ok(self)
    }

    pub fn build(self) -> RdfGraph<'a> {
        self.graph
    }
}
