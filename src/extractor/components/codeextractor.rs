use dioxus::prelude::*;

use crate::extractor::components::appheader::ExtractorAppHeader;
use crate::extractor::components::clearbutton::ClearButton;
use crate::extractor::components::codeextractor::dioxus_elements::FileEngine;
use crate::extractor::components::dropzone::DropZone;
use crate::extractor::components::filedisplay::FileDisplay;
use crate::extractor::components::fileinput::FileInput;
use crate::extractor::components::progress::ProcessingIndicator;
use crate::extractor::components::welcome::WelcomeMessage;
use crate::extractor::styles::STYLE;
use crate::extractor::system::clipboard::copy_all_snippets_combined;
use crate::extractor::system::clipboard::copy_to_clipboard;
use crate::extractor::model::files::process_files;
use crate::extractor::types::CodeSnippet;
use crate::extractor::types::ExtractedFile;
use crate::extractor::types::ProcessingFile;
        
use dioxus_logger::tracing::info;
use std::sync::Arc;

#[component]
fn CodeExtractor() -> Element {
    let files = use_signal(|| Vec::new() as Vec<ExtractedFile>);
    let processing_file = use_signal::<Option<ProcessingFile>>(|| None);
    let hovered = use_signal(|| false);
    let copied_snippets = use_signal(|| std::collections::HashSet::new() as std::collections::HashSet<String>);

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
