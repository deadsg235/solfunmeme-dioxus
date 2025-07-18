use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_clifford::generate_multivector_from_string;

pub fn add_emoji_data_internal(graph: &mut RdfGraph) -> Result<()> {
    let type_prop = graph.namespaces.get_term("rdf", "type")?;
    let emoji_class = graph.namespaces.get_term("em", "Emoji")?;
    let has_clifford_vector_iri = graph.namespaces.get_term("onto", "hasCliffordVector")?;

    let subjects = graph.get_subjects_with_property(&type_prop, &emoji_class)?;

    for subject in subjects {
        let emoji_name = subject.iri().unwrap().as_str().split('#').last().unwrap_or("").to_string();
        let multivector = generate_multivector_from_string(&emoji_name);
        let multivector_str = format!("{}", multivector);
        graph.add_literal_triple(
            &subject,
            &has_clifford_vector_iri,
            &multivector_str,
            graph.namespaces.get_base_iri("xsd").unwrap(),
        )?;
    }
    Ok(())
}
