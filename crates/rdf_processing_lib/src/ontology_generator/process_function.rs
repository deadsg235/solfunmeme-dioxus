use sophia::api::graph::MutableGraph;
use sophia_api::prelude::*;
use sophia_api::term::SimpleTerm;

use crate::ontology_generator::namespaces::Namespaces;

pub struct AnalyzedFunction {
    pub function_name: String,
    pub file_path: String,
    pub code_snippet: String,
    pub semantic_summary: String,
    pub multivector_str: String,
    pub sieve_address: String,
    pub closest_emojis: Vec<ClosestEmojiInfo>,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

pub struct ClosestEmojiInfo {
    pub emoji: String,
    pub category: String,
    pub distance: f32,
}

pub struct AnalyzedToken {
    pub token: String,
    pub count: u32,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
}

pub fn process_analyzed_function<G>(
    _graph: &mut G,
    _func: AnalyzedFunction,
    _ns: &Namespaces,
) -> anyhow::Result<()>
where
    G: MutableGraph,
    <G as MutableGraph>::MutationError: Send + Sync + 'static,
{
    Ok(())
}

pub fn process_analyzed_token<G>(
    _graph: &mut G,
    _token_data: AnalyzedToken,
    _ns: &Namespaces,
) -> anyhow::Result<()>
where
    G: MutableGraph,
    <G as MutableGraph>::MutationError: Send + Sync + 'static,
{
    Ok(())
}
