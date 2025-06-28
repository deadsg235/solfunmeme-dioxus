use dioxus::prelude::*;
use crate::extractor::components::clearbutton::ClearButton;
use crate::extractor::components::fileinput::FileInput;
use crate::extractor::components::dropzone::DropZone;
use crate::extractor::components::progress::ProcessingIndicator;
use crate::extractor::components::filedisplay::FileDisplay;
use crate::extractor::components::welcome::WelcomeMessage;

//use std::collections::HashSet;
//use crate::extractor::Arc;
//use crate::extractor::FileEngine;
//use crate::extractor::ExtractedFile;

//use crate::extractor::ProcessingFile;
use crate::extractor::components::appheader::ExtractorAppHeader;
use crate::extractor::system::files::process_files;
use crate::extractor::system::clipboard::copy_to_clipboard;
//use crate::extractor::system::extract::extract_code_snippets;
use crate::extractor::types::ExtractedFile;
use crate::extractor::types::ProcessingFile;
use std::sync::Arc;

use crate::Route::Clusters;
use crate::Route::Dashboard;
use crate::Route::Extras;
//use crate::extractor::components::app::dioxus_elements::FileEngine;
//use crate::extractor::components::appheader::ExtractorAppHeader;
//use crate::extractor::components::clearbutton::ClearButton;
use crate::extractor::components::codeextractor::dioxus_elements::FileEngine;
//use crate::extractor::components::dropzone::DropZone;
//use crate::extractor::components::dropzone::dioxus_elements::FileEngine;
//use crate::extractor::components::filedisplay::FileDisplay;
//use crate::extractor::components::fileinput::FileInput;
use crate::extractor::components::fileresult::FileResults;
use crate::extractor::components::filesummary::FileSummary;
//use crate::extractor::components::progress::ProcessingIndicator;
use crate::extractor::components::upload::FileUploadArea;
//use //crate::extractor::components::welcome::WelcomeMessage;
use crate::extractor::model::content_hash::create_content_hash;
use crate::extractor::model::download::create_download_handler;
use crate::extractor::model::extract::process_file_engine;
use crate::extractor::model::token_count::estimate_token_count;
use crate::extractor::styles::STYLE;
use crate::extractor::system::clipboard::copy_all_snippets_combined;
use crate::extractor::system::clipboard::create_copy_handler;
use crate::extractor::system::files::create_download_filename;
use crate::extractor::system::files::create_file_reader;
//use crate::extractor::system::process_file::process_file;
use crate::extractor::system::test_code::test_code_snippet;
use crate::extractor::types::CodeSnippet;
//use crate::extractor::types::ExtractedFile;
////use crate::extractor::types::ProcessingFile;
use crate::extractor::types::TestResult;
use crate::header::ActiveAccountDropDown;
use crate::header::ConnectWalletModalModal;
use crate::header::Header;
use crate::model::UseConnections;
use crate::model::crypto::SolanaEncryption;
use crate::model::{cluster_store::ClusterStore, AdapterCluster, MyCluster};        
use crate::password_manager::PasswordApp;
use crate::playground::MenuOption::Airdrop;
use crate::playground::MenuOption::MemeManagement;
use crate::playground::MenuOption::Memes;
use crate::playground::MenuOption::MetaMemeOperations;
use crate::playground::MenuOption::ReceiveSol;
use crate::playground::MenuOption::SendSol;
use crate::playground::MenuOption::StylingAndEmojis;
use crate::playground::MenuOption;
use crate::playground::test_components::ComponentName::ActionButton;
use crate::playground::test_components::ComponentName::AddClusterModal;
use crate::playground::test_components::ComponentName::AddPasswordForm;
use crate::playground::test_components::ComponentName::CardHeader;
use crate::playground::test_components::ComponentName::ClusterInfo;
use crate::playground::test_components::ComponentName::CodeDisplay;
use crate::playground::test_components::ComponentName::ComponentMemeExplorer;
use crate::playground::test_components::ComponentName::ComponentMemeView;
use crate::playground::test_components::ComponentName::ConnectWalletFirst;
use crate::playground::test_components::ComponentName::ConnectionButtons;
use crate::playground::test_components::ComponentName::CoreButtons;
use crate::playground::test_components::ComponentName::CreateButton;
use crate::playground::test_components::ComponentName::CryptoAppHeader;
use crate::playground::test_components::ComponentName::CryptoButtons;
use crate::playground::test_components::ComponentName::CryptoErrorMessage;
use crate::playground::test_components::ComponentName::CryptoFrontendApp;
use crate::playground::test_components::ComponentName::EncryptionForm;
use crate::playground::test_components::ComponentName::ExpressionCard;
use crate::playground::test_components::ComponentName::ExpressionInputs;
use crate::playground::test_components::ComponentName::ExpressionList;
use crate::playground::test_components::ComponentName::ExpressionMetadata;
use crate::playground::test_components::ComponentName::ExpressionTypeSelector;
use crate::playground::test_components::ComponentName::Footer;
use crate::playground::test_components::ComponentName::GitParser2;
use crate::playground::test_components::ComponentName::InputField;
use crate::playground::test_components::ComponentName::InputSection;
use crate::playground::test_components::ComponentName::LoginScreen;
use crate::playground::test_components::ComponentName::ManagementButtons;
use crate::playground::test_components::ComponentName::MemeCardHeader;
use crate::playground::test_components::ComponentName::MemeCategoryView;
use crate::playground::test_components::ComponentName::MemesFooter;
use crate::playground::test_components::ComponentName::MetadataInputs;
use crate::playground::test_components::ComponentName::NavWalletItem;
use crate::playground::test_components::ComponentName::Notification2;
use crate::playground::test_components::ComponentName::Notification;
use crate::playground::test_components::ComponentName::PageNotFound;
use crate::playground::test_components::ComponentName::PasswordAppHeader;
use crate::playground::test_components::ComponentName::PasswordDetail;
use crate::playground::test_components::ComponentName::PasswordErrorMessage;
use crate::playground::test_components::ComponentName::PasswordList;
use crate::playground::test_components::ComponentName::PasswordMainApp;
use crate::playground::test_components::ComponentName::PingCluster;
use crate::playground::test_components::ComponentName::QueryAccountDialog;
use crate::playground::test_components::ComponentName::QueryCoinDialog;
use crate::playground::test_components::ComponentName::SearchInput;
use crate::playground::test_components::ComponentName::SignInWithSolana;
use crate::playground::test_components::ComponentName::SignMessage;
use crate::playground::test_components::ComponentName::SignTx;
use crate::playground::test_components::ComponentName::SimilaritySection;
use crate::playground::test_components::ComponentName::SuccessMessage;
use crate::playground::test_components::ComponentName::TextAreaField;
use crate::playground::test_components::ComponentName::TransactionButtons;
use crate::playground::test_components::ComponentName::VectorSpace;
use crate::playground::test_components::ComponentName::WelcomeScreen;
use crate::playground::test_components::ComponentName::WikidataMemeExplorer;
use crate::playground::test_components::ComponentName::WikidataMemeView;
use crate::playground::test_components::ComponentName::WorkflowMemeExplorer;
use crate::playground::test_components::ComponentName::WorkflowMemeView;
use crate::playground::test_components::ComponentName::WorkflowStepView;
use crate::views::accounts::Accounts;
use crate::views::accounts::ClusterSuccess;
use crate::views::accounts::TokenAccountCard;
use crate::views::accounts::TxCard;
//use crate::views::crypto_frontend::AppHeader;
use dioxus::prelude::*;
use dioxus_clipboard::prelude::use_clipboard;

use dioxus_logger::tracing::{Level, info};

#[component]
fn CodeExtractor() -> Element {
    let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let mut hovered = use_signal(|| false);
    let mut copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        process_files(file_engine, files, processing_file).await;
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    
    //let copy_to_clipboard_handler = move |text: String, snippet_id: String| {
    //copy_to_clipboard(text, snippet_id, copied_snippets);
//    };
    let copy_to_clipboard_handler = move |(text, snippet_id): (String, String)| {
	info!("going to copy to clipboard: {:?}", text);
	info!("going to copy to clipboard: {:?}", text);
	copy_to_clipboard(text, snippet_id, copied_snippets);  
    };
    
    let copy_all_snippets_handler = move |snippets: Vec<CodeSnippet>| {
	info!("going to copy all to clipboard: {:?} {:?}", snippets, copied_snippets);
        copy_all_snippets_combined(snippets, copied_snippets);
    };

    files.with(|files_vec| {
        rsx! {
            style { "{STYLE}" }
            
            div { class: "code-extractor",
                ExtractorAppHeader {}
                
                ClearButton { files, copied_snippets }
                
                FileInput { upload_files }
                
                DropZone { hovered, read_files }
                
                ProcessingIndicator { processing_file }
                
                // Results
                for file in files_vec.iter() {
                    FileDisplay {
                        file: file.clone(),
                        copied_snippets,
                        copy_all_snippets: copy_all_snippets_handler,
                        copy_to_clipboard: copy_to_clipboard_handler
                    }
                }

                if files_vec.is_empty() && processing_file().is_none() {
                    WelcomeMessage { show: true }
                }
            }
        }
    })
}
