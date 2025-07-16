use serde::{Deserialize, Serialize};

pub mod wallet_manager;
pub use wallet_manager::{WalletCredentials, SecretStore, EncryptedSecret, WalletManager};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageVector {
    pub cost_per_token_input: f64,
    pub cost_per_token_output: f64,
    pub rate_limit_per_minute: u32,
    pub available_credits: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmAccount {
    pub id: String,
    pub api_key: Option<String>,
    pub usage_vector: UsageVector,
    pub command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProvider {
    pub name: String,
    pub accounts: Vec<LlmAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CliffordOperationRequest {
    pub operation: String,
    #[serde(default)]
    pub scalar_value: f32,
    #[serde(default)]
    pub vector_values: Vec<f32>,
    #[serde(default)]
    pub input_multivector: Option<SolMultivector>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CliffordOperationResponse {
    pub result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

use solfunmeme_clifford::SolMultivector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeReflectionTask {
    pub code_chunks: Vec<String>,
    pub multivector: SolMultivector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LlmTaskPayload {
    CodeReflection(CodeReflectionTask),
    CliffordOperation(CliffordOperationRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmTaskGroup {
    pub task_type: String, // Still useful for high-level categorization
    pub payload: LlmTaskPayload,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}