use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::namespace_manager::NamespaceManager;

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

pub fn process_analyzed_function<'a>(
    graph: &mut RdfGraph<'a>,
    func: AnalyzedFunction,
    ns: &NamespaceManager<'a>,
) -> anyhow::Result<()> {
    let func_iri = ns.get_term("ex", &func.function_name)?;
    let type_pred = ns.get_term("rdf", "type")?;
    let func_class = ns.get_term("ex", "Function")?;
    graph.add_triple(&func_iri, &type_pred, &func_class)?;

    let file_path_pred = ns.get_term("ex", "filePath")?;
    graph.add_literal_triple(
        &func_iri,
        &file_path_pred,
        &func.file_path,
        ns.get_base_iri("xsd").unwrap(),
    )?;

    let code_snippet_pred = ns.get_term("ex", "codeSnippet")?;
    graph.add_literal_triple(
        &func_iri,
        &code_snippet_pred,
        &func.code_snippet,
        ns.get_base_iri("xsd").unwrap(),
    )?;

    let semantic_summary_pred = ns.get_term("ex", "semanticSummary")?;
    graph.add_literal_triple(
        &func_iri,
        &semantic_summary_pred,
        &func.semantic_summary,
        ns.get_base_iri("xsd").unwrap(),
    )?;

    let multivector_pred = ns.get_term("ex", "multivector")?;
    graph.add_literal_triple(
        &func_iri,
        &multivector_pred,
        &func.multivector_str,
        ns.get_base_iri("xsd").unwrap(),
    )?;

    let sieve_address_pred = ns.get_term("ex", "sieveAddress")?;
    graph.add_literal_triple(
        &func_iri,
        &sieve_address_pred,
        &func.sieve_address,
        ns.get_base_iri("xsd").unwrap(),
    )?;

    for emoji_info in func.closest_emojis {
        let emoji_pred = ns.get_term("em", &emoji_info.emoji)?;
        let emoji_dist_pred = ns.get_term("ex", "emojiDistance")?;
        let emoji_cat_pred = ns.get_term("ex", "emojiCategory")?;

        let emoji_bnode = graph.new_bnode()?;
        graph.add_triple(&func_iri, &emoji_pred, &emoji_bnode)?;

        graph.add_literal_triple(
            &emoji_bnode,
            &emoji_dist_pred,
            &emoji_info.distance.to_string(),
            ns.get_base_iri("xsd").unwrap(),
        )?;

        graph.add_literal_triple(
            &emoji_bnode,
            &emoji_cat_pred,
            &emoji_info.category,
            ns.get_base_iri("xsd").unwrap(),
        )?;
    }

    Ok(())
}

pub fn process_analyzed_token<'a>(
    graph: &mut RdfGraph<'a>,
    token_data: AnalyzedToken,
    ns: &NamespaceManager<'a>,
) -> anyhow::Result<()> {
    let token_iri = ns.get_term("ex", &token_data.token)?;
    let type_pred = ns.get_term("rdf", "type")?;
    let token_class = ns.get_term("ex", "Token")?;
    graph.add_triple(&token_iri, &type_pred, &token_class)?;

    let count_pred = ns.get_term("ex", "count")?;
    graph.add_literal_triple(
        &token_iri,
        &count_pred,
        &token_data.count.to_string(),
        ns.get_base_iri("xsd").unwrap(),
    )?;

    let multivector_pred = ns.get_term("ex", "multivector")?;
    graph.add_literal_triple(
        &token_iri,
        &multivector_pred,
        &token_data.multivector_str,
        ns.get_base_iri("xsd").unwrap(),
    )?;

    Ok(())
}