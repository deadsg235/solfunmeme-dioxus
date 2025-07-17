use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use solfunmeme_rdf_utils::sophia_api::graph::Graph;
use solfunmeme_rdf_utils::sophia_inmem::graph::FastGraph;
use solfunmeme_rdf_utils::sophia_parser::turtle;
use solfunmeme_rdf_utils::sophia_term::{SimpleTerm, Term};

pub struct OntologyResolver {
    graph: FastGraph,
    uri_to_emoji: HashMap<String, String>,
}

impl OntologyResolver {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let graph: FastGraph = turtle::parse_bufread(reader).collect_quads()?.collect_graph()?;

        let mut uri_to_emoji = HashMap::new();
        let emoji_prop_uri = SimpleTerm::new_iri_unchecked("http://example.org/emoji#hasEmojiRepresentation");

        for triple in graph.triples() {
            let triple = triple?;
            if triple.p() == &emoji_prop_uri {
                uri_to_emoji.insert(triple.s().value().to_string(), triple.o().value().to_string());
            }
        }

        Ok(OntologyResolver { graph, uri_to_emoji })
    }

    pub fn resolve_emoji(&self, concept_uri: &str) -> Option<&String> {
        self.uri_to_emoji.get(concept_uri)
    }
}
