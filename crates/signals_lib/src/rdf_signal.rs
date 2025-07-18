use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use crate::common::SignalManager;

pub type RdfSignal<'a> = SignalManager<RdfGraph<'a>>;

/// Create a new RDF signal from Turtle data
pub fn new_rdf_signal_from_turtle(turtle_data: &str) -> RdfSignal {
    let graph = RdfGraph::from_turtle_str(turtle_data).expect("Failed to parse Turtle");
    SignalManager::new(graph)
}

/// Add triples to the RDF signal from Turtle data
pub fn add_triples_from_turtle(signal: &mut RdfSignal, turtle_data: &str) {
    let mut graph = signal.get();
    let new_graph = RdfGraph::from_turtle_str(turtle_data).expect("Failed to parse Turtle");
    graph.graph.insert_all(new_graph.graph.triples()).unwrap();
    signal.set_success(graph);
}

/// Query the RDF signal for triples matching a pattern
pub fn query_triples(signal: &RdfSignal, subj: Option<&str>, pred: Option<&str>, obj: Option<&str>) -> Vec<(String, String, String)> {
    let graph = signal.get();
    let s = subj.map(|s| graph.namespaces.get_term("ex", s).unwrap());
    let p = pred.map(|p| graph.namespaces.get_term("ex", p).unwrap());
    let o = obj.map(|o| graph.namespaces.get_term("ex", o).unwrap());

    let mut results = Vec::new();
    for t in graph.graph.triples() {
        let t = t.unwrap();
        let s_str = t.s().to_string();
        let p_str = t.p().to_string();
        let o_str = t.o().to_string();

        if s.as_ref().map_or(true, |s_| s_ == t.s())
            && p.as_ref().map_or(true, |p_| p_ == t.p())
            && o.as_ref().map_or(true, |o_| o_ == t.o())
        {
            results.push((s_str, p_str, o_str));
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rdf_signal() {
        let turtle = "@prefix ex: <http://example.org/> . ex:a ex:b ex:c .";
        let mut signal = new_rdf_signal_from_turtle(turtle);
        let triples = query_triples(&signal, None, None, None);
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].0, "<http://example.org/a>");
        assert_eq!(triples[0].1, "<http://example.org/b>");
        assert_eq!(triples[0].2, "<http://example.org/c>");
        // Add another triple
        add_triples_from_turtle(&mut signal, "@prefix ex: <http://example.org/> . ex:x ex:y ex:z .");
        let triples = query_triples(&signal, None, None, None);
        assert_eq!(triples.len(), 2);
    }
}