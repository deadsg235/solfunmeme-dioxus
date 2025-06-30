// lib.rs - Complete usage example
//pub mod mcp;
//pub mod r#macro;

// Re-export for convenience
//pub use mcp::*;
//pub use r#macro::mcp_component;
use serde_json::json;
//use serde::ser::Error;
use std::error::Error;

//use rrust_kontekst::*;
use rrust_kontekst::mcp_component;
use rrust_kontekst_base::invoke_mcp_tool;
use rrust_kontekst_base::get_mcp_tools_schema;
use rrust_kontekst_base::McpError;


// mod Comp0 {

//     pub async fn embedding_component() -> Result < String, Box < dyn std :: error
// 								 :: Error > > { Ok("Test component executed successfully".to_string()) } fn
// 	embedding_component_mcp_handler(params : serde_json :: Value) -> std :: pin ::
//     Pin < Box < dyn std :: future :: Future < Output = Result < serde_json ::
// 								Value, rrust_kontekst_base :: McpError >> + Send >>
//     {
// 	Box ::
// 	pin(async move
// 	    {
// 		match embedding_component().await
// 		{
// 		    Ok(result) =>
// 		    {
// 			Ok(serde_json :: json!
// 			   ({
// 			       "content" :
// 			       [{
// 				   "type" : "text", "text" : format!
// 				       ("Component '{}' executed successfully",
// 					"Embedding Generator")
// 			       }], "result" : result, "isError" : false
// 			   }))
// 		    }, Err(e) =>
// 		    {
// 			Err(rrust_kontekst_base :: McpError ::
// 			    ExecutionError(format!
// 					   ("Component '{}' failed: {}", "Embedding Generator", e)))
// 		    }
// 		}
// 	    })
//     } #[allow(non_snake_case)] fn register_embedding_component()
//     {
// 	use rrust_kontekst_base :: { McpToolInfo, register_mcp_tool }; let
// 	    tool_info = McpToolInfo
// 	{
//             component_name : "embedding_component", tool_name : "embedding_",
//             menu_type : "core", label : "Embedding Generator", emoji : "🔤",
//             description : "Generate embeddings for text input", visible : true,
//             order : 1i32, mcp_enabled : true, parameters : & [], returns :
//             "Operation completed",
// 	}; if let Err(e) =
// 	    register_mcp_tool(& tool_info, embedding_component_mcp_handler)
// 	{ eprintln! ("Failed to register MCP tool '{}': {}", "embedding_", e); }
//     } #[allow(non_upper_case_globals)] pub static EMBEDDING_COMPONENT_METADATA :
//     std :: sync :: OnceLock < serde_json :: Value > = std :: sync :: OnceLock ::
//     new(); pub fn get_metadata() -> & 'static serde_json :: Value
//     {
// 	EMBEDDING_COMPONENT_METADATA.get_or_init(||
// 						 {
// 						     serde_json :: json!
// 							 ({
// 							     "component_name" : "embedding_component", "tool_name" :
// 							     "embedding_", "description" :
// 							     "Generate embeddings for text input", "function_signature" :
// 							     stringify!
// 								 (pub async fn embedding_component() -> Result < String, Box < dyn
// 								  std :: error :: Error > >
// 								  { Ok("Test component executed successfully".to_string()) }),
// 							     "generated_at_compile_time" : true
// 							 })
// 						 })
//     }
        
// }

mod Comp1 {
    //use rrust_kontekst::mcp_component;
// Example 1: Simple component
#[rrust_kontekst::mcp_component(
    menu = "core",
    label = "Embedding Generator",
    emoji = "🔤",
    description = "Generate embeddings for text input",
    order = 1
)]
    // pub async fn embedding_component( name: & str, name2: & str)  -> Result<String, Box<dyn std::error::Error>> {
    pub async fn embedding_component()  -> Result<String, Box<dyn std::error::Error>> {
	Ok("Test component executed successfully".to_string())
}
}

mod Comp2{
    use rrust_kontekst::mcp_component;
// Example 2: Component with custom configuration
#[mcp_component(
    menu = "ai_models",
    label = "BERT Inference",
    emoji = "🤖", 
    description = "Run BERT model inference on input text",
    tool_name = "bert_inference",
    visible = true,
    order = 2,
    returns = "BERT model predictions and confidence scores"
)]
    pub async fn bert_test_component()     -> Result<String, Box<dyn std::error::Error>> {
	Ok("Test component executed successfully".to_string())
    }

}

mod Comp4{
use rrust_kontekst::mcp_component;
// Example 3: Disabled component (for development)
#[mcp_component(
    menu = "experimental",
    label = "Experimental Feature",
    emoji = "⚗️",
    description = "Experimental feature under development", 
    visible = false,
    mcp = false
)]
    pub async fn experimental_component()  -> Result<String, Box<dyn std::error::Error>> {
	Ok("Test component executed successfully".to_string())
    }

}

mod Comp5 {
    use rrust_kontekst::mcp_component;
// Example 4: High-priority component
#[mcp_component(
    menu = "core",
    label = "Text Processor",
    emoji = "📝",
    description = "Process and analyze text content",
    order = -1  // Higher priority (negative numbers come first)
)]
    pub async fn text_processor_component()  -> Result<String, Box<dyn std::error::Error>> {
	Ok("Test component executed successfully".to_string())
    }
}

/// Initialize all MCP components
pub fn initialize_mcp_system() {
    println!("Initializing MCP system...");
    
    // inventory automatically calls all registration functions
    // This happens at runtime when the library is loaded
}

/// Get available tools for a specific menu
pub async fn get_tools_for_menu(menu_type: &str) -> Result<serde_json::Value, McpError> {
    get_mcp_tools_schema(menu_type)
}

/// Example of manual tool invocation
pub async fn example_tool_usage() -> Result<(), Box<dyn Error>> {
    println!("=== MCP System Demo ===");
    
    // Initialize the system
    initialize_mcp_system();
    
    // Get available tools
    match get_tools_for_menu("core").await {
        Ok(schema) => {
            println!("Available core tools: {:?}", serde_json::to_string_pretty(&schema)?);
        }
        Err(e) => {
            println!("Error getting tools: {}", e);
        }
    }
    
    // Invoke a specific tool
    match invoke_mcp_tool("embedding", json!({"text": "Hello, world!"})).await {
        Ok(result) => {
            println!("Tool result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("Tool invocation failed: {}", e);
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_component_execution() {
        // Test direct component execution
        let result = embedding_component().await;
        assert!(result.is_ok());
        
        let result = bert_test_component().await;
        assert!(result.is_ok());
        
        let json_result = result.unwrap();
        assert!(json_result.get("predictions").is_some());
    }
    
    #[tokio::test]
    async fn test_mcp_system_integration() {
        initialize_mcp_system();
        
        // Test getting tools schema
        let schema = get_tools_for_menu("core").await;
        assert!(schema.is_ok());
        
        // Test tool invocation (if tools are registered)
        let tools = list_all_tools();
        if let Ok(tool_list) = tools {
            println!("Registered tools: {:?}", tool_list);
        }
    }
    
    #[test]
    fn test_configuration_parsing() {
        // Test that the macro generates the expected code structure
        // This would be tested by ensuring the generated code compiles
        // and the registration functions are created properly
        assert!(true); // Placeholder - actual testing would involve more complex macro testing
    }
}

// // Example of how you might use this in a main application
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     println!("Starting MCP-enabled application...");
    
//     // Initialize the MCP system
//     initialize_mcp_system();
    
//     // Run the example
//     example_tool_usage().await?;
    
//     println!("Application completed successfully!");
//     Ok(())
// }
