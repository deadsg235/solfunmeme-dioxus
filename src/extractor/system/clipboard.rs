use dioxus::prelude::*;
use web_sys::window;
use gloo_timers::future::TimeoutFuture;
//use crate::extractor::CodeSnippet;
use dioxus_clipboard::prelude::*;
use std::collections::HashSet;
use wasm_bindgen::JsCast;
use crate::extractor::types::CodeSnippet;
//use crate::extractor::model::clipboard::copy_to_clipboard;
// pub fn copy_to_clipboard(text: String, snippet_id: String, copied_snippets: Signal<std::collections::HashSet<String>>) {
//     let mut copied_snippets = copied_snippets.clone();
//     wasm_bindgen_futures::spawn_local(async move {
//         if let Some(window) = window() {
//             let clipboard = window.navigator().clipboard();
//             match wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text)).await {
//                 Ok(_) => {
//                     copied_snippets.write().insert(snippet_id.clone());
//                     TimeoutFuture::new(2000).await;
//                     copied_snippets.write().remove(&snippet_id);
//                 }
//                 Err(_) => {
//                     log::error!("Failed to copy to clipboard");
//                 }
//             }
//         }
//     });
// }


// // Copy All Snippets Function

// pub fn copy_to_clipboard(text: String, snippet_id: String, mut copied_snippets: Signal<HashSet<String>>) {
//     wasm_bindgen_futures::spawn_local(async move {
//         if let Some(window) = window() {
//             if let clipboard = window.navigator().clipboard() {
//                 match wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text)).await {
//                     Ok(_) => {
//                         copied_snippets.write().insert(snippet_id.clone());
//                         TimeoutFuture::new(2000).await;
//                         copied_snippets.write().remove(&snippet_id);
//                     }
//                     Err(_) => {
//                         log::error!("Failed to copy to clipboard");
//                     }
//                 }
//             }
//         }
//     });
// }
// // ```


// // ### 3. Copy All Snippets Function
// // ```rust
pub fn copy_all_snippets_combined(snippets: Vec<CodeSnippet>, copied_snippets: Signal<HashSet<String>>) {
    let combined = snippets.iter()
        .map(|snippet| snippet.content.clone())
        .collect::<Vec<_>>()
        .join("\n\n");
    
    copy_to_clipboard(combined, "all_snippets".to_string(), copied_snippets);
}

// pub fn create_copy_handler(
//     mut clipboard: UseClipboard, 
//     copied_snippets: Signal<std::collections::HashSet<String>>
// ) -> impl FnMut(String, String) + Clone {
//     move |text: String, snippet_id: String| {
//         //#[cfg(target_arch = "wasm32")]
//         {
//             copy_to_clipboard(text, snippet_id, copied_snippets.clone());
//         }
        
//     }
// }

// //#[cfg(target_arch = "wasm32")]
// fn copy_to_clipboard_web(
//     text: String, 
//     snippet_id: String, 
//     copied_snippets: Signal<std::collections::HashSet<String>>
// ) {
//     wasm_bindgen_futures::spawn_local(async move {
//         if let Some(window) = window() {
//             if let navigator = window.navigator().clipboard() {
//                 match wasm_bindgen_futures::JsFuture::from(navigator.write_text(&text)).await {
//                     Ok(_) => {
//                         mark_as_copied(snippet_id, copied_snippets).await;
//                         return;
//                     }
//                     Err(_) => {
//                         // Fall through to fallback
// 			log::error!("Failed to copy to clipboard - Clipboard API not available");  
//                     }
//                 }
//             }
            
//             // // Fallback method
//             // if let Some(document) = window.document() {
//             //     if let Ok(textarea) = document.create_element("textarea") {
//             //         let textarea = textarea.dyn_into::<web_sys::HtmlTextAreaElement>().unwrap();
//             //         textarea.set_value(&text);
//             //         textarea.style().set_property("position", "fixed").unwrap();
//             //         textarea.style().set_property("left", "-999999px").unwrap();
//             //         textarea.style().set_property("top", "-999999px").unwrap();
//             //         let _ = document.body().unwrap().append_child(&textarea);
//             //         textarea.focus().unwrap();
//             //         textarea.select();
                    
//             //         if document.exec_command("copy").is_ok() {
//             //             mark_as_copied(snippet_id, copied_snippets).await;
//             //         } else {
//             //             log::error!("Failed to copy to clipboard");
//             //         }
                    
//             //         let _ = document.body().unwrap().remove_child(&textarea);
//             //     }
//             // }
//         }
//     });
// }

// //#[cfg(not(target_arch = "wasm32"))]
// // fn copy_to_clipboard_desktop(
// //     text: String, 
// //     snippet_id: String, 
// //     mut clipboard: UseClipboard,
// //     mut copied_snippets: Signal<std::collections::HashSet<String>>
// // ) {
// //     match clipboard.set(text) {
// //         Ok(_) => {
// //             copied_snippets.write().insert(snippet_id.clone());
// //             spawn(async move {
// //                 TimeoutFuture::new(2000).await;		
// //                 copied_snippets.write().remove(&snippet_id);
// //             });
// //         }
// //         Err(e) => {
// //             log::error!("Failed to copy to clipboard: {:?}", e);
// //         }
// //     }
// // }

// async fn mark_as_copied(
//     snippet_id: String, 
//     mut copied_snippets: Signal<std::collections::HashSet<String>>
// ) {
//     copied_snippets.write().insert(snippet_id.clone());
//     TimeoutFuture::new(2000).await;
//     copied_snippets.write().remove(&snippet_id);
// }

pub fn copy_to_clipboard(
    text: String, 
    snippet_id: String, 
    mut copied_snippets: Signal<HashSet<String>>
) {
    let mut clipboard = use_clipboard();
    
    match clipboard.set(text) {
        Ok(_) => {
            spawn(async move {
                copied_snippets.write().insert(snippet_id.clone());
                TimeoutFuture::new(2000).await;
                copied_snippets.write().remove(&snippet_id);
            });
        }
        Err(e) => {
            log::error!("Failed to copy to clipboard: {:?}", e);
        }
    }
}

// pub fn copy_to_clipboard(text: String, snippet_id: String, mut copied_snippets: Signal<HashSet<String>>) {

pub fn create_copy_handler(
    copied_snippets: Signal<HashSet<String>>
) -> impl FnMut(String, String) + Clone {
    move |text: String, snippet_id: String| {
        copy_to_clipboard(text, snippet_id, copied_snippets.clone());
    }
}
