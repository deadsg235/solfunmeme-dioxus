use linfa::prelude::*;
//use linfa_preprocessing::PrincipalComponentAnalysis;
use linfa_reduction::Pca;
use ndarray::{array, Array2};
use serde::{Serialize, Deserialize};
use ndarray::Array1;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ThemeNode {
    pub emoji: String,
    pub color: String,
    pub dim_values: [f64; 8],
}

/// Reduce 768D BERT embeddings to 8D ThemeNodes using PCA.
pub fn reduce_bert_to_8d(embeddings: Array2<f64>) -> Vec<ThemeNode> {
    //let pca = Pca::fit(&embeddings).unwrap().with_components(8);


    
    //let dataset = DatasetBase::new(  
    //	embeddings.clone(),  
//	Array1::default(embeddings.nrows()) // empty targets  
    //    );
    let dataset = DatasetBase::new(    
	embeddings.clone(),    
	Array1::<f64>::zeros(embeddings.nrows()) // create zero-filled array  
    );
    //let dataset = Dataset::from(embeddings.clone()); // or embeddings.view() if you want to avoid cloning  
    let pca = Pca::params(8).fit(&dataset).unwrap();
    
    //let pca = Pca::params(8).fit(&embeddings).unwrap();
    //let reduced_8d = pca.transform(&embeddings);
    let reduced_8d = pca.transform(dataset);

    let emojis = vec!["🚀", "📜", "🔍", "💬", "🔀", "💡", "💭", "🔑"];
    let colors = vec![
        "rgba(255, 0, 0, 0.8)", "rgba(255, 255, 0, 0.8)", "rgba(0, 255, 255, 0.8)",
        "rgba(255, 0, 255, 0.8)", "rgba(0, 255, 0, 0.8)", "rgba(255, 128, 0, 0.8)",
        "rgba(128, 255, 0, 0.8)", "rgba(255, 0, 128, 0.8)",
    ];

//    reduced_8d.outer_iter()
    reduced_8d.records.outer_iter()
        .zip(emojis.iter().cycle().zip(colors.iter().cycle()))
        .map(|(row, (emoji, color))| ThemeNode {
            emoji: emoji.to_string(),
            color: color.to_string(),
            dim_values: row.to_vec().try_into().unwrap(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_reduce_bert_to_8d_output_length() {
        let embeddings = array![
            [0.1; 768],
            [0.2; 768],
            [0.3; 768],
            [0.4; 768],
            [0.5; 768],
            [0.6; 768],
            [0.7; 768],
            [0.8; 768],
        ];
        let nodes = reduce_bert_to_8d(embeddings);
        assert_eq!(nodes.len(), 8);
        for node in nodes {
            assert_eq!(node.dim_values.len(), 8);
        }
    }

    #[test]
    fn test_theme_node_fields() {
        let embeddings = array![
            [0.1; 768],
            [0.2; 768],
        ];
        let nodes = reduce_bert_to_8d(embeddings);
        assert_eq!(nodes[0].emoji, "🚀");
        assert_eq!(nodes[1].emoji, "📜");
        assert_eq!(nodes[0].color, "rgba(255, 0, 0, 0.8)");
        assert_eq!(nodes[1].color, "rgba(255, 255, 0, 0.8)");
    }
} 
