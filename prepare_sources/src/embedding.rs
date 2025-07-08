use anyhow::Result;
use candle_core::{Device, Tensor, DType};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config as BertConfig};
use hf_hub::{api::sync::Api, Repo, RepoType};
use serde_json;

pub fn embed_text(text: &str) -> Result<Vec<f32>> {
    let api = Api::new()?;
    let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let revision = "main".to_string();

    let repo = api.repo(Repo::with_revision(model_id, RepoType::Model, revision));
    let config_filename = repo.get("config.json")?;
    let _vocab_filename = repo.get("tokenizer.json")?;
    let weights_filename = repo.get("model.safetensors")?;

    let device = Device::Cpu;
    let config = std::fs::read_to_string(config_filename)?;
    let config: BertConfig = serde_json::from_str(&config)?;
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DType::F32, &device)? };
    let model = BertModel::load(vb, &config)?;

    // Tokenizer (simplified for demonstration)
    // In a real application, you'd load a proper tokenizer.
    let tokens = text.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    let token_ids = tokens.iter().map(|_| 0u32).collect::<Vec<u32>>(); // Placeholder

    let input_ids = Tensor::new(&token_ids[..], &device)?.unsqueeze(0)?;
    let token_type_ids = input_ids.zeros_like()?; // Placeholder

    let embeddings = model.forward(&input_ids, &token_type_ids, None)?;

    // Pool the embeddings (e.g., mean pooling)
    let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
    let embeddings = (embeddings.sum_keepdim(1)? / (n_tokens as f64))?;
    let embeddings = embeddings.squeeze(0)?.flatten_all()?;

    Ok(embeddings.to_vec1()?)
}