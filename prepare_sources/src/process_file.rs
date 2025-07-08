use anyhow::Result;

use crate::clifford::{BertCliffordEncoder, BertConfig, SolMultivector};
use crate::embedding;

pub fn process_code(code_content: &str) -> Result<SolMultivector> {
    let bert_embedding = embedding::embed_text(code_content)?;

    let config = BertConfig::default();
    let encoder = BertCliffordEncoder::new(config);

    let multivector = encoder.encode_embedding(&bert_embedding)?;
    Ok(multivector)
}