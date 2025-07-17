use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::sieve::{BertCliffordEncoder, SieveAddress};
use solfunmeme_function_analysis::{AnalyzedFunction, ClosestEmojiInfo};

pub fn embed_text(text: &str) -> Result<Vec<f32>> {
    // This is a stub for embedding text.
    // In a real scenario, this would call an actual embedding model.
    // For now, it returns a dummy vector.
    Ok(vec![0.0; 384])
}

pub fn calculate_closest_emoji(
    embedding: &[f32],
    emoji_multivectors: &HashMap<String, (Vec<f32>, SieveAddress, String)>,
) -> ClosestEmojiInfo {
    let mut closest_emoji = String::new();
    let mut closest_category = String::from("Unclassified");
    let mut min_distance = f32::MAX;

    for (emoji, (mv_embedding, _, category)) in emoji_multivectors {
        let distance = calculate_euclidean_distance(embedding, mv_embedding);
        if distance < min_distance {
            min_distance = distance;
            closest_emoji = emoji.clone();
            closest_category = category.clone();
        }
    }

    ClosestEmojiInfo {
        emoji: closest_emoji,
        category: closest_category,
        distance: min_distance,
    }
}

fn calculate_euclidean_distance(v1: &[f32], v2: &[f32]) -> f32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

pub fn get_bert_embedding_for_function(
    func: &AnalyzedFunction,
    emoji_multivectors: &HashMap<String, (Vec<f32>, SieveAddress, String)>,
) -> Result<ClosestEmojiInfo> {
    let embedding = embed_text(&func.semantic_summary)?;
    Ok(calculate_closest_emoji(&embedding, emoji_multivectors))
}

pub fn get_bert_embedding_for_text(
    text: &str,
    emoji_multivectors: &HashMap<String, (Vec<f32>, SieveAddress, String)>,
) -> Result<ClosestEmojiInfo> {
    let embedding = embed_text(text)?;
    Ok(calculate_closest_emoji(&embedding, emoji_multivectors))
}
