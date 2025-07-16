use anyhow::{Result, anyhow};
use std::io::{self, Read, Write};

// Import necessary types from the bootstrap crate
use bootstrap::clifford::{CliffordMultivector, Clifford};
use solfunmeme_models::{CliffordOperationRequest, CliffordOperationResponse, LlmTaskPayload};

fn main() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let payload: LlmTaskPayload = serde_json::from_str(&buffer)?;

    let response = match payload {
        LlmTaskPayload::CliffordOperation(request) => {
            match request.operation.as_str() {
                "create_scalar_multivector" => {
                    let mv = CliffordMultivector::from_scalar(request.scalar_value);
                    CliffordOperationResponse { result: format!("{:?}", mv), error: None }
                },
                "create_vector_multivector" => {
                    if request.vector_values.len() > 0 {
                        let mv = CliffordMultivector::from_vector(&request.vector_values);
                        CliffordOperationResponse { result: format!("{:?}", mv), error: None }
                    } else {
                        CliffordOperationResponse { result: String::new(), error: Some("Vector values not provided for vector multivector creation.".to_string()) }
                    }
                },
                "reverse_multivector" => {
                    // This operation would require a multivector as input, which is not yet handled
                    CliffordOperationResponse { result: String::new(), error: Some("Reverse operation not fully implemented for direct input.".to_string()) }
                }
                _ => CliffordOperationResponse { result: String::new(), error: Some(format!("Unknown operation: {}", request.operation)) },
            }
        },
        _ => CliffordOperationResponse { result: String::new(), error: Some("Unsupported task payload type.".to_string()) },
    };

    let response_json = serde_json::to_string_pretty(&response)?;
    io::stdout().write_all(response_json.as_bytes())?;

    Ok(())
}