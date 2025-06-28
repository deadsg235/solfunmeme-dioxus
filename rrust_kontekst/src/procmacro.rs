// lib.rs - Enhanced procedural macro for MCP integration
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn mcp_component(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input_fn = parse_macro_input!(input as ItemFn);
    
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    
    // Parse MCP tool attributes
    let mut menu_type = "core".to_string();
    let mut label = fn_name_str.clone();
    let mut emoji = "".to_string();
    let mut description = "".to_string();
    let mut visible = true;
    let mut order = 0;
    let mut mcp_enabled = true;
    let mut tool_name = fn_name_str.to_lowercase().replace("component", "");
    let mut parameters = Vec::new();
    let mut returns = "UI component rendering".to_string();
    
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                match nv.path.get_ident().map(|i| i.to_string()).as_deref() {
                    Some("menu") => if let Lit::Str(s) = nv.lit { menu_type = s.value(); },
                    Some("label") => if let Lit::Str(s) = nv.lit { label = s.value(); },
                    Some("emoji") => if let Lit::Str(s) = nv.lit { emoji = s.value(); },
                    Some("description") => if let Lit::Str(s) = nv.lit { description = s.value(); },
                    Some("visible") => if let Lit::Bool(b) = nv.lit { visible = b.value; },
                    Some("order") => if let Lit::Int(i) = nv.lit { order = i.base10_parse().unwrap_or(0); },
                    Some("mcp") => if let Lit::Bool(b) = nv.lit { mcp_enabled = b.value; },
                    Some("tool_name") => if let Lit::Str(s) = nv.lit { tool_name = s.value(); },
                    Some("returns") => if let Lit::Str(s) = nv.lit { returns = s.value(); },
                    _ => {}
                }
            }
            NestedMeta::Meta(Meta::List(list)) if list.path.is_ident("params") => {
                // Parse parameter definitions for MCP schema
                for nested in list.nested {
                    if let NestedMeta::Lit(Lit::Str(param)) = nested {
                        parameters.push(param.value());
                    }
                }
            }
            _ => {}
        }
    }
    
    let menu_info_name = syn::Ident::new(&format!("{}_MCP_INFO", fn_name.to_string().to_uppercase()), fn_name.span());
    let mcp_handler_name = syn::Ident::new(&format!("{}_mcp_handler", fn_name), fn_name.span());
    
    let expanded = quote! {
        // Generate MCP tool metadata
        pub static #menu_info_name: crate::mcp::McpToolInfo = crate::mcp::McpToolInfo {
            component_name: #fn_name_str,
            tool_name: #tool_name,
            menu_type: #menu_type,
            label: #label,
            emoji: #emoji,
            description: #description,
            visible: #visible,
            order: #order,
            mcp_enabled: #mcp_enabled,
            parameters: &[#(#parameters),*],
            returns: #returns,
        };
        
        // Original component with dioxus::component attribute
        #[dioxus::prelude::component]
        #input_fn
        
        // Generate MCP tool handler
        #[allow(non_snake_case)]
        pub async fn #mcp_handler_name(params: serde_json::Value) -> Result<serde_json::Value, crate::mcp::McpError> {
            // This would invoke the component and return its state/data
            // For now, return metadata about the tool
            Ok(serde_json::json!({
                "tool": #tool_name,
                "description": #description,
                "invoked": true,
                "ui_component": #fn_name_str
            }))
        }
        
        // Auto-register the MCP tool
        #[ctor::ctor]
        fn #fn_name() {
            crate::mcp::register_mcp_tool(&#menu_info_name, #mcp_handler_name);
        }
    };
    
    TokenStream::from(expanded)
}

// mcp.rs - MCP (Model Context Protocol) integration
use std::collections::HashMap;
use std::sync::OnceLock;
use std::future::Future;
use std::pin::Pin;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct McpToolInfo {
    pub component_name: &'static str,
    pub tool_name: &'static str,
    pub menu_type: &'static str,
    pub label: &'static str,
    pub emoji: &'static str,
    pub description: &'static str,
    pub visible: bool,
    pub order: i32,
    pub mcp_enabled: bool,
    pub parameters: &'static [&'static str],
    pub returns: &'static str,
}

#[derive(Debug)]
pub enum McpError {
    InvalidParams,
    ExecutionError(String),
    NotFound,
}

type McpHandler = fn(Value) -> Pin<Box<dyn Future<Output = Result<Value, McpError>> + Send>>;

static MCP_REGISTRY: OnceLock<HashMap<&'static str, (McpToolInfo, McpHandler)>> = OnceLock::new();

pub fn register_mcp_tool(info: &'static McpToolInfo, handler: McpHandler) {
    // In a real implementation, this would be thread-safe
    println!("Registering MCP tool: {} -> {}", info.tool_name, info.description);
}

// Generate MCP tools schema for AI
pub fn get_mcp_tools_schema() -> Value {
    let tools: Vec<Value> = get_mcp_tools("core").into_iter().map(|tool| {
        serde_json::json!({
            "name": tool.tool_name,
            "description": format!("{} {}", tool.emoji, tool.description),
            "inputSchema": {
                "type": "object",
                "properties": tool.parameters.iter().map(|p| {
                    (p.to_string(), serde_json::json!({"type": "string", "description": p}))
                }).collect::<serde_json::Map<String, Value>>()
            }
        })
    }).collect();
    
    serde_json::json!({
        "tools": tools
    })
}

pub fn get_mcp_tools(menu_type: &str) -> Vec<McpToolInfo> {
    // Return available MCP tools for the AI to discover
    vec![] // Simplified - would read from registry
}

pub async fn invoke_mcp_tool(tool_name: &str, params: Value) -> Result<Value, McpError> {
    // AI calls this to invoke tools
    match tool_name {
        "embedding" => {
            // Invoke embedding operations
            Ok(serde_json::json!({"result": "embedding_processed"}))
        }
        "bert_test" => {
            // Run BERT inference
            Ok(serde_json::json!({"result": "bert_inference_complete"}))
        }
        _ => Err(McpError::NotFound)
    }
}

// Example usage with enhanced MCP attributes:
/*
#[mcp_component(
    menu = "core",
    label = "Embedding Operations", 
    emoji = "🔗",
    description = "Perform vector similarity search and embedding operations on text data",
    tool_name = "embedding_ops",
    params = ["query: text to embed", "similarity_threshold: float between 0-1"],
    returns = "similarity scores and matched documents",
    mcp = true,
    order = 1
)]
pub fn EmbeddingComponent() -> Element {
    rsx! {
        div { 
            h2 { "🔗 Embedding Operations" }
            p { "AI can invoke embedding similarity search through MCP" }
        }
    }
}

#[mcp_component(
    menu = "core",
    label = "BERT Inference",
    emoji = "🧠", 
    description = "Run BERT model inference on text for classification and analysis",
    tool_name = "bert_inference",
    params = ["text: input text to analyze", "model: bert model variant"],
    returns = "classification results and confidence scores",
    mcp = true,
    order = 2
)]
pub fn BertTestComponent() -> Element {
    rsx! {
        div { 
            h2 { "🧠 BERT Inference" }
            p { "AI can run BERT analysis through MCP" }
        }
    }
}

#[mcp_component(
    menu = "core",
    label = "Data Encryption",
    emoji = "🔐",
    description = "Encrypt and decrypt sensitive data using various algorithms",
    tool_name = "crypto_ops",
    params = ["data: text or binary to encrypt", "algorithm: encryption method"],
    returns = "encrypted data or decryption result",
    mcp = true,
    order = 5
)]
pub fn EncryptionComponent() -> Element {
    rsx! {
        div { 
            h2 { "🔐 Encryption Tools" }
            p { "AI can perform crypto operations through MCP" }
        }
    }
}

// UI-only component (not exposed to AI)
#[mcp_component(
    menu = "core",
    label = "Settings Panel",
    emoji = "⚙️",
    description = "Application configuration and settings",
    mcp = false  // Not available to AI
)]
pub fn SettingsComponent() -> Element {
    rsx! {
        div { "Settings UI only" }
    }
}
*/
