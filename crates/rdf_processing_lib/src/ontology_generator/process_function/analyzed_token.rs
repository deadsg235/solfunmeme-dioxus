use crate::ontology_generator::namespaces::Namespaces;
use sophia::api::graph::MutableGraph;
use sophia_api::prelude::{IriRef, Term};
use sophia_api::term::SimpleTerm;

pub struct AnalyzedToken {
    pub token: String,
    pub count: u32,
    pub multivector_str: String,
    pub orbital_path: Option<Vec<(f64, f64)>>,
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
