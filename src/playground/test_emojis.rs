use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
//use crate::password_manager::PasswordAppState;
use crate::views::component_memes::MemeCategory;
use crate::views::wikidata_memes::WikidataMeme;
use crate::views::workflow_memes::WorkflowMeme;

//use crate::Route;
use crate::header::Header;
//use crate::playground::test_emojis::ComponentName::Header;
//use crate::views::memes::Header;
//use crate::header::ConnectWalletModalModal;
//use crate::playground::test_emojis::ComponentName::ConnectWalletModalModal;
//use crate::playground::test_emojis::ComponentName::NavWalletItem;
//use crate::header::ActiveAccountDropDown;
//use crate::playground::test_emojis::ComponentName::ActiveAccountDropDown;
use crate::playground::test_emojis::ComponentName::PingCluster;
//use crate::password_manager::PasswordApp;
//use crate::playground::test_emojis::ComponentName::PasswordApp;
//use crate::playground::test_emojis::ComponentName::PasswordAppHeader;
// use crate::playground::test_emojis::ComponentName::PasswordErrorMessage;
// use crate::playground::test_emojis::ComponentName::LoginScreen;
// use crate::playground::test_emojis::ComponentName::PasswordMainInterface;
// use crate::playground::test_emojis::ComponentName::PasswordList;
// use crate::playground::test_emojis::ComponentName::AddPasswordForm;
// use crate::playground::test_emojis::ComponentName::PasswordDetail;
// use crate::playground::test_emojis::ComponentName::WelcomeScreen;
// use crate::playground::test_emojis::ComponentName::GitParser2;

//use crate::model::meme_types::{ComponentMeme, MemeCategory, WikidataMeme, WorkflowMeme, WorkflowStep};
//use crate::model::password_manager::PasswordAppState;
use crate::views::{
    accounts::{Accounts},
        //ClusterSuccess, TokenAccountCard, TxCard},
    //airdrop::Airdrop,
    clusters::{
        //AddClusterModal, ClusterInfo,
         Clusters},
    coins::QueryCoinDialog
    
    };

    // component_memes::{ComponentMemeExplorer, ComponentMemeView, MemeCategoryView},
    // connect_first::ConnectWalletFirst,
    // connection_buttons::ConnectionButtons,
    // core_buttons::CoreButtons,
    // crypto_buttons::CryptoButtons,
    // crypto_frontend::{
    //     app::{AppHeader as CryptoAppHeader, CryptoFrontendApp},
    //     components::{ActionButton, CardHeader, ErrorMessage as CryptoErrorMessage, InputField, SuccessMessage, TextAreaField},
    //     forms::{DecryptionForm, EncryptionForm},
    // },
    // dashboard::Dashboard,
    // encryption::Encryption,
    // expression_parsing::ExpressionParsing,
    // extras::Extras,
    // extras_views::{sign_message::SignMessage, sign_tx::SignTx, siws::SignInWithSolana},
    // footer::Footer,
    // //ogit::GitParser2,
    // //header::{ActiveAccountDropDown, ConnectWalletModalModal, Header, NavWalletItem, PingCluster},
    // management_buttons::ManagementButtons,
    // meme_management::MemeManagement,
    // memes::{
    //     CardHeader as MemeCardHeader, CodeDisplay, CreateButton, ExpressionCard, ExpressionInputs, ExpressionList,
    //     ExpressionMetadata, ExpressionTypeSelector, InputSection, Memes, MemesFooter, MetadataInputs, SearchInput,
    //     SimilaritySection, VectorSpace,
    // },
    // meta_meme_operations::MetaMemeOperations,
    // notification::{Notification, Notification2},
    // page_not_found::PageNotFound,
    //password_manager::{
    //AddPasswordForm, AppHeader as PasswordAppHeader, ErrorMessage as PasswordErrorMessage, LoginScreen, MainInterface as PasswordMainInterface,
    //PasswordApp, PasswordDetail, PasswordList, WelcomeScreen,
    //},
    // query_accounts::QueryAccountDialog,
    // receive_sol::ReceiveSol,
    // send_sol::SendSol,
    // styling_and_emojis::StylingAndEmojis,
    // transaction_buttons::TransactionButtons,
    // wikidata_memes::{WikidataMemeExplorer, WikidataMemeView},
    // workflow_memes::{WorkflowMemeExplorer, WorkflowMemeView, WorkflowStepView},

//use crate::{MenuOption, UseConnections};
use crate::views::component_memes::ComponentMeme;

// Component metadata
#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum ComponentName {
    Header,
    ConnectWalletModalModal,
    NavWalletItem,
    MainApp,
    ActiveAccountDropDown,
    PingCluster,
    PasswordApp,
    PasswordAppHeader,
    PasswordErrorMessage,
    LoginScreen,
    PasswordMainInterface,
    PasswordList,
    AddPasswordForm,
    PasswordDetail,
    WelcomeScreen,
    Accounts,
    ClusterSuccess,
    TokenAccountCard,
    TxCard,
    Airdrop,
    Clusters,
    ClusterInfo,
    AddClusterModal,
    QueryCoinDialog,
    ComponentMemeView,
    MemeCategoryView,
    ComponentMemeExplorer,
    ConnectionButtons,
    ConnectWalletFirst,
    CoreButtons,
    CryptoButtons,
    CryptoFrontendApp,
    CryptoAppHeader,
    CardHeader,
    InputField,
    TextAreaField,
    ActionButton,
    CryptoErrorMessage,
    SuccessMessage,
    EncryptionForm,
    DecryptionForm,
    Dashboard,
    Encryption,
    Extras,
    SignMessage,
    SignTx,
    SignInWithSolana,
    Footer,
    GitParser2,
    ManagementButtons,
    MemeManagement,
    Memes,
    MemeCardHeader,
    InputSection,
    ExpressionTypeSelector,
    ExpressionInputs,
    MetadataInputs,
    CreateButton,
    SearchInput,
    ExpressionList,
    ExpressionCard,
    CodeDisplay,
    ExpressionMetadata,
    SimilaritySection,
    VectorSpace,
    MemesFooter,
    MetaMemeOperations,
    Notification,
    Notification2,
    PageNotFound,
    QueryAccountDialog,
    ReceiveSol,
    SendSol,
    StylingAndEmojis,
    TransactionButtons,
    WikidataMemeView,
    WikidataMemeExplorer,
    WorkflowStepView,
    WorkflowMemeView,
    WorkflowMemeExplorer,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ComponentInstance {
    name: ComponentName,
    props: HashMap<String, PropValue>,
    id: u32,
}


#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct WorkflowStep {}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct UseConnections {}
#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct MenuOption {}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
enum PropValue {
    Bool(bool),
    String(String),
//    SignalBool(Signal<bool>),
//    SignalString(Signal<String>),
    //    SignalPasswordAppState(Signal<PasswordAppState>),
//    PasswordAppState(PasswordAppState), has key
    ComponentMeme(ComponentMeme),
    MemeCategory(MemeCategory),
    WikidataMeme(WikidataMeme),
    WorkflowMeme(WorkflowMeme),
    WorkflowStep(WorkflowStep),
    UseConnections(UseConnections),
//    MenuOptionHandler(EventHandler<MenuOption>),
    StringVec(Vec<String>),
}

fn update1(key:String, value:PropValue) {
	//props_config.dict().insert(key, value);

}

#[component]
pub fn ComponentBuilderEmojiApp() -> Element {
    let selected_component = use_signal(|| None::<ComponentName>);
    let components = use_signal(|| vec![] as Vec<ComponentInstance>);
    let next_id = use_signal(|| 0u32);
    let mut props_config = use_signal(|| HashMap::<String, PropValue>::new());
    let show_emoji_dialog = use_signal(|| false);

    // Handle prop updates
    //let update_prop = update1move |key: String, value: PropValue| {
    //props_config.write().insert(key, value);
    //};

    // Assuming props_config is some state, e.g., UseRef or UseState
    //let update_prop: Callback<(String, PropValue), ()> = Callback::from(update1);

    let update_prop = Callback::new(move |(key, value): (String, PropValue)| {
        //update1(props_config, key, value);
	props_config.write().insert(key, value);
    });

    
    //expected `Callback<(String, PropValue)>`, found closure
    // Add component to composition
    let add_component = move || {
//         if let Some(name) = selected_component.read() {
//             let instance = ComponentInstance {
//                 name: name.clone(),
//                 props: props_config.read().clone(),
//                 id: *next_id.read(),
//             };
//             components.write().push(instance);
// 	    // FIXME
// //            next_id.write_with(|id| *id + 1);
//             props_config.set(HashMap::new()); // Reset props
//             selected_component.set(None);
//         }
    };

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6 text-center",
                "Component Builder (Emoji Edition)"
            }
            div { class: "mb-4",
                button {
                    class: "bg-purple-500 text-white px-4 py-2 rounded-lg hover:bg-purple-600",
//                    onclick: move |_| show_emoji_dialog.set(!show_emoji_dialog()),
                    "Toggle Emoji Dialog"
                }
            }
            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // Component Selection Sidebar
                div { class: "lg:col-span-1",
                    div { class: "bg-white shadow-lg rounded-lg p-4",
                        h2 { class: "text-xl font-semibold mb-4", "Select Component" }
                        for category in get_component_categories() {
                            div { class: "mb-4",
                                h3 { class: "text-lg font-medium mb-2", "{category.0}" }
                                for component in category.1 {
                                    button {
                                        class: format!(
                                            "w-full text-left p-3 mb-2 rounded-lg transition-colors {}",
                                            if selected_component() == Some(component.clone()) {
                                                "bg-blue-500 text-white"
                                            } else {
                                                "bg-gray-100 hover:bg-gray-200"
                                            }
                                        ),
//                                        onclick: move |_| selected_component.set(Some(component.clone())),
                                        "{component_name(&component)}"
                                        span { class: "ml-2", "{get_emoji(&component, None)}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Configuration and Preview Panel
                div { class: "lg:col-span-3",
                    div { class: "bg-white shadow-lg rounded-lg p-6",
                        div { class: "flex justify-between items-center mb-4",
                            h2 { class: "text-2xl font-semibold",
                                "{selected_component().map(|c| component_name(&c)).unwrap_or_default()}"
                            }
                            button {
                                class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
                                onclick: move |_| add_component(),
                                "Add Component"
                            }
                        }
                        if let Some(component) = selected_component() {
                            ComponentConfigPanel {
                                component,
                                on_update: update_prop
                            }
                        }
                        div { class: "mt-6",
                            h3 { class: "text-lg font-semibold mb-2", "Composed Components" }
                            div { class: "grid gap-4",
                                for instance in components.read().iter() {
                                    RenderComponent { instance: instance.clone() }
                                }
                            }
                        }
                    }
                }
            }
            if *show_emoji_dialog.read() {
                ComponentEmojiDialog {
                    components: components.read().clone(),
                }
            }
            // Navigation Link
            // div { class: "mt-4",
            //     Link {
            //         to: Route::TestMenuApp {},
            //         class: "bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600",
            //         "Back to Test Menu"
            //     }
            // }
        }
    }
}

#[component]
fn ComponentEmojiDialog(components: Vec<ComponentInstance>) -> Element {
    let show_emoji_dialog = use_signal(|| false);
    rsx! {
        div {
            class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
            div {
                class: "bg-white rounded-lg p-6 w-full max-w-md",
                h2 { class: "text-2xl font-semibold mb-4", "Emoji Composition" }
                div { class: "flex flex-wrap gap-2 mb-4",
                    for instance in components.iter() {
                        span {
                            class: format!("text-2xl {}", get_emoji_style(&instance.name, &instance.props)),
                            "{get_emoji(&instance.name, Some(&instance.props))}"
                        }
                    }
                }
                button {
                    class: "bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600",
//                    onclick: move |_| show_emoji_dialog.set(false),
                    "Close"
                }
            }
        }
    }
}

#[component]
fn ComponentConfigPanel(component: ComponentName, on_update: EventHandler<(String, PropValue)>) -> Element {
    let props = get_component_props(&component);
    rsx! {
        div { class: "border border-gray-200 rounded-lg p-4",
            h3 { class: "font-medium text-lg mb-2", "Configure Properties" }
            for prop in props {
                div { class: "mb-4",
                    label { class: "block text-sm font-medium text-gray-700", "{prop.0}" }
                    match prop.1 {
                        // "bool" => {
                        //     let value = use_signal(|| false);
                        //     rsx! {
                        //         input {
                        //             r#type: "checkbox",
                        //             checked: "{value}",
                        //             onchange: move |evt| {
                        //                 value.set(evt.checked());
                        //                 on_update.call((prop.0.to_string(), PropValue::SignalBool(value)));
                        //             }
                        //         }
                        //     }
                        // }
                        // "string" => {
                        //     let value = use_signal(|| String::new());
                        //     rsx! {
                        //         input {
                        //             class: "w-full p-2 border border-gray-300 rounded-lg",
                        //             value: "{value}",
                        //             oninput: move |evt| {
                        //                 value.set(evt.value().clone());
                        //                 on_update.call((prop.0.to_string(), PropValue::SignalString(value)));
                        //             }
                        //         }
                        //     }
                        // }
                        // "password_app_state" => {
                        //     let value = use_signal(|| PasswordAppState::default());
                        //     rsx! {
                        //         button {
                        //             class: "bg-blue-500 text-white px-3 py-1 rounded",
                        //             onclick: move |_| {
                        //                 value.set(PasswordAppState::default());
                        //                 on_update.call((prop.0.to_string(), PropValue::SignalPasswordAppState(value)));
                        //             },
                        //             "Reset State"
                        //         }
                        //     }
                        // }
                        _ => rsx! { span { class: "text-sm text-gray-500", "Unsupported prop type" } },
                    }
                }
            }
        }
    }
}

#[component]
fn RenderComponent(instance: ComponentInstance) -> Element {
    match instance.name {
        ComponentName::Header => rsx! { Header {} },
        // ComponentName::ConnectWalletModalModal => {
        //     // let show_modal = instance.props.get("show_modal").and_then(|v| match v {
        //     //     PropValue::SignalBool(s) => Some(*s),
        //     //     _ => None,
        //     // }).unwrap_or_default();
        //     // let show_connecting = instance.props.get("show_connecting").and_then(|v| match v {
        //     //     PropValue::SignalBool(s) => Some(*s),
        //     //     _ => None,
        //     // }).unwrap_or_default();
        //     //rsx! { ConnectWalletModalModal { show_modal, show_connecting } }
        // }
        // ComponentName::NavWalletItem => {
        //     let show_modal = instance.props.get("show_modal").and_then(|v| match v {
        //         PropValue::SignalBool(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     let show_connecting = instance.props.get("show_connecting").and_then(|v| match v {
        //         PropValue::SignalBool(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { NavWalletItem { show_modal, show_connecting } }
        // }
        ComponentName::MainApp => rsx! { div { "MainApp (Placeholder)" } },
        ComponentName::ActiveAccountDropDown => {
            // let show_modal = instance.props.get("show_modal").and_then(|v| match v {
            //     PropValue::SignalBool(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            // let shortened_address = instance.props.get("shortened_address").and_then(|v| match v {
            //     PropValue::SignalString(s) => Some(s.read().clone()),
            //     _ => None,
            // }).unwrap_or_default();
	    //            rsx! { ActiveAccountDropDown { show_modal, shortened_address } }
	    rsx! { div {"fixme"} }
        }
        //ComponentName::PingCluster => rsx! { PingCluster {} },
        //ComponentName::PasswordApp => rsx! { PasswordApp {} },
        ComponentName::PasswordAppHeader => {
	    	    rsx! { div {"fixme"} }
            // let app_state = instance.props.get("app_state").and_then(|v| match v {
            //     PropValue::SignalPasswordAppState(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { PasswordAppHeader { app_state } }
        }
        ComponentName::PasswordErrorMessage => {
	    	    rsx! { div {"fixme"} }
            // let message = instance.props.get("message").and_then(|v| match v {
            //     PropValue::SignalString(s) => Some(s.read().clone()),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { PasswordErrorMessage { message } }
        }
        ComponentName::LoginScreen => {
	    	    rsx! { div {"fixme"} }
            // let app_state = instance.props.get("app_state").and_then(|v| match v {
            //     PropValue::SignalPasswordAppState(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { LoginScreen { app_state } }
        }
        ComponentName::PasswordMainInterface => {
	    rsx! { div {"fixme"} }
            // let app_state = instance.props.get("app_state").and_then(|v| match v {
            //     PropValue::SignalPasswordAppState(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { PasswordMainInterface { app_state } }
        }
        ComponentName::PasswordList => {
	    rsx! { div {"fixme"} }
            // let app_state = instance.props.get("app_state").and_then(|v| match v {
            //     PropValue::SignalPasswordAppState(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { PasswordList { app_state } }
        }
        ComponentName::AddPasswordForm => {
	    	    rsx! { div {"fixme"} }
            // let app_state = instance.props.get("app_state").and_then(|v| match v {
            //     PropValue::SignalPasswordAppState(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { AddPasswordForm { app_state } }
        }
        ComponentName::PasswordDetail => {
	    	    rsx! { div {"fixme"} }
            // let app_state = instance.props.get("app_state").and_then(|v| match v {
            //     PropValue::SignalPasswordAppState(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { PasswordDetail { app_state } }
        }
        ComponentName::WelcomeScreen => {
	    rsx! { div {"fixme"} }
	    //WelcomeScreen {}
	},
        ComponentName::Accounts => rsx! { Accounts {} },
        ComponentName::ClusterSuccess => {
	    rsx! { div {"fixme"} }
            // let address = instance.props.get("address").and_then(|v| match v {
            //     PropValue::SignalString(s) => Some(s.read().clone()),
            //     _ => None,
            // }).unwrap_or_default();
            // let shortened_address = instance.props.get("shortened_address").and_then(|v| match v {
            //     PropValue::SignalString(s) => Some(s.read().clone()),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { ClusterSuccess { address, shortened_address } }
        }
        ComponentName::TokenAccountCard => {
	    rsx! { div {"fixme"} }
            // let mint = instance.props.get("mint").and_then(|v| match v {
            //     PropValue::SignalString(s) => Some(s.read().clone()),
            //     _ => None,
            // }).unwrap_or_default();
            // let ata_address = instance.props.get("ata_address").and_then(|v| match v {
            //     PropValue::SignalString(s) => Some(s.read().clone()),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { TokenAccountCard { mint, ata_address } }
        }
        ComponentName::TxCard => {
	    rsx! { div {"fixme"} }
            // let tx = instance.props.get("tx").and_then(|v| match v {
            //     PropValue::SignalString(s) => Some(s.read().clone()),
            //     _ => None,
            // }).unwrap_or_default();
            // let timestamp = instance.props.get("timestamp").and_then(|v| match v {
            //     PropValue::String(s) => s.parse::<i64>().ok(),
            //     _ => None,
            // });
            // rsx! { TxCard { tx, timestamp } }
        }
        ComponentName::Airdrop => {
	    rsx! { div {"fixme"} }
            // let show_airdrop_modal = instance.props.get("show_airdrop_modal").and_then(|v| match v {
            //     PropValue::SignalBool(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            //rsx! { Airdrop { show_airdrop_modal } }
        }
        ComponentName::Clusters => rsx! { Clusters {} },
        ComponentName::ClusterInfo => {
            //let connections = instance.props.get("connections").and_then(|v| match v {
            //PropValue::UseConnections(c) => Some(c.clone()),
	    //                _ => None,
	    //          }).unwrap_or_default();
            //rsx! { ClusterInfo { connections } }
	    rsx! { div {"fixme"} }
        }
        ComponentName::AddClusterModal => {
            // let show_add_entry_modal = instance.props.get("show_add_entry_modal").and_then(|v| match v {
            //     PropValue::SignalBool(s) => Some(*s),
            //     _ => None,
            // }).unwrap_or_default();
            //let connections = instance.props.get("connections").and_then(|v| match v {
            //PropValue::UseConnections(c) => Some(c.clone()),
            //_ => None,
            //}).unwrap_or_default();
            //rsx! { AddClusterModal { show_add_entry_modal, connections } }
	    rsx! { div {"fixme"} }
        }
        ComponentName::QueryCoinDialog => rsx! { QueryCoinDialog {} },
        ComponentName::ComponentMemeView => {
	    rsx! { div {"fixme"} }
            //let component_meme = instance.props.get("component_meme").and_then(|v| match v {
            //PropValue::ComponentMeme(c) => Some(c.clone()),
            //_ => None,
            //}).unwrap_or_default();
            //rsx! { ComponentMemeView { component_meme } }
        }
        ComponentName::MemeCategoryView => {
	    rsx! { div {"fixme"} }
            // let category = instance.props.get("category").and_then(|v| match v {
            //     PropValue::MemeCategory(c) => Some(c.clone()),
            //     _ => None,
            // }).unwrap_or_default();
            // rsx! { MemeCategoryView { category } }
        }
        ComponentName::ComponentMemeExplorer => {
	    rsx! { div {"fixme"} }
	    //rsx! { ComponentMemeExplorer {} },
	}
        ComponentName::ConnectionButtons => {
	    rsx! { div {"fixme"} }
	    
	    //            let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
              //  PropValue::MenuOptionHandler(h) => Some(h.clone()),
            //_ => None,
        //}).unwrap_or_default();
//            rsx! { ConnectionButtons { on_menu_change } }
        }
        ComponentName::ConnectWalletFirst => {
	    rsx! { div {"fixme"} }
	    //rsx! { ConnectWalletFirst {} },
	},
        ComponentName::CoreButtons => {
            //let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
            //PropValue::MenuOptionHandler(h) => Some(h.clone()),
            //_ => None,
	    //            }).unwrap_or_default();
            //rsx! { CoreButtons { on_menu_change } }
	    rsx! { div {"fixme"} }
        }
        // ComponentName::CryptoButtons => {
	//     rsx! { div {"fixme"} }
        //     let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //         PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { CryptoButtons { on_menu_change } }
        // }
        // ComponentName::CryptoFrontendApp => rsx! { CryptoFrontendApp {} },
        // ComponentName::CryptoAppHeader => rsx! { CryptoAppHeader {} },
        // ComponentName::CardHeader => {
        //     let title = instance.props.get("title").and_then(|v| match v {
        //         PropValue::SignalString(s) => Some(s.read().clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { CardHeader { title } }
        // }
        // ComponentName::InputField => {
        //     let label = instance.props.get("label").and_then(|v| match v {
        //         PropValue::SignalString(s) => Some(s.read().clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { InputField { label } }
        // }
        // ComponentName::TextAreaField => {
        //     let label = instance.props.get("label").and_then(|v| match v {
        //         PropValue::SignalString(s) => Some(s.read().clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { TextAreaField { label } }
        // }
        // ComponentName::ActionButton => {
        //     let label = instance.props.get("label").and_then(|v| match v {
        //         PropValue::SignalString(s) => Some(s.read().clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { ActionButton { label } }
        // }
        // ComponentName::CryptoErrorMessage => {
        //     let message = instance.props.get("message").and_then(|v| match v {
        //         PropValue::SignalString(s) => Some(s.read().clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { CryptoErrorMessage { message } }
        // }
        // ComponentName::SuccessMessage => {
        //     let message = instance.props.get("message").and_then(|v| match v {
        //         PropValue::SignalString(s) => Some(s.read().clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { SuccessMessage { message } }
        // }
        // ComponentName::EncryptionForm => rsx! { EncryptionForm {} },
        // ComponentName::DecryptionForm => rsx! { DecryptionForm {} },
        // ComponentName::Dashboard => rsx! { Dashboard {} },
        // ComponentName::Encryption => rsx! { Encryption {} },
        // ComponentName::Extras => rsx! { Extras {} },
        // ComponentName::SignMessage => rsx! { SignMessage {} },
        // ComponentName::SignTx => rsx! { SignTx {} },
        // ComponentName::SignInWithSolana => rsx! { SignInWithSolana {} },
        // ComponentName::Footer => rsx! { Footer {} },
        // ComponentName::GitParser2 => rsx! { GitParser2 {} },
        // ComponentName::ManagementButtons => {
        //     let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //         PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { ManagementButtons { on_menu_change } }
        // }
        // ComponentName::MemeManagement => rsx! { MemeManagement {} },
        // ComponentName::Memes => rsx! { Memes {} },
        // ComponentName::MemeCardHeader => {
        //     let expr = instance.props.get("expression").and_then(|v| match v {
        //         PropValue::String(s) => Some(s.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { MemeCardHeader { expression: expr, state } }
        // }
        // ComponentName::InputSection => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { InputSection { state } }
        // }
        // ComponentName::ExpressionTypeSelector => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { ExpressionTypeSelector { state } }
        // }
        // ComponentName::ExpressionInputs => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { ExpressionInputs { state } }
        // }
        // ComponentName::MetadataInputs => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { MetadataInputs { state } }
        // }
        // ComponentName::CreateButton => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { CreateButton { state } }
        // }
        // ComponentName::SearchInput => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { SearchInput { state } }
        // }
        // ComponentName::ExpressionList => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { ExpressionList { state } }
        // }
        // ComponentName::ExpressionCard => {
        //     let expr = instance.props.get("expression").and_then(|v| match v {
        //         PropValue::String(s) => Some(s.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { ExpressionCard { expression: expr, state } }
        // }
        // ComponentName::CodeDisplay => {
        //     let expr = instance.props.get("expression").and_then(|v| match v {
        //         PropValue::String(s) => Some(s.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { CodeDisplay { expression: expr } }
        // }
        // ComponentName::ExpressionMetadata => {
        //     let expr = instance.props.get("expression").and_then(|v| match v {
        //         PropValue::String(s) => Some(s.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { ExpressionMetadata { expression: expr } }
        // }
        // ComponentName::SimilaritySection => {
        //     let expr = instance.props.get("expression").and_then(|v| match v {
        //         PropValue::String(s) => Some(s.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { SimilaritySection { expression: expr, state } }
        // }
        // ComponentName::VectorSpace => {
        //     let state = instance.props.get("state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(*s),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { VectorSpace { state } }
        // }
        // ComponentName::MemesFooter => rsx! { MemesFooter {} },
        // ComponentName::MetaMemeOperations => rsx! { MetaMemeOperations {} },
        // ComponentName::Notification => rsx! { Notification {} },
        // ComponentName::Notification2 => rsx! { Notification2 {} },
        // ComponentName::PageNotFound => {
        //     let route = instance.props.get("route").and_then(|v| match v {
        //         PropValue::StringVec(v) => Some(v.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { PageNotFound { route } }
        // }
        // ComponentName::QueryAccountDialog => {
        //     // let show_query_dialog = instance.props.get("show_query_dialog").and_then(|v| match v {
        //     //     PropValue::SignalBool(s) => Some(*s),
        //     //     _ => None,
        //     // }).unwrap_or_default();
        //     //rsx! { QueryAccountDialog { show_query_dialog } }
        // }
        // ComponentName::ReceiveSol => {
        //     // let show_receive_modal = instance.props.get("show_receive_modal").and_then(|v| match v {
        //     //     PropValue::SignalBool(s) => Some(*s),
        //     //     _ => None,
        //     // }).unwrap_or_default();
        //     //rsx! { ReceiveSol { show_receive_modal } }
        // }
        // ComponentName::SendSol => {
        //     // let show_send_modal = instance.props.get("show_send_modal").and_then(|v| match v {
        //     //     PropValue::SignalBool(s) => Some(*s),
        //     //     _ => None,
        //     // }).unwrap_or_default();
        //     //rsx! { SendSol { show_send_modal } }
        // }
        // ComponentName::StylingAndEmojis => rsx! { StylingAndEmojis {} },
        // ComponentName::TransactionButtons => {
        //     let on_menu_change = instance.props.get("on_menu_change").and_then(|v| match v {
        //         PropValue::MenuOptionHandler(h) => Some(h.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { TransactionButtons { on_menu_change } }
        // }
        // ComponentName::WikidataMemeView => {
        //     let meme = instance.props.get("meme").and_then(|v| match v {
        //         PropValue::WikidataMeme(m) => Some(m.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { WikidataMemeView { meme } }
        // }
        // ComponentName::WikidataMemeExplorer => rsx! { WikidataMemeExplorer {} },
        // ComponentName::WorkflowStepView => {
        //     let step = instance.props.get("step").and_then(|v| match v {
        //         PropValue::WorkflowStep(s) => Some(s.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { WorkflowStepView { step } }
        // }
        // ComponentName::WorkflowMemeView => {
        //     let workflow = instance.props.get("workflow").and_then(|v| match v {
        //         PropValue::WorkflowMeme(w) => Some(w.clone()),
        //         _ => None,
        //     }).unwrap_or_default();
        //     rsx! { WorkflowMemeView { workflow } }
        // }
        // ComponentName::WorkflowMemeExplorer => rsx! { WorkflowMemeExplorer {} },
	_ => {
	    rsx! { div {"fixme!"} }
	}
    }
}

fn get_emoji(component: &ComponentName, props: Option<&HashMap<String, PropValue>>) -> &'static str {
    match component {
	ComponentName::StylingAndEmojis => "🎭",
        &ComponentName::ConnectWalletFirst => todo!(),

        //ConnectWallet
	//&ConnectWalletFirst => {
	//"FIXME"
	//},
        ComponentName::Header | ComponentName::MainApp => "🏠",
        ComponentName::ConnectWalletModalModal | ComponentName::NavWalletItem | ComponentName::ActiveAccountDropDown => {
            if let Some(props) = props {
                // if props.get("show_modal").and_then(|v| match v {
                //     PropValue::SignalBool(s) => Some(*s.read()),
                //     _ => None,
                // }).unwrap_or(false) {
                //     "🪟"
                // } else {
                //     "💸"
                // }
		"Hello"
            } else {
		
                "💸"
            }
        }
        ComponentName::PingCluster => "📡",
        ComponentName::PasswordApp | ComponentName::PasswordAppHeader | ComponentName::PasswordErrorMessage |
        ComponentName::LoginScreen | ComponentName::PasswordMainInterface | ComponentName::PasswordList |
        ComponentName::AddPasswordForm | ComponentName::PasswordDetail => {
            // if let Some(props) = props {
            //     if props.get("app_state").and_then(|v| match v {
            //         PropValue::SignalPasswordAppState(s) => Some(s.read().is_locked()),
            //         _ => None,
            //     }).unwrap_or(true) {
            //         "🔒"
            //     } else {
            //         "🔓"
            //     }
            // } else {
                "🔒"
        //}
    }
        ComponentName::WelcomeScreen => "👋",
        ComponentName::Accounts | ComponentName::ClusterSuccess | ComponentName::TokenAccountCard | ComponentName::TxCard => "💳",
        ComponentName::Airdrop => "🎁",
        ComponentName::Clusters | ComponentName::ClusterInfo | ComponentName::AddClusterModal => "🌐",
        ComponentName::QueryCoinDialog => "🪙",
        ComponentName::ComponentMemeView | ComponentName::MemeCategoryView | ComponentName::ComponentMemeExplorer |
        ComponentName::Memes | ComponentName::MemeCardHeader | ComponentName::InputSection |
        ComponentName::ExpressionTypeSelector | ComponentName::ExpressionInputs | ComponentName::MetadataInputs |
        ComponentName::CreateButton | ComponentName::SearchInput | ComponentName::ExpressionList |
        ComponentName::ExpressionCard | ComponentName::CodeDisplay | ComponentName::ExpressionMetadata |
        ComponentName::SimilaritySection | ComponentName::VectorSpace | ComponentName::MemesFooter |
        ComponentName::WikidataMemeView | ComponentName::WikidataMemeExplorer | ComponentName::WorkflowStepView |
        ComponentName::WorkflowMemeView | ComponentName::WorkflowMemeExplorer => "😂",
        ComponentName::ConnectionButtons | ComponentName::CoreButtons | ComponentName::CryptoButtons |
        ComponentName::ManagementButtons | ComponentName::TransactionButtons => "🛠️",
        ComponentName::CryptoFrontendApp | ComponentName::CryptoAppHeader | ComponentName::CardHeader |
        ComponentName::InputField | ComponentName::TextAreaField | ComponentName::ActionButton |
        ComponentName::CryptoErrorMessage | ComponentName::SuccessMessage | ComponentName::EncryptionForm |
        ComponentName::DecryptionForm | ComponentName::Encryption => "🔐",
        ComponentName::Dashboard => "📊",
        ComponentName::Extras | ComponentName::SignMessage | ComponentName::SignTx | ComponentName::SignInWithSolana => "✨",
        ComponentName::Footer => "📍",
        ComponentName::GitParser2 => "📜",
        ComponentName::MemeManagement | ComponentName::MetaMemeOperations => "🎨",
        ComponentName::Notification | ComponentName::Notification2 => "🔔",
        ComponentName::PageNotFound => "❓",
        ComponentName::QueryAccountDialog => "🔍",
        ComponentName::ReceiveSol => "📥",
        ComponentName::SendSol => "📤",
        ComponentName::StylingAndEmojis => "🎭",
    }
}

fn get_emoji_style(component: &ComponentName, props: &HashMap<String, PropValue>) -> &'static str {
    match component {
        ComponentName::ConnectWalletModalModal | ComponentName::NavWalletItem | ComponentName::ActiveAccountDropDown => {
            //if props.get("show_modal").and_then(|v| match v {
                //PropValue::SignalBool(s) => Some(*s.read()),
            //_ => None,
        //}).unwrap_or(false) {
          //      "text-blue-500"
           // } else {
                "text-green-500"
            //}
        }
        ComponentName::PasswordApp | ComponentName::PasswordAppHeader | ComponentName::PasswordErrorMessage |
        ComponentName::LoginScreen | ComponentName::PasswordMainInterface | ComponentName::PasswordList |
        //ComponentName::AddPasswordForm | ComponentName::PasswordDetail => {
        //     if props.get("app_state").and_then(|v| match v {
        //         PropValue::SignalPasswordAppState(s) => Some(s.read().is_locked()),
        //         _ => None,
        //     }).unwrap_or(true) {
        //         "text-red-500"
        //     } else {
        //         "text-green-500"
        //     }
        // }
        _ => "text-gray-700",
    }
}

fn component_name(component: &ComponentName) -> &'static str {
    match component {
        ComponentName::Header => "Header",
        ComponentName::ConnectWalletModalModal => "Connect Wallet Modal",
        ComponentName::NavWalletItem => "Nav Wallet Item",
        ComponentName::MainApp => "Main App",
        ComponentName::ActiveAccountDropDown => "Active Account Dropdown",
        ComponentName::PingCluster => "Ping Cluster",
        ComponentName::PasswordApp => "Password App",
        ComponentName::PasswordAppHeader => "Password App Header",
        ComponentName::PasswordErrorMessage => "Password Error Message",
        ComponentName::LoginScreen => "Login Screen",
        ComponentName::PasswordMainInterface => "Password Main Interface",
        ComponentName::PasswordList => "Password List",
        ComponentName::AddPasswordForm => "Add Password Form",
        ComponentName::PasswordDetail => "Password Detail",
        ComponentName::WelcomeScreen => "Welcome Screen",
        ComponentName::Accounts => "Accounts",
        ComponentName::ClusterSuccess => "Cluster Success",
        ComponentName::TokenAccountCard => "Token Account Card",
        ComponentName::TxCard => "Tx Card",
        ComponentName::Airdrop => "Airdrop",
        ComponentName::Clusters => "Clusters",
        ComponentName::ClusterInfo => "Cluster Info",
        ComponentName::AddClusterModal => "Add Cluster Modal",
        ComponentName::QueryCoinDialog => "Query Coin Dialog",
        ComponentName::ComponentMemeView => "Component Meme View",
        ComponentName::MemeCategoryView => "Meme Category View",
        ComponentName::ComponentMemeExplorer => "Component Meme Explorer",
        ComponentName::ConnectionButtons => "Connection Buttons",
        ComponentName::ConnectWalletFirst => "Connect Wallet First",
        ComponentName::CoreButtons => "Core Buttons",
        ComponentName::CryptoButtons => "Crypto Buttons",
        ComponentName::CryptoFrontendApp => "Crypto Frontend App",
        ComponentName::CryptoAppHeader => "Crypto App Header",
        ComponentName::CardHeader => "Card Header",
        ComponentName::InputField => "Input Field",
        ComponentName::TextAreaField => "Text Area Field",
        ComponentName::ActionButton => "Action Button",
        ComponentName::CryptoErrorMessage => "Crypto Error Message",
        ComponentName::SuccessMessage => "Success Message",
        ComponentName::EncryptionForm => "Encryption Form",
        ComponentName::DecryptionForm => "Decryption Form",
        ComponentName::Dashboard => "Dashboard",
        ComponentName::Encryption => "Encryption",
        ComponentName::Extras => "Extras",
        ComponentName::SignMessage => "Sign Message",
        ComponentName::SignTx => "Sign Tx",
        ComponentName::SignInWithSolana => "Sign In With Solana",
        ComponentName::Footer => "Footer",
        ComponentName::GitParser2 => "Git Parser",
        ComponentName::ManagementButtons => "Management Buttons",
        ComponentName::MemeManagement => "Meme Management",
        ComponentName::Memes => "Memes",
        ComponentName::MemeCardHeader => "Meme Card Header",
        ComponentName::InputSection => "Input Section",
        ComponentName::ExpressionTypeSelector => "Expression Type Selector",
        ComponentName::ExpressionInputs => "Expression Inputs",
        ComponentName::MetadataInputs => "Metadata Inputs",
        ComponentName::CreateButton => "Create Button",
        ComponentName::SearchInput => "Search Input",
        ComponentName::ExpressionList => "Expression List",
        ComponentName::ExpressionCard => "Expression Card",
        ComponentName::CodeDisplay => "Code Display",
        ComponentName::ExpressionMetadata => "Expression Metadata",
        ComponentName::SimilaritySection => "Similarity Section",
        ComponentName::VectorSpace => "Vector Space",
        ComponentName::MemesFooter => "Memes Footer",
        ComponentName::MetaMemeOperations => "Meta Meme Operations",
        ComponentName::Notification => "Notification",
        ComponentName::Notification2 => "Notification 2",
        ComponentName::PageNotFound => "Page Not Found",
        ComponentName::QueryAccountDialog => "Query Account Dialog",
        ComponentName::ReceiveSol => "Receive Sol",
        ComponentName::SendSol => "Send Sol",
        ComponentName::StylingAndEmojis => "Styling And Emojis",
        ComponentName::TransactionButtons => "Transaction Buttons",
        ComponentName::WikidataMemeView => "Wikidata Meme View",
        ComponentName::WikidataMemeExplorer => "Wikidata Meme Explorer",
        ComponentName::WorkflowStepView => "Workflow Step View",
        ComponentName::WorkflowMemeView => "Workflow Meme View",
        ComponentName::WorkflowMemeExplorer => "Workflow Meme Explorer",
    }
}

fn get_component_categories() -> Vec<(&'static str, Vec<ComponentName>)> {
    vec![
        ("Header", vec![
            ComponentName::Header,
            ComponentName::ConnectWalletModalModal,
            ComponentName::MainApp,
            ComponentName::NavWalletItem,
            ComponentName::ActiveAccountDropDown,
            ComponentName::PingCluster,
            ComponentName::Encryption,
        ]),
        ("Password Manager", vec![
            ComponentName::PasswordApp,
            ComponentName::PasswordAppHeader,
            ComponentName::PasswordErrorMessage,
            ComponentName::LoginScreen,
            ComponentName::PasswordMainInterface,
            ComponentName::PasswordList,
            ComponentName::AddPasswordForm,
            ComponentName::PasswordDetail,
            ComponentName::WelcomeScreen,
        ]),
        ("Accounts", vec![
            ComponentName::Accounts,
            ComponentName::ClusterSuccess,
            ComponentName::TokenAccountCard,
            ComponentName::TxCard,
        ]),
        ("Clusters", vec![
            ComponentName::Clusters,
            ComponentName::ClusterInfo,
            ComponentName::AddClusterModal,
        ]),
        ("Airdrop", vec![ComponentName::Airdrop]),
        ("Coins", vec![ComponentName::QueryCoinDialog]),
        ("Memes", vec![
            ComponentName::ComponentMemeView,
            ComponentName::MemeCategoryView,
            ComponentName::ComponentMemeExplorer,
            ComponentName::Memes,
            ComponentName::MemeCardHeader,
            ComponentName::InputSection,
            ComponentName::ExpressionTypeSelector,
            ComponentName::ExpressionInputs,
            ComponentName::MetadataInputs,
            ComponentName::CreateButton,
            ComponentName::SearchInput,
            ComponentName::ExpressionList,
            ComponentName::ExpressionCard,
            ComponentName::CodeDisplay,
            ComponentName::ExpressionMetadata,
            ComponentName::SimilaritySection,
            ComponentName::VectorSpace,
            ComponentName::MemesFooter,
            ComponentName::WikidataMemeView,
            ComponentName::WikidataMemeExplorer,
            ComponentName::WorkflowStepView,
            ComponentName::WorkflowMemeView,
            ComponentName::WorkflowMemeExplorer,
        ]),
        ("Buttons", vec![
            ComponentName::ConnectionButtons,
            ComponentName::CoreButtons,
            ComponentName::CryptoButtons,
            ComponentName::ManagementButtons,
            ComponentName::TransactionButtons,
        ]),
        ("Crypto Frontend", vec![
            ComponentName::CryptoFrontendApp,
            ComponentName::CryptoAppHeader,
            ComponentName::CardHeader,
            ComponentName::InputField,
            ComponentName::TextAreaField,
            ComponentName::ActionButton,
            ComponentName::CryptoErrorMessage,
            ComponentName::SuccessMessage,
            ComponentName::EncryptionForm,
            ComponentName::DecryptionForm,
        ]),
        ("Dashboard", vec![ComponentName::Dashboard]),
        ("Extras", vec![
            ComponentName::Extras,
            ComponentName::SignMessage,
            ComponentName::SignTx,
            ComponentName::SignInWithSolana,
        ]),
        ("Footer", vec![ComponentName::Footer]),
        ("Git", vec![ComponentName::GitParser2]),
        ("Meme Management", vec![
            ComponentName::MemeManagement,
            ComponentName::MetaMemeOperations,
        ]),
        ("Notifications", vec![
            ComponentName::Notification,
            ComponentName::Notification2,
        ]),
        ("Page Not Found", vec![ComponentName::PageNotFound]),
        ("Transactions", vec![
            ComponentName::QueryAccountDialog,
            ComponentName::ReceiveSol,
            ComponentName::SendSol,
        ]),
        ("Styling", vec![ComponentName::StylingAndEmojis]),
        ("Connection", vec![ComponentName::ConnectWalletFirst]),
    ]
}

fn get_component_props(component: &ComponentName) -> Vec<(&'static str, &'static str)> {
    match component {
        ComponentName::ConnectWalletModalModal => vec![
            ("show_modal", "bool"),
            ("show_connecting", "bool"),
        ],
        ComponentName::NavWalletItem => vec![
            ("show_modal", "bool"),
            ("show_connecting", "bool"),
        ],
        ComponentName::ActiveAccountDropDown => vec![
            ("show_modal", "bool"),
            ("shortened_address", "string"),
        ],
        ComponentName::PasswordAppHeader => vec![("app_state", "password_app_state")],
        ComponentName::PasswordErrorMessage => vec![("message", "string")],
        ComponentName::LoginScreen => vec![("app_state", "password_app_state")],
        ComponentName::PasswordMainInterface => vec![("app_state", "password_app_state")],
        ComponentName::PasswordList => vec![("app_state", "password_app_state")],
        ComponentName::AddPasswordForm => vec![("app_state", "password_app_state")],
        ComponentName::PasswordDetail => vec![("app_state", "password_app_state")],
        ComponentName::ClusterSuccess => vec![
            ("address", "string"),
            ("shortened_address", "string"),
        ],
        ComponentName::TokenAccountCard => vec![
            ("mint", "string"),
            ("ata_address", "string"),
        ],
        ComponentName::TxCard => vec![
            ("tx", "string"),
            ("timestamp", "string"),
        ],
        ComponentName::Airdrop => vec![("show_airdrop_modal", "bool")],
        ComponentName::ClusterInfo => vec![("connections", "use_connections")],
        ComponentName::AddClusterModal => vec![
            ("show_add_entry_modal", "bool"),
            ("connections", "use_connections"),
        ],
        ComponentName::ComponentMemeView => vec![("component_meme", "component_meme")],
        ComponentName::MemeCategoryView => vec![("category", "meme_category")],
        ComponentName::ConnectionButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::CoreButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::CryptoButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::CardHeader => vec![("title", "string")],
        ComponentName::InputField => vec![("label", "string")],
        ComponentName::TextAreaField => vec![("label", "string")],
        ComponentName::ActionButton => vec![("label", "string")],
        ComponentName::CryptoErrorMessage => vec![("message", "string")],
        ComponentName::SuccessMessage => vec![("message", "string")],
        ComponentName::MemeCardHeader => vec![
            ("expression", "string"),
            ("state", "password_app_state"),
        ],
        ComponentName::InputSection => vec![("state", "password_app_state")],
        ComponentName::ExpressionTypeSelector => vec![("state", "password_app_state")],
        ComponentName::ExpressionInputs => vec![("state", "password_app_state")],
        ComponentName::MetadataInputs => vec![("state", "password_app_state")],
        ComponentName::CreateButton => vec![("state", "password_app_state")],
        ComponentName::SearchInput => vec![("state", "password_app_state")],
        ComponentName::ExpressionList => vec![("state", "password_app_state")],
        ComponentName::ExpressionCard => vec![
            ("expression", "string"),
            ("state", "password_app_state"),
        ],
        ComponentName::CodeDisplay => vec![("expression", "string")],
        ComponentName::ExpressionMetadata => vec![("expression", "string")],
        ComponentName::SimilaritySection => vec![
            ("expression", "string"),
            ("state", "password_app_state"),
        ],
        ComponentName::VectorSpace => vec![("state", "password_app_state")],
        ComponentName::PageNotFound => vec![("route", "string_vec")],
        ComponentName::QueryAccountDialog => vec![("show_query_dialog", "bool")],
        ComponentName::ReceiveSol => vec![("show_receive_modal", "bool")],
        ComponentName::SendSol => vec![("show_send_modal", "bool")],
        ComponentName::TransactionButtons => vec![("on_menu_change", "menu_option_handler")],
        ComponentName::WikidataMemeView => vec![("meme", "wikidata_meme")],
        ComponentName::WorkflowStepView => vec![("step", "workflow_step")],
        ComponentName::WorkflowMemeView => vec![("workflow", "workflow_meme")],
        _ => vec![],
    }
}
