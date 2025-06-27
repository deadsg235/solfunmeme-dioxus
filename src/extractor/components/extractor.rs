//use crate::Route::Extras;
//use crate::extractor::components::app::dioxus_elements::FileEngine;
use crate::extractor::components::appheader::ExtractorAppHeader;
use crate::extractor::components::clearbutton::ClearButton;
//use crate::extractor::components::codeextractor::dioxus_elements::FileEngine;
use crate::extractor::components::dropzone::DropZone;
//use crate::extractor::components::dropzone::dioxus_elements::FileEngine;
use crate::extractor::components::filedisplay::FileDisplay;
use crate::extractor::components::fileinput::FileInput;
use crate::extractor::components::fileresult::FileResults;
use crate::extractor::components::filesummary::FileSummary;
use crate::extractor::components::progress::ProcessingIndicator;
use crate::extractor::components::upload::FileUploadArea;
use crate::extractor::components::welcome::WelcomeMessage;
use crate::extractor::model::content_hash::create_content_hash;
use crate::extractor::model::download::create_download_handler;
use crate::extractor::model::extract::process_file_engine;
use crate::extractor::model::token_count::estimate_token_count;
use crate::extractor::styles::STYLE;
use crate::extractor::system::clipboard::copy_all_snippets_combined;
use crate::extractor::system::clipboard::copy_text_to_clipboard;
use crate::extractor::system::clipboard::create_copy_handler;
use crate::extractor::system::files::create_download_filename;
use crate::extractor::system::files::create_file_reader;
//use crate::extractor::system::process_file::process_file;
use crate::extractor::system::test_code::test_code_snippet;
use crate::extractor::types::CodeSnippet;
use crate::extractor::types::ExtractedFile;
use crate::extractor::types::ProcessingFile;
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
use crate::views::accounts::Accounts;
use crate::views::accounts::ClusterSuccess;
use crate::views::accounts::TokenAccountCard;
use crate::views::accounts::TxCard;
use crate::views::crypto_frontend::AppHeader;
use dioxus::html::FileEngine;
use dioxus::prelude::*;
use dioxus_clipboard::prelude::use_clipboard;
use std::sync::Arc;
//use dioxus::dioxus_html::HasFileData;
use dioxus_html::HasFileData;
use crate::extractor::model::drag::create_upload_handler;
use crate::extractor::model::drag::create_drop_handler;

#[component]
pub fn MarkdownCodeExtractor() -> Element {
    let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let mut hovered = use_signal(|| false);
    let mut copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

    // Clipboard handlers
    let copy_to_clipboard = move |(text, snippet_id): (String, String)| {
        copy_text_to_clipboard(text, snippet_id, copied_snippets);
    };

    let copy_all_snippets = move |snippets: Vec<CodeSnippet>| {
        copy_all_snippets_combined(snippets, copied_snippets);
    };

    // File processing handlers
    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        process_file_engine(file_engine, files, processing_file).await;
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
    };

    // Drag and drop handlers
    let on_drop = move |evt: DragEvent| {
        let read_files_clone = read_files.clone();
        spawn(async move {
            if let Some(file_engine) = evt.files() {
                read_files_clone(file_engine).await;
            }
        });
    };

    files.with(|files_vec| {
        rsx! {
            style { "{STYLE}" }
            
            div { class: "code-extractor",
                ExtractorAppHeader {}
                
                ClearButton { 
                    files: files,
                    copied_snippets: copied_snippets
                }
                
                FileInput { 
                    upload_files: upload_files
                }
                
                DropZone { 
                    hovered: hovered,
                    read_files: read_files
                }
                
                ProcessingIndicator { 
                    processing_file: processing_file()
                }
                
                // Results section
                for file in files_vec.iter() {
                    FileDisplay {
                        file: file.clone(),
                        copied_snippets,
                        copy_all_snippets,
                        copy_to_clipboard
                    }
                }

                // Welcome message when no files are loaded
                if files_vec.is_empty() && processing_file().is_none() {
                    WelcomeMessage { show: true }
                }
            }
        }
    })
}
