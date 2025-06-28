use proc_macro::TokenStream;  
use quote::quote;  
//use syn::{parse_macro_input, ItemFn, Lit, Meta, Attribute};  

use syn::{parse_macro_input, ItemFn};  
  
#[proc_macro_attribute]  
pub fn mcp_component(args: TokenStream, input: TokenStream) -> TokenStream {  
    let input_fn = parse_macro_input!(input as ItemFn);  
      
    let fn_name = &input_fn.sig.ident;  
    let fn_name_str = fn_name.to_string();  
      
    // Parse MCP tool attributes using modern syn 2.0 approach  
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
      
    // Use syn::meta::parser directly with parse_macro_input!  
    let parser = syn::meta::parser(|meta| {  
        if meta.path.is_ident("menu") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_str) = value.parse::<syn::LitStr>() {  
                    menu_type = lit_str.value();  
                }  
            }  
        } else if meta.path.is_ident("label") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_str) = value.parse::<syn::LitStr>() {  
                    label = lit_str.value();  
                }  
            }  
        } else if meta.path.is_ident("emoji") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_str) = value.parse::<syn::LitStr>() {  
                    emoji = lit_str.value();  
                }  
            }  
        } else if meta.path.is_ident("description") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_str) = value.parse::<syn::LitStr>() {  
                    description = lit_str.value();  
                }  
            }  
        } else if meta.path.is_ident("visible") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_bool) = value.parse::<syn::LitBool>() {  
                    visible = lit_bool.value;  
                }  
            }  
        } else if meta.path.is_ident("order") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_int) = value.parse::<syn::LitInt>() {  
                    order = lit_int.base10_parse().unwrap_or(0);  
                }  
            }  
        } else if meta.path.is_ident("mcp") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_bool) = value.parse::<syn::LitBool>() {  
                    mcp_enabled = lit_bool.value;  
                }  
            }  
        } else if meta.path.is_ident("tool_name") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_str) = value.parse::<syn::LitStr>() {  
                    tool_name = lit_str.value();  
                }  
            }  
        } else if meta.path.is_ident("returns") {  
            if let Ok(value) = meta.value() {  
                if let Ok(lit_str) = value.parse::<syn::LitStr>() {  
                    returns = lit_str.value();  
                }  
            }  
        } else if meta.path.is_ident("params") {  
            let _ = meta.parse_nested_meta(|nested_meta| {  
                // Parse each parameter as a string literal  
                if let Ok(value) = nested_meta.value() {  
                    if let Ok(lit_str) = value.parse::<syn::LitStr>() {  
                        parameters.push(lit_str.value());  
                    }  
                }  
                Ok(())  
            });  
        }  
        Ok(())  
    });  
      
    // Parse the arguments using parse_macro_input! with the parser  
    parse_macro_input!(args with parser);  
      
    // Rest of your macro implementation...  
    let menu_info_name = syn::Ident::new(&format!("{}_MCP_INFO", fn_name.to_string().to_uppercase()), fn_name.span());  
    let mcp_handler_name = syn::Ident::new(&format!("{}_mcp_handler", fn_name), fn_name.span());  
      
    let expanded = quote! {  
        // Your existing quote! block...  
    };  
      
    TokenStream::from(expanded)  
}

