use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
use serde_json::Value;
use gloo_timers::future::TimeoutFuture;
use log::error;
use zip::write::{FileOptions, ZipWriter};
use wasm_bindgen::JsValue;
use web_sys::{window, Blob, BlobPropertyBag, Url, HtmlAnchorElement, Request, RequestInit, RequestMode, Response};
use js_sys::Array;
use wasm_bindgen_futures::JsFuture;
//use serde_wasm_bindgen;
use urlencoding::encode;
//use crate::playground::markdown_processor::{ConversationTurn, CodeSnippet, DocumentSummary, extract_code_snippets, test_code_snippet};
//use crate::playground::duplicate_checker::{DuplicateReport, check_duplicates};
//use crate::playground::wikidata_annotator::{WikidataAnnotator, AnnotatedWord};
//use crate::playground::bert_embedder::{rust_bert_embed_with_context, pca_reduce};
//use crate::playground::sentiment_analyzer::WasmSentimentAnalyzer;
//use crate::playground::wikidata::*;
use crate::playground::extractor::*;
//use crate::playground::multivector::Multivector;

//const STYLE: &str = "";
    //include_str!("/assets/file_upload.css");


