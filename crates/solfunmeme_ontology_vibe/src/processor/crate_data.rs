use anyhow::Result;
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_clifford::generate_multivector_from_string;

pub fn add_crate_data_internal(graph: &mut RdfGraph) -> Result<()> {
    let rdfs_label = graph.namespaces.get_term("rdfs", "label")?;
    let has_clifford_vector_iri = graph.namespaces.get_term("onto", "hasCliffordVector")?;
    let crates_root_prefix = graph.namespaces.get_base_iri("crates_root").unwrap().as_str();

    let subjects = graph.get_subjects_with_property(&rdfs_label, &graph.namespaces.get_term("rdf", "type")?)?;

    for subject in subjects {
        if subject.iri().unwrap().as_str().starts_with(crates_root_prefix) {
            if let Some(crate_name) = graph.get_property_value(&subject, &rdfs_label)? {
                let multivector = generate_multivector_from_string(&crate_name);
                let multivector_str = format!("{}", multivector);
                graph.add_literal_triple(
                    &subject,
                    &has_clifford_vector_iri,
                    &multivector_str,
                    graph.namespaces.get_base_iri("xsd").unwrap(),
                )?;
            }
        }
    }
    Ok(())
}
