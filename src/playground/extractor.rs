use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
use gloo_timers::future::TimeoutFuture;
use std::collections::HashSet;
//use log::error;
use web_sys::{ window, Blob, BlobPropertyBag, Url, HtmlAnchorElement};
use wasm_bindgen::JsCast;
use js_sys::Array;
use wasm_bindgen::{closure::Closure, JsValue};


