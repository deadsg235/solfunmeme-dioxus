use std::collections::HashMap;
use sophia_api::term::Term;
use sophia_api::triple::Triple;
use sophia_inmem::graph::FastGraph;
use sophia_api::graph::Graph;

// Helper function to convert a Term to a String
fn term_to_string(term: &Term) -> String {
    match term {
        Term::Iri(iri) => iri.to_string(),
        Term::Literal(lit) => lit.value().to_string(),
        _ => "".to_string(), // Handle other term types as needed
    }
}

pub fn extract_emoji_data(graph: &FastGraph) -> (HashMap<String, (String, String)>, HashMap<String, String>) {
    let mut emoji_data: HashMap<String, (String, String)> = HashMap::new(); // emoji_char -> (concept_id, category)
    let mut concept_descriptions: HashMap<String, String> = HashMap::new(); // concept_id -> description

    let em_ns = "http://example.org/emoji#";

    for t in graph.triples() {
        let t = t.unwrap(); // Handle potential errors
        let subject = term_to_string(t.s());
        let predicate = term_to_string(t.p());
        let object = term_to_string(t.o());

        if subject.starts_with(em_ns) {
            let concept_id = subject.replace(em_ns, "");
            if predicate == format!("{}emoji", em_ns) {
                let emoji_char = object.trim_matches('"').to_string();
                let category = graph.triples()
                    .filter_map(|t2| {
                        let t2 = t2.unwrap();
                        if term_to_string(t2.s()) == subject && term_to_string(t2.p()) == format!("{}category", em_ns) {
                            Some(term_to_string(t2.o()).trim_matches('"').to_string())
                        } else {
                            None
                        }
                    })
                    .next()
                    .unwrap_or_else(|| "Unknown".to_string());
                emoji_data.insert(emoji_char, (concept_id.clone(), category));
            } else if predicate == format!("{}description", em_ns) {
                concept_descriptions.insert(concept_id, object.trim_matches('"').to_string());
            }
        }
    }
    (emoji_data, concept_descriptions)
}
