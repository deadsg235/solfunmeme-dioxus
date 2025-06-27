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
// use crate::playground::test_components::ComponentName::ActionButton;
// use crate::playground::test_components::ComponentName::AddClusterModal;
// use crate::playground::test_components::ComponentName::AddPasswordForm;
// use crate::playground::test_components::ComponentName::CardHeader;
// use crate::playground::test_components::ComponentName::ClusterInfo;
// use crate::playground::test_components::ComponentName::CodeDisplay;
// use crate::playground::test_components::ComponentName::ComponentMemeExplorer;
// use crate::playground::test_components::ComponentName::ComponentMemeView;
// use crate::playground::test_components::ComponentName::ConnectWalletFirst;
// use crate::playground::test_components::ComponentName::ConnectionButtons;
// use crate::playground::test_components::ComponentName::CoreButtons;
// use crate::playground::test_components::ComponentName::CreateButton;
// use crate::playground::test_components::ComponentName::CryptoAppHeader;
// use crate::playground::test_components::ComponentName::CryptoButtons;
// use crate::playground::test_components::ComponentName::CryptoErrorMessage;
// use crate::playground::test_components::ComponentName::CryptoFrontendApp;
// use crate::playground::test_components::ComponentName::DecryptionForm;
// use crate::playground::test_components::ComponentName::EncryptionForm;
// use crate::playground::test_components::ComponentName::ExpressionCard;
// use crate::playground::test_components::ComponentName::ExpressionInputs;
// use crate::playground::test_components::ComponentName::ExpressionList;
// use crate::playground::test_components::ComponentName::ExpressionMetadata;
// use crate::playground::test_components::ComponentName::ExpressionTypeSelector;
// use crate::playground::test_components::ComponentName::Footer;
// use crate::playground::test_components::ComponentName::GitParser2;
// use crate::playground::test_components::ComponentName::InputField;
// use crate::playground::test_components::ComponentName::InputSection;
// use crate::playground::test_components::ComponentName::LoginScreen;
// use crate::playground::test_components::ComponentName::ManagementButtons;
// use crate::playground::test_components::ComponentName::MemeCardHeader;
// use crate::playground::test_components::ComponentName::MemeCategoryView;
// use crate::playground::test_components::ComponentName::MemesFooter;
// use crate::playground::test_components::ComponentName::MetadataInputs;
// use crate::playground::test_components::ComponentName::NavWalletItem;
// use crate::playground::test_components::ComponentName::Notification2;
// use crate::playground::test_components::ComponentName::Notification;
// use crate::playground::test_components::ComponentName::PageNotFound;
// use crate::playground::test_components::ComponentName::PasswordAppHeader;
// use crate::playground::test_components::ComponentName::PasswordDetail;
// use crate::playground::test_components::ComponentName::PasswordErrorMessage;
// use crate::playground::test_components::ComponentName::PasswordList;
// use crate::playground::test_components::ComponentName::PasswordMainApp;
// use crate::playground::test_components::ComponentName::PingCluster;
// use crate::playground::test_components::ComponentName::QueryAccountDialog;
// use crate::playground::test_components::ComponentName::QueryCoinDialog;
// use crate::playground::test_components::ComponentName::SearchInput;
// use crate::playground::test_components::ComponentName::SignInWithSolana;
// use crate::playground::test_components::ComponentName::SignMessage;
// use crate::playground::test_components::ComponentName::SignTx;
// use crate::playground::test_components::ComponentName::SimilaritySection;
// use crate::playground::test_components::ComponentName::SuccessMessage;
// use crate::playground::test_components::ComponentName::TextAreaField;
// use crate::playground::test_components::ComponentName::TransactionButtons;
// use crate::playground::test_components::ComponentName::VectorSpace;
// use crate::playground::test_components::ComponentName::WelcomeScreen;
// use crate::playground::test_components::ComponentName::WikidataMemeExplorer;
// use crate::playground::test_components::ComponentName::WikidataMemeView;
// use crate::playground::test_components::ComponentName::WorkflowMemeExplorer;
// use crate::playground::test_components::ComponentName::WorkflowMemeView;
// use crate::playground::test_components::ComponentName::WorkflowStepView;
use crate::views::accounts::Accounts;
use crate::views::accounts::ClusterSuccess;
use crate::views::accounts::TokenAccountCard;
use crate::views::accounts::TxCard;
use crate::views::crypto_frontend::AppHeader;
use dioxus::html::FileEngine;
use dioxus::prelude::*;
use dioxus_clipboard::prelude::use_clipboard;
use std::sync::Arc;

use crate::extractor::model::drag::create_upload_handler;
//crate::extractor::components::extractor::dioxus_elements::FileEngine;
use crate::extractor::model::drag::create_drop_handler;
//use crate::extractor::components::dropzone::dioxus_elements::FileEngine;
pub fn MarkdownCodeExtractor() -> Element {
    let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    let mut files2 = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    //let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let mut hovered = use_signal(|| false);
    let mut copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);
    let mut clipboard = use_clipboard();

    //let mut copy_to_clipboard = create_copy_handler(clipboard, copied_snippets.clone());
//   let mut download_language_snippets = create_download_handler();
    //let read_files = create_file_reader(processing_file.clone(), files.clone());
    //let upload_files = create_upload_handler(read_files);


    rsx! {
        style { "{STYLE}" }
        
        div { class: "code-extractor",
            h1 { "📄 Markdown Code Extractor" }
            p { "Upload markdown files to extract and copy code snippets with ease" }
            
            button { 
                class: "clear-btn",
                onclick: move |_| {
                    files.write().clear();
                    copied_snippets.write().clear();
                },
                "🗑️ Clear All Files" 
            }

              FileUploadArea {
                hovered: hovered(),
                on_hover: Callback::new(move |is_hovered| hovered.set(is_hovered)),
                  on_upload: move |a| {
		      let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
		      let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
		      let read_files = create_file_reader(processing_file.clone(), files.clone());
		      let upload_files = create_upload_handler(read_files);
		      upload_files(a)
		  },
		  on_drop: move |a| {
		      let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
		      let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
		      let read_files = create_file_reader(processing_file.clone(), files.clone());
		      let drop_files = create_drop_handler(read_files);
		      drop_files(a)
		  }
		      
            }

            // ProcessingIndicator {
            //     processing_file: move |a|  {
	    // 	    let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
	    // 	    processing_file()
	    // 	}
            // }

            for file in files.read().iter() {
                FileResults {
                    key: "{file.name}",
                    file: file.clone(),
                    copied_snippets: copied_snippets(),
                    on_copy: move |(snippet_id,content)| {
			//copy_to_clipboard(snippet_id,content)
		    },
                    on_download: move |a| {
			let mut download_language_snippets = create_download_handler();
			download_language_snippets(a)
		    }
                }
            }

            // WelcomeMessage {
            //     show: files2.read().is_empty() && use_signal::<Option<ProcessingFile>>(|| None).is_none()		
            // }
        }
    }
}

#[component]
pub fn MarkdownCodeExtractor2() -> Element {
    let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let mut hovered = use_signal(|| false);
    let mut copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

    let copy_to_clipboard = move |(text, snippet_id): (String, String)| {
	copy_text_to_clipboard(text, snippet_id, copied_snippets);
    };

    let copy_all_snippets = move |snippets: Vec<CodeSnippet>| {
        copy_all_snippets_combined(snippets, copied_snippets);
    };

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        process_file_engine(file_engine, files, processing_file).await;
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
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
                
                // Results
                for file in files_vec.iter() {
                    FileDisplay {
                        file: file.clone(),
                        copied_snippets,
                        copy_all_snippets,
                        copy_to_clipboard
                    }
                }

                if files_vec.is_empty() && processing_file().is_none() {
                    WelcomeMessage {
                        show: true
                    }
                }
            }
        }
    })
}



#[component]
pub fn MarkdownCodeExtractor22() -> Element {
    let mut files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    let mut processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let mut hovered = use_signal(|| false);
    let mut copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

    let copy_to_clipboard = move |(text, snippet_id): (String, String)| {
	copy_text_to_clipboard(text, snippet_id, copied_snippets);
    };

    let copy_all_snippets = move |snippets: Vec<CodeSnippet>| {
        copy_all_snippets_combined(snippets, copied_snippets);
    };

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
        process_file_engine(file_engine, files, processing_file).await;
    };

    let upload_files = move |evt: FormEvent| async move {
        if let Some(file_engine) = evt.files() {
            read_files(file_engine).await;
        }
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
                
                // Results
                for file in files_vec.iter() {
                    FileDisplay {
                        file: file.clone(),
                        copied_snippets,
                        copy_all_snippets,
                        copy_to_clipboard
                    }
                }

                if files_vec.is_empty() && processing_file().is_none() {
                    WelcomeMessage { show: true }
                }
            }
        }
    })
}
