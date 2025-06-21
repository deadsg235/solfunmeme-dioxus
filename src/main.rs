#![allow(non_snake_case)]

use dioxus::launch;
use crate::playground::app::PlaygroundApp;
//mod views;
//use views::*;
mod views;
//use views::*;
mod model;
use model::*;

// pub mod use_memes;
// pub mod git_parser;

// pub mod utils;
// pub mod views;
//use views::*;

mod header;
mod utils;

use fetch_parser::*;

mod svg_assets;
pub(crate) use svg_assets::*;

//mod dioxus_adapter;
//pub(crate) use dioxus_adapter::*;

mod fetch_util;
pub(crate) use fetch_util::*;

mod app;
pub(crate) use app::LOGO;
pub(crate) use app::Route;


//use dioxus::prelude::*;
// Remove unused import
// use views::*;

// use crate::model::*;
// use crate::header::*;
// use crate::utils::*;
// use crate::fetch_parser::*;
// use crate::svg_assets::*;

//use gloo_timers::callback::Timeout;

use crate::{
    model::NotificationInfo //, Header
};

//const FAVICON: Asset = asset!("/assets/favicon.png");
//const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
//pub(crate) const LOGO: Asset = asset!("/assets/logo.png");

//use crate::model::storage::WALLET_ADAPTER;
//use crate::model::storage::{GLOBAL_MESSAGE, ACCOUNT_STATE, ACTIVE_CONNECTION};
//use crate::model::storage::{ClusterStore, NotificationInfo};
//use crate::model::adaptercluster::AdapterCluster;
//use crate::views::dashboard::Dashboard;
//use crate::views::accounts::Accounts;
//use crate::views::clusters::Clusters;
//use crate::views::extras::Extras;


mod password_manager;
//use password_manager::App;


pub mod dioxus_adapter;
pub mod fetch_parser;
pub mod state;
pub mod playground;
//pub mod git_parser;


//use crate::app::App;
//use crate::crypto_frontend::App;
//use crate::password_manager::App;
//use crate::playground::MenuOption   

fn main() {
    // Use the memes App component from views
    launch(PlaygroundApp);
}
