use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use libloading::{Library, Symbol};
use solfunmeme_lean4::OntologyResolver as Lean4OntologyResolver;
use solfunmeme_rdf_utils::sophia_api::graph::Graph;
use solfunmeme_rdf_utils::sophia_inmem::graph::FastGraph;
use solfunmeme_rdf_utils::sophia_parser::turtle;
use solfunmeme_rdf_utils::sophia_term::{SimpleTerm, Term};

/// Represents a generic semiotic entity loaded from the ontology.
/// This can be a task, a concept, or eventually, a Tarot card,
/// identified by its unique URI.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SemioticEntity {
    pub uri: String,
    pub label: String, // The human-readable part of the URI, e.g., "task"
    pub emoji: String,
    pub category: String,
}

/// A trait for dynamically loaded meme objects.
pub trait ExecutableMeme: Send + Sync {
    fn execute(&self) -> String;
    fn get_name(&self) -> &str;
}

/// Represents the binding of a SemioticEntity to a dynamic function.
#[derive(Debug, Clone)]
pub struct FunctionBinding {
    pub entity: SemioticEntity,
    pub library_path: String,
    pub function_symbol: String,
}

/// Manages the ontology and the bindings between entities and functions.
#[derive(Debug)]
pub struct TarotEngine {
    entities: HashMap<String, SemioticEntity>, // Maps URI to Entity
    bindings: HashMap<String, FunctionBinding>, // Maps URI to FunctionBinding
    lean4_resolver: Option<Lean4OntologyResolver>,
}

impl TarotEngine {
    /// Loads entities from a Turtle ontology file.
    pub fn from_ontology(path: &Path, lean4_resolver: Option<Lean4OntologyResolver>) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let graph: sophia::inmem::graph::FastGraph = turtle::parse_bufread(reader).collect_quads()?.collect_graph()?;

        let mut entities = HashMap::new();
        let concept_type_uri = "http://example.org/emoji#Concept";
        let rdf_type_uri = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
        let emoji_prop_uri = "http://example.org/emoji#emoji";
        let category_prop_uri = "http://example.org/emoji#category";

        let concept_term = SimpleTerm::new_iri_unchecked(concept_type_uri);
        let type_term = SimpleTerm::new_iri_unchecked(rdf_type_uri);

        let subjects = graph.subjects().cloned().collect::<Vec<_>>();

        for subject in subjects {
            if graph.triples_with_sp(&subject, &type_term).any(|t| t.map_or(false, |t| t.o() == &concept_term)) {
                let uri = subject.value().to_string();
                let label = uri.split('#').last().unwrap_or("").to_string();

                let mut emoji = get_property(&graph, &subject, emoji_prop_uri)?.unwrap_or_default();
                let category = get_property(&graph, &subject, category_prop_uri)?.unwrap_or_default();

                // If a Lean4 resolver is provided and the URI is a Lean4 concept, use its emoji
                if let Some(resolver) = &lean4_resolver {
                    if uri.starts_with("http://example.org/lean4_code#") {
                        if let Some(lean4_emoji) = resolver.resolve_emoji(&uri) {
                            emoji = lean4_emoji.clone();
                        }
                    }
                }

                let entity = SemioticEntity {
                    uri: uri.clone(),
                    label,
                    emoji,
                    category,
                };
                entities.insert(uri, entity);
            }
        }

        Ok(TarotEngine {
            entities,
            bindings: HashMap::new(),
            lean4_resolver,
        })
    }

    /// Binds a function to a semiotic entity by its URI.
    pub fn add_binding(&mut self, uri: &str, library_path: &str, function_symbol: &str) -> Result<(), String> {
        if let Some(entity) = self.entities.get(uri) {
            let binding = FunctionBinding {
                entity: entity.clone(),
                library_path: library_path.to_string(),
                function_symbol: function_symbol.to_string(),
            };
            self.bindings.insert(uri.to_string(), binding);
            Ok(())
        } else {
            Err(format!("Entity with URI '{}' not found in ontology.", uri))
        }
    }

    pub fn get_entity(&self, uri: &str) -> Option<&SemioticEntity> {
        self.entities.get(uri)
    }

    pub fn get_binding(&self, uri: &str) -> Option<&FunctionBinding> {
        self.bindings.get(uri)
    }

    pub fn all_entities(&self) -> Vec<&SemioticEntity> {
        self.entities.values().collect()
    }

    /// Executes the function bound to a semiotic entity, returning an ExecutableMeme object.
    pub fn execute_binding(&self, uri: &str) -> Result<Box<dyn ExecutableMeme>, Box<dyn Error>> {
        if let Some(binding) = self.get_binding(uri) {
            let lib = Library::open(&binding.library_path)?;
            let func: Symbol<unsafe extern "C" fn() -> Box<dyn ExecutableMeme>> = unsafe { lib.symbol(&binding.function_symbol) }?;
            
            println!("Executing function for entity: {:?}", binding.entity);
            let result = unsafe { func() };
            Ok(result)
        } else {
            Err(format!("No binding found for URI '{}'", uri).into())
        }
    }
}

fn get_property(graph: &sophia::inmem::graph::FastGraph, subject: &SimpleTerm, property_uri: &str) -> Result<Option<String>, Box<dyn Error>> {
    let prop_term = SimpleTerm::new_iri_unchecked(property_uri);
    if let Some(triple_res) = graph.triples_with_sp(subject, &prop_term).next() {
        let triple = triple_res?;
        return Ok(Some(triple.o().value().to_string()));
    }
    Ok(None)
}
