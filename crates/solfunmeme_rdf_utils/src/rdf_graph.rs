use crate::namespace_manager::NamespaceManager;
use sophia_api::graph::{MutableGraph, Graph};
use sophia_api::serializer::TripleSerializer;
use sophia_inmem::graph::FastGraph;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use sophia_turtle::parser::turtle::TurtleParser;
use sophia_iri::Iri;
use std::path::Path;
use crate::term_factory; // Import the new term_factory
use std::io::Cursor;
use sophia_api::parser::TripleParser;

pub struct RdfGraph<'a> {
    pub graph: FastGraph,
    pub namespaces: NamespaceManager<'a>,
}

impl<'a> RdfGraph<'a> {
    pub fn new() -> Self {
        RdfGraph {
            graph: FastGraph::new(),
            namespaces: NamespaceManager::new(),
        }
    }

    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let graph: FastGraph = TurtleParser::new(Cursor::new(content.as_bytes()), Some(Iri::new("http://example.org/base/").unwrap()))
            .collect_triples::<FastGraph>()?;
        Ok(RdfGraph {
            graph,
            namespaces: NamespaceManager::new(),
        })
    }

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

    pub fn serialize_to_turtle_string(&self) -> anyhow::Result<String> {
        let mut buffer = Vec::new();
        let mut serializer = TurtleSerializer::new(&mut buffer);
        serializer.serialize_graph(&self.graph)?;
        Ok(String::from_utf8(buffer)?)
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

    pub fn with_namespace(mut self, prefix: &str, iri: &'a str) -> anyhow::Result<Self> {
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