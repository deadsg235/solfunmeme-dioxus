use anyhow::Result;
use sophia::api::graph::MutableGraph;
use sophia_api::prelude::{IriRef, Term};
use sophia_api::term::SimpleTerm;

use crate::project_analyzer::AnalyzedFunction;
use super::namespaces::Namespaces;

pub fn process_analyzed_function<G>(
    graph: &mut G,
    func: AnalyzedFunction,
    ns: &Namespaces,
) -> anyhow::Result<()>
where
    G: MutableGraph,
    <G as MutableGraph>::MutationError: Send + Sync + 'static,
{
    let func_iri = SimpleTerm::Iri(IriRef::new_unchecked(format!("{}function/{}", ns.ex_iri.as_str(), func.function_name).into()));
    let file_iri = SimpleTerm::Iri(IriRef::new_unchecked(format!("{}file/{}", ns.ex_iri.as_str(), func.file_path.replace(".", "_").replace("/", "_").replace("-", "_")).into()));

    // Function triples
    graph.insert(&func_iri, &ns.rdf.iri().unwrap(), &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}Function", ns.ex_iri.as_str()).into())))?;
    graph.insert(&func_iri, &ns.rdfs.iri().unwrap().with_suffix("label").unwrap(), &func.function_name.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasCodeSnippet", ns.ex_iri.as_str()).into())), &func.code_snippet.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasSemanticSummary", ns.ex_iri.as_str()).into())), &func.semantic_summary.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasMultivectorEmbedding", ns.ex_iri.as_str()).into())), &func.multivector_str.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasSieveAddress", ns.ex_iri.as_str()).into())), &func.sieve_address.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasClosestEmoji", ns.ex_iri.as_str()).into())), &func.closest_emoji.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasEmojiCategory", ns.ex_iri.as_str()).into())), &func.emoji_category.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}hasEmojiDistance", ns.ex_iri.as_str()).into())), &func.emoji_distance.to_string().into_term())?;
    graph.insert(&func_iri, &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}isInFile", ns.ex_iri.as_str()).into())), &file_iri)?;

    // File triples
    graph.insert(&file_iri, &ns.rdf.iri().unwrap(), &SimpleTerm::Iri(IriRef::new_unchecked(format!("{}CodeFile", ns.ex_iri.as_str()).into())))?;
    graph.insert(&file_iri, &ns.rdfs.iri().unwrap().with_suffix("label").unwrap(), &func.file_path.to_string().into_term())?;

    Ok(())
}