#![allow(non_snake_case)]
use playground_lib::app::PlaygroundApp;
use dioxus::launch;
use model_lib::*;
use views_lib::*;
use solfunmeme_app::header;
use solfunmeme_wallet_integration::utils;
use solfunmeme_wallet_integration::fetch_parser;
use solfunmeme_app::svg_assets;
pub(crate) use solfunmeme_app::svg_assets::*;
use solfunmeme_wallet_integration::fetch_util;
pub(crate) use solfunmeme_wallet_integration::fetch_util::*;
use solfunmeme_app::app;
use model_lib::NotificationInfo;
pub(crate) use solfunmeme_app::app::{Route, LOGO};
use solfunmeme_wallet_integration::password_manager;
pub use solfunmeme_wallet_integration::fetch_parser;
pub use playground_lib;
pub use model_lib::state;

pub use solfunmeme_core_logic::core;
pub use solfunmeme_generated::embedself;
pub use solfunmeme_generated;

pub use solfunmeme_core_logic::project_algebra;
pub use solfunmeme_core_logic::project_reflector;
pub use solfunmeme_core_logic::solfunmeme_maps;
pub use model_lib::models;
fn main() {
    // Use the memes App component from views
//    embedself::printall();

    launch(PlaygroundApp);
}
