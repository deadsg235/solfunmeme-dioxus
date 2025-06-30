#![allow(non_snake_case)]
use dioxus::launch;
use crate::playground::app::PlaygroundApp;
mod views;
mod model;
use model::*;
mod header;
mod utils;
use fetch_parser::*;
mod svg_assets;
pub(crate) use svg_assets::*;
mod fetch_util;
pub(crate) use fetch_util::*;
mod app;
pub(crate) use app::LOGO;
pub(crate) use app::Route;
use crate::{    model::NotificationInfo };
mod password_manager;

pub mod fetch_parser;
pub mod state;
pub mod playground;
pub mod extractor;


fn main() {
    // Use the memes App component from views
    launch(PlaygroundApp);
}
