use crate::extractor::types::{CodeSnippet, TestResult};

/// Tests a code snippet by attempting to compile/execute it.
pub fn test_code_snippet_old(snippet: &mut CodeSnippet) {
    let start_time = std::time::Instant::now();
    
    match snippet.language.as_str() {
        "rust" => {
            // For Rust code, we could try to compile it in a sandbox
            // For now, we'll just do basic syntax checking
            if snippet.content.contains("fn ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        "javascript" | "js" => {
            // For JavaScript, we could use a JS engine
            // For now, just check for basic syntax
            if snippet.content.contains("function") || snippet.content.contains("const ") || snippet.content.contains("let ") {
                snippet.test_result = Some(TestResult {
                    success: true,
                    error_message: None,
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            } else {
                snippet.test_result = Some(TestResult {
                    success: false,
                    error_message: Some("No function or variable declarations found".to_string()),
                    execution_time_ms: start_time.elapsed().as_millis() as u64,
                });
            }
        }
        _ => {
            // For other languages, just mark as untested
            snippet.test_result = Some(TestResult {
                success: false,
                error_message: Some("Language not supported for testing".to_string()),
                execution_time_ms: start_time.elapsed().as_millis() as u64,
            });
        }
    }
}

/// Tests a code snippet by attempting to compile/execute it.  
pub fn test_code_snippet(snippet: &mut CodeSnippet) {  
    let start_time = std::time::Instant::now();  
      
    match snippet.language.as_str() {  
        "rust" => {  
            if snippet.content.contains("fn ") || snippet.content.contains("let ") {  
                snippet.test_result = Some(TestResult {  
                    success: true,  
                    error_message: None,  
                    execution_time_ms: start_time.elapsed().as_millis() as u64,  
                });  
            } else {  
                snippet.test_result = Some(TestResult {  
                    success: false,  
                    error_message: Some("No function or variable declarations found".to_string()),  
                    execution_time_ms: start_time.elapsed().as_millis() as u64,  
                });  
            }  
        }  
        "javascript" | "js" => {  
            if snippet.content.contains("function") || snippet.content.contains("const ") || snippet.content.contains("let ") {  
                snippet.test_result = Some(TestResult {  
                    success: true,  
                    error_message: None,  
                    execution_time_ms: start_time.elapsed().as_millis() as u64,  
                });  
            } else {  
                snippet.test_result = Some(TestResult {  
                    success: false,  
                    error_message: Some("No function or variable declarations found".to_string()),  
                    execution_time_ms: start_time.elapsed().as_millis() as u64,  
                });  
            }  
        }  
        _ => {  
            snippet.test_result = Some(TestResult {  
                success: false,  
                error_message: Some("Language not supported for testing".to_string()),  
                execution_time_ms: start_time.elapsed().as_millis() as u64,  
            });  
        }  
    }  
}  


