

use dioxus::prelude::*;
use crate::header::Header;
use crate::model::NotificationInfo;
    
    //git_parser::GitParser,


use crate::{
    //model::NotificationInfo,
    views::footer::Footer,
    views::notification::Notification ,
    views::page_not_found::PageNotFound,
    //git_parser::GitParser,
};

use crate::views::connections::Connections;
use crate::model::{AccountState};
#[allow(dead_code)]
pub const FAVICON: Asset = asset!("/assets/favicon.png");
#[allow(dead_code)]
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub(crate) const LOGO: Asset = asset!("/assets/logo.png");

use crate::model::storage::WALLET_ADAPTER;
use crate::model::storage::{GLOBAL_MESSAGE, ACCOUNT_STATE, ACTIVE_CONNECTION};
//use crate::model::storage::{ClusterStore, NotificationInfo};
//use crate::model::adaptercluster::AdapterCluster;
use crate::views::dashboard::Dashboard;
use crate::views::accounts::Accounts;
use crate::views::clusters::Clusters;
use crate::views::extras::Extras;



#[component]
pub(crate) fn MainApp() -> Element {
    let wallet_event_listener = WALLET_ADAPTER.read().events().clone();

    // let clusters = vec![
    //     AdapterCluster::devnet(),
    //     AdapterCluster::mainnet(),
    //     AdapterCluster::testnet(),
    //     AdapterCluster::localnet(),
    // ];
    //    if CLUSTER_STORAGE.write().add_clusters(clusters).is_err() {}     // FIXME: add default clusters


    spawn(async move {
        while let Ok(wallet_event) = wallet_event_listener.recv().await {
            *ACCOUNT_STATE.write() = AccountState::default();

            let connection_info = (*WALLET_ADAPTER.read().connection_info().await).clone();
            *ACTIVE_CONNECTION.write() = connection_info;

            GLOBAL_MESSAGE
                .write()
                .push_back(NotificationInfo::new(wallet_event));
        }
    });
    let fi = FAVICON.to_string();
    let tw = TAILWIND_CSS.to_string();
    rsx! {
        document::Link { rel: "icon", href: fi }
        document::Link { rel: "stylesheet", href: tw }
        document::Title {"Rust Wallet Adapter"}

        div { class: "w-full flex min-h-screen font-[sans-serif] dark:bg-rich-black bg-white text-black dark:text-white",

            Notification {}


            div { class: "flex flex-col w-full min-h-full justify-between items-center",
                Router::<Route> {}
                 Connections {}
                Footer{}
            }
        }
    }
}



#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
pub enum Route { 
    #[layout(Header)]
        #[route("/")]
        Dashboard(),
        #[route("/accounts")]
        Accounts(),
        #[route("/clusters")]
        Clusters(),
        #[route("/extras")]
        Extras(),
        // #[feature ("git2")]
        // #[route("/git-parser")]
        // GitParser2(),
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

