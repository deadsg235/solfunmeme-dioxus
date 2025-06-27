// use std::pin::Pin;
// use dioxus::prelude::*;
// use std::sync::Arc;
// use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
//use gloo_timers::future::TimeoutFuture;
//use log::error;
// #[cfg(target_arch = "wasm32")]
// use {
//     web_sys::{window, Blob, BlobPropertyBag, Url, HtmlAnchorElement},
//     wasm_bindgen::JsCast,
//     js_sys::Array,
// };

pub mod styles;
pub mod types;
pub mod utils;
pub mod components;
pub mod model;
pub mod system;

// use crate::extractor::styles::STYLE;
// use crate::extractor::types::{CodeSnippet, ExtractedFile, ProcessingFile};
// use crate::extractor::utils::{extract_code_snippets, create_download_filename};
// //use crate::extractor::components::{
// //    FileUploadArea, ProcessingIndicator, FileResults, WelcomeMessage
// //};
// use dioxus_clipboard::prelude::use_clipboard;
// use dioxus_clipboard::prelude::UseClipboard;



