// signals_lib/src/rdf_signal.rs
use dioxus::prelude::Signal;
use sophia::api::term::{BoxTerm, TermKind};
use sophia::api::triple::stream::TripleSource;
use sophia::parser::turtle;
use std::fs;
use std::path::Path;

/// RDF graph stored as a vector of triples
pub type RdfGraph = Vec<[BoxTerm; 3]>;
pub type RdfSignal = Signal<RdfGraph>;

/// Create a new RDF signal from a Turtle file
pub fn new_rdf_signal_from_turtle_file(path: &str) -> RdfSignal {
    let turtle_data = fs::read_to_string(path).expect("Failed to read Turtle file");
    let mut graph: RdfGraph = Vec::new();
    let triples = turtle::parse_str(&turtle_data)
        .map(|t| t.map(|t| [t.s().into(), t.p().into(), t.o().into()]))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse Turtle");
    graph.extend(triples);
    Signal::new(graph)
}

/// Add triples to the RDF signal from Turtle data
pub fn add_triples_from_turtle(signal: &mut RdfSignal, turtle_data: &str) {
    let mut graph = signal.read().clone();
    let triples = turtle::parse_str(turtle_data)
        .map(|t| t.map(|t| [t.s().into(), t.p().into(), t.o().into()]))
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse Turtle");
    graph.extend(triples);
    *signal.write() = graph;
}

/// Query the RDF signal for triples matching a pattern
pub fn query_triples(signal: &RdfSignal, subj: Option<&str>, pred: Option<&str>, obj: Option<&str>) -> Vec<(String, String, String)> {
    let graph = signal.read();
    let mut results = Vec::new();
    for t in graph.iter() {
        let s = t[0].to_string();
        let p = t[1].to_string();
        let o = t[2].to_string();
        if subj.map_or(true, |s_| s_ == s)
            && pred.map_or(true, |p_| p_ == p)
            && obj.map_or(true, |o_| o_ == o)
        {
            results.push((s, p, o));
        }
    }
    results
}

/// Query tasks from the RDF signal
pub fn query_tasks(signal: &RdfSignal) -> Vec<(String, String, String, Vec<String>)> {
    let graph = signal.read();
    let mut tasks = Vec::new();
    let tm_ns = "https://example.org/taskmanager#";
    for t in graph.iter() {
        if t[1].to_string() == format!("{}type", tm_ns) && t[2].to_string() == format!("{}Task", tm_ns) {
            let task_id = t[0].to_string();
            let mut id = String::new();
            let mut content = String::new();
            let mut status = String::new();
            let mut dependencies = Vec::new();
            for t2 in graph.iter() {
                if t2[0].to_string() == task_id {
                    match t2[1].to_string().as_str() {
                        s if s == format!("{}id", tm_ns) => id = t2[2].to_string(),
                        s if s == format!("{}content", tm_ns) => content = t2[2].to_string(),
                        s if s == format!("{}status", tm_ns) => status = t2[2].to_string(),
                        s if s == format!("{}dependsOn", tm_ns) => dependencies.push(t2[2].to_string()),
                        _ => {}
                    }
                }
            }
            tasks.push((id, content, status, dependencies));
        }
    }
    tasks
}

/// Watch a Turtle file and update the signal on changes
pub fn watch_turtle_file(cx: Scope, path: &str) -> RdfSignal {
    let signal = new_rdf_signal_from_turtle_file(path);
    let path = path.to_string();
    cx.spawn({
        async move {
            // Placeholder for file watching (requires a crate like `notify`)
            // For now, we'll assume manual reload
            // Future: Use `notify` to watch `path` and update `signal`
        }
    });
    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rdf_signal() {
        let turtle = "@prefix tm: <https://example.org/taskmanager#> . tm:task1 a tm:Task ; tm:id \"task1\" ; tm:content \"Test task\" ; tm:status \"pending\" .";
        let signal = new_rdf_signal_from_turtle_file("test.ttl");
        let tasks = query_tasks(&signal);
        assert_eq!(tasks.len(), 0); // Adjust based on actual test file content
    }
}