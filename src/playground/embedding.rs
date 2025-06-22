use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
//use serde_json::{Value, json}; // For Wikidata JSON
use serde_json::{Value}; // For Wikidata JSON
//use plotters::prelude::*; // For visualization
use std::fs::write; // For saving generated program
use gloo_timers::future::TimeoutFuture;
use log::error;
use crate::model::wasm_bert::{WasmBertEmbedder, WasmSentimentAnalyzer};

const STYLE: Asset = asset!("/assets/file_upload.css");

// Use the WASM-compatible BERT functionality
#[derive(Clone, Debug, PartialEq)]
struct Multivector { scalar: f32, vector: [f32; 3] }

// Replace placeholder functions with actual WASM-compatible implementations
fn rust_bert_embed(text: &str) -> Vec<f32> { 
    let mut embedder = WasmBertEmbedder::new(384);
    embedder.embed_text(text)
}

fn pca_reduce(embedding: &[f32]) -> [f32; 3] { 
    // Simple PCA-like reduction to 3D
    let mut reduced = [0.0; 3];
    for (i, &val) in embedding.iter().take(3).enumerate() {
        reduced[i] = val;
    }
    // Add some dimensionality reduction logic here if needed
    reduced
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedWord {
    word: String,
    primary_emoji: String,
    secondary_emoji: String,
    wikidata: Option<String>,
    embedding: Vec<f32>,
    multivector: Multivector,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ProcessingFile {
    name: String,
    contents: String,
    annotations: Vec<AnnotatedWord>,
    progress: usize,
    total_lines: usize,
}

pub  struct UploadedFile {
    name: String,
    contents: String,
    annotations: Vec<AnnotatedWord>,
    generated_program: String,
}

//fn main() {
//    dioxus::launch(app);
//}

pub fn EmbeddingApp() -> Element {
    let mut enable_directory_upload = use_signal(|| false);
    let mut files_uploaded = use_signal(|| Vec::new() as Vec<UploadedFile>);
    let mut hovered = use_signal(|| false);
    let mut visualize_trigger = use_signal(|| false);
    let mut currently_processing_file = use_signal::<Option<ProcessingFile>>(|| None);

    // Emoji DFA and Wikidata annotation
    let annotate_word = |word: &str, wikidata_data: &Value| -> AnnotatedWord {
	let (primary, secondary, description, wikidata) = match word.to_lowercase().as_str() {
	    "incredible" => ("😊", "❤️", "adjective, positive sentiment", Some("Q21168966 (incredible, skos:altLabel: unbelievable)")),
	    "awesome" => ("😊", "❤️", "adjective, positive sentiment", Some("Q107027826 (awesome, instance of: single)")),
	    "explorer" => ("🧑‍🚀", "🏠", "noun, concrete entity", Some("Q11900058 (explorer, instance of: profession)")),
	    "journey" => ("🧑‍🚀", "🌌", "noun, abstract concept", Some("Q607862 (journey, instance of: travel)")),
	    "inspiration" => ("🧑‍🚀", "🌌", "noun, abstract concept", Some("Q1751856 (inspiration, instance of: mental state)")),
	    _ => ("🌟", "✨", "other, general modifier", None),
	};
	let wikidata_text = wikidata.map(|wd| {
	    wikidata_data["results"]["bindings"]
		.as_array()
		.unwrap()
		.iter()
		.filter(|b| b["item"]["value"].as_str().unwrap().contains(&wd[..8]))
		.map(|b| format!(
		    "{}: {}",
		    b["propertyLabel"]["value"].as_str().unwrap(),
		    b["valueLabel"]["value"].as_str().unwrap()
		))
		.collect::<Vec<_>>()
		.join("; ")
	}).unwrap_or_default();
	let embedding = rust_bert_embed(&format!("{} {}", word, wikidata_text)); // Embed word + Wikidata
	let reduced = pca_reduce(&embedding);
	let multivector = Multivector {
	    scalar: embedding[0] * if primary == "😊" && secondary == "❤️" { 1.6 } else { 1.0 },
	    vector: if wikidata.as_deref().map(|wd| wd.contains("Q21168966")).unwrap_or(false) {
		[reduced[0], reduced[1], reduced[2] * 1.3]
	    } else {
		reduced
	    },
	};
	AnnotatedWord {
	    word: word.to_string(),
	    primary_emoji: primary.to_string(),
	    secondary_emoji: secondary.to_string(),
	    wikidata: wikidata.map(|s| s.to_string()),
	    embedding,
	    multivector,
	}
    };

    // Generate Rust program from annotations
    let generate_program = |annotations: &[AnnotatedWord]| -> String {
	let struct_defs = r#"
#[derive(Debug)]
struct Annotation {
    word: String,
    primary_emoji: String,
    secondary_emoji: String,
    wikidata: Option<String>,
}
"#;
	let data = annotations.iter().map(|anno| format!(
	    r#"Annotation {{
		word: "{}".to_string(),
		primary_emoji: "{}".to_string(),
		secondary_emoji: "{}".to_string(),
		wikidata: {},"#,
	    anno.word,
	    anno.primary_emoji,
	    anno.secondary_emoji,
	    anno.wikidata.as_ref().map(|wd| format!("Some(\"{}\".to_string())", wd)).unwrap_or("None".to_string())
	)).collect::<Vec<_>>().join("\n    ");
	let main_fn = format!(
	    r#"
fn main() {{
    let annotations = vec![    {{}}
    ];
    for anno in annotations {{
	println!("{:?}", anno);
    }}
}}
"#,
	    data
	);
	format!("{}\n{}", struct_defs, main_fn)
    };

    // Wikidata SPARQL query
    let fetch_wikidata_graph = move || async move {
	let query = r#"
	    SELECT ?item ?itemLabel ?property ?propertyLabel ?value ?valueLabel WHERE {
		VALUES ?item { wd:Q21168966 wd:Q107027826 wd:Q11900058 wd:Q607862 wd:Q1751856 }
		?item ?prop ?value .
		?property wikibase:directClaim ?prop .
		SERVICE wikibase:label { bd:serviceParam wikibase:language "en". }
	    }
	    LIMIT 50
	"#;
	let client = reqwest::Client::new();
	client
	    .get("https://query.wikidata.org/sparql")
	    .query(&[("query", query), ("format", "json")])
	    .send()
	    .await
	    .unwrap()
	    .json::<Value>()
	    .await
	    .unwrap()
    };

    // Visualize multivectors
    let visualize_multivectors = move |annotations: &[AnnotatedWord]| {
	// let root = BitMapBackend::new("output.png", (800, 600)).into_drawing_area();
	// let mut chart = ChartBuilder::on(&root)
	//     .build_cartesian_3d(-1.0..1.0, -1.0..1.0, -1.0..1.0)
	//     .unwrap();
	// chart.draw_series(annotations.iter().map(|anno| {
	//     let color = match (anno.primary_emoji.as_str(), anno.secondary_emoji.as_str(), anno.wikidata.as_deref()) {
	//         ("😊", "❤️", Some(wd)) if wd.contains("Q21168966") => RED,
	//         ("😊", "❤️", Some(wd)) if wd.contains("Q107027826") => PURPLE,
	//         ("🧑‍🚀", "🏠", Some(wd)) if wd.contains("Q11900058") => BLUE,
	//         ("🧑‍🚀", "🌌", Some(wd)) if wd.contains("Q607862") => GREEN,
	//         ("🧑‍🚀", "🌌", Some(wd)) if wd.contains("Q1751856") => CYAN,
	//         _ => BLACK,
	//     };
	//     Circle::new(
	//         (anno.multivector.vector[0], anno.multivector.vector[1], anno.multivector.vector[2]),
	//         5,
	//         color.filled(),
	//     )
	// })).unwrap();
	// root.present().unwrap();
    };

    let read_files = move |file_engine: Arc<dyn FileEngine>| async move {
	let wikidata_data = fetch_wikidata_graph().await;
	let files = file_engine.files();
	for file_name in &files {
	    // Set initial processing state
	    currently_processing_file.set(Some(ProcessingFile {
		name: file_name.clone(),
		..Default::default()
	    }));

	    // Yield to allow UI to update
	    TimeoutFuture::new(1).await;

	    if let Some(contents) = file_engine.read_file_to_string(file_name).await {
		let lines: Vec<&str> = contents.lines().collect();
		let total_lines = lines.len();

		// Update processing state with total lines
		if let Some(mut p) = currently_processing_file.write().as_mut() {
		    p.total_lines = total_lines;
		}

		for (i, line) in lines.iter().enumerate() {
		    let words = line.split_whitespace().collect::<Vec<_>>();

		    if let Some(mut p) = currently_processing_file.write().as_mut() {
			let line_annotations = words.iter().map(|&w| annotate_word(w, &wikidata_data)).collect::<Vec<_>>();
			p.annotations.extend(line_annotations);
			p.contents.push_str(line);
			p.contents.push('\n');
			p.progress = i + 1;
		    }

		    // Yield every 10 lines to allow UI updates
		    if i % 10 == 0 || i == total_lines - 1 {
			TimeoutFuture::new(1).await;
		    }
		}

		// Processing complete - move to uploaded files
		if let Some(p_file) = currently_processing_file.take() {
		    let generated_program = generate_program(&p_file.annotations);

		    #[cfg(not(target_arch = "wasm32"))]
		    {
			if let Err(e) = write("generated.rs", &generated_program) {
			    error!("Failed to write generated.rs: {}", e);
			}
		    }

		    visualize_multivectors(&p_file.annotations);
		    visualize_trigger.set(true);

		    files_uploaded.write().push(UploadedFile {
			name: file_name.clone(),
			contents: contents.clone(),
			annotations: p_file.annotations,
			generated_program,
		    });
		}
	    }
	}
    };

    let upload_files = move |evt: FormEvent| async move {
	if let Some(file_engine) = evt.files() {
	    read_files(file_engine).await;
	}
    };

    rsx! {
	document::Link { rel: "stylesheet", href: STYLE }

	h1 { "Semantic Hyperspace File Upload" }
	p { "Drop a .txt, .md, .rs, or .js file to process for semantic hyperspace" }
	button { onclick: move |_| files_uploaded.write().clear(), "Clear files" }
	button { onclick: move |_| visualize_trigger.set(true), "Refresh Visualization" }

	div {
	    label { r#for: "directory-upload", "Enable directory upload" }
	    input {
		r#type: "checkbox",
		id: "directory-upload",
		checked: enable_directory_upload,
		oninput: move |evt| enable_directory_upload.set(evt.checked()),
	    }
	}

	div {
	    label { r#for: "textreader", "Upload text files for semantic analysis" }
	    input {
		r#type: "file",
		accept: ".txt,.md,.rs,.js",
		multiple: true,
		name: "textreader",
		directory: enable_directory_upload,
		onchange: upload_files,
	    }
	}

	div {
	    id: "drop-zone",
	    background_color: if hovered() { "lightblue" } else { "lightgray" },
	    ondragover: move |evt| {
		evt.prevent_default();
		hovered.set(true)
	    },
	    ondragleave: move |_| hovered.set(false),
	    ondrop: move |evt| async move {
		evt.prevent_default();
		hovered.set(false);
		if let Some(file_engine) = evt.files() {
		    read_files(file_engine).await;
		}
	    },
	    "Drop files here"
	}

	// Show currently processing file with progress
	if let Some(file) = currently_processing_file() {
	    div {
		div {
		    style: "border: 2px solid #007bff; padding: 15px; margin: 10px 0; border-radius: 5px; background-color: #f8f9fa;",
		    h3 { "🔄 Processing: {file.name}" }

		    if file.total_lines > 0 {
			div {
			    div {
				style: "margin: 10px 0;",
				"Progress: {file.progress} / {file.total_lines} lines ({((file.progress as f32 / file.total_lines as f32) * 100.0) as i32}%)"
			    }
			    div {
				style: "width: 100%; background-color: #e9ecef; border-radius: 10px; overflow: hidden;",
				div {
				    style: "width: {((file.progress as f32 / file.total_lines as f32) * 100.0) as i32}%; height: 20px; background-color: #007bff; transition: width 0.3s ease;",
				}
			    }
			}
		    }

		    div {
			style: "max-height: 200px; overflow-y: auto; border: 1px solid #dee2e6; padding: 10px; background-color: white;",
			pre { "{file.contents}" }
		    }

		    if !file.annotations.is_empty() {
			div {
			    h4 { "Annotations so far:" }
			    div {
				style: "max-height: 150px; overflow-y: auto;",
				ul {
				    for anno in file.annotations.iter().take(20) {
					li {
					    style: "margin: 2px 0;",
					    span { "{anno.word} {anno.primary_emoji}{anno.secondary_emoji}" }
					    {
						if let Some(wd) = &anno.wikidata {
						    rsx! { span { " | Wikidata: {wd}" } }
						} else {
						    rsx! {}
						}
					    }
					}
				    }
				}
				if file.annotations.len() > 20 {
				    div { p { "... and {file.annotations.len() - 20} more annotations" } }
				}
			    }
			}
		    }
		}
	    }
	}

	div {
	    if visualize_trigger() {
		img { src: "output.png", alt: "Semantic Hyperspace Visualization" }
	    }
	}

	ul {
	    for file in files_uploaded.read().iter().rev() {
		li {
		    span { "{file.name}" }
		    pre { "{file.contents}" }
		    ul {
			for anno in file.annotations.iter() {
			    li {
				span { "{anno.word} {anno.primary_emoji}{anno.secondary_emoji}" }
				{
				    if let Some(wd) = &anno.wikidata {
					rsx! { span { " | Wikidata: {wd}" } }
				    } else {
					rsx! {}
				    }
				}
				//				anno.wikidata.as_ref().map(|wd| rsx! {
				//    span { " | Wikidata: {wd}" }
				//}).unwrap_or_default()

				//                                anno.wikidata.as_ref().map(|wd| rsx! {
				//                                    span { " | Wikidata: {wd}" }
				//                                })
			    }
			}
		    }
		    h3 { "Generated Program" }
		    pre { "{file.generated_program}" }
		}
	    }
	}
    }
}


/*

panicked at src\playground\embedding.rs:184:59:
called `Result::unwrap()` on an `Err` value: Error { kind: Unsupported, message: "operation not supported on this platform" }

Stack:

__wbg_get_imports/imports.wbg.__wbg_new_8a6f238a6ece86ea/<@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:2156:21
logError@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:89:18
__wbg_get_imports/imports.wbg.__wbg_new_8a6f238a6ece86ea@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:2155:66
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.__wbg_new_8a6f238a6ece86ea externref shim@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[31306]:0x9173fd
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.console_error_panic_hook::Error::new::h8fd2edbe98e06cd1@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[17508]:0x83ebc6
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.console_error_panic_hook::hook_impl::h77cf582950d9cd9a@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[5042]:0x5f0325
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.console_error_panic_hook::hook::h0ab20291ffe33955@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[27968]:0x8f4ee5
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::ops::function::Fn::call::h1ca563c1039484d9@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[23466]:0x8b54f8
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.std::panicking::rust_panic_with_hook::h885a2e0f7cb6094c@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[9255]:0x7171dc
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.std::panicking::begin_panic_handler::{{closure}}::hf631a5b49e1d8742@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[12230]:0x79dcea
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.std::sys::backtrace::__rust_end_short_backtrace::h055708a75b60b26d@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[31787]:0x918caa
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.__rustc[90fd524071601a38]::rust_begin_unwind@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[29682]:0x909e28
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::panicking::panic_fmt::h3b9ae52b0e452a99@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[29683]:0x909e55
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::result::unwrap_failed::h4997b33bfd193b1e@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[14368]:0x7e7729
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.solfunmeme_dioxus::playground::embedding::EmbeddingApp::{{closure}}::{{closure}}::hc2e21a74df566889@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[540]:0x1e551a
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.solfunmeme_dioxus::playground::embedding::EmbeddingApp::{{closure}}::{{closure}}::hf5c04643e3598636@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[2001]:0x442cb9
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<F as dioxus_core::events::SpawnIfAsync<dioxus_core::events::AsyncMarker>>::spawn::{{closure}}::hd7ee48db4fb81d1b@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3827]:0x56c1e1
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::tasks::<impl dioxus_core::runtime::Runtime>::handle_task_wakeup::{{closure}}::h2d01714c68bce8ea@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[2295]:0x47debd
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::runtime::Runtime::with_scope_on_stack::hccd6ac0a01822a88@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[16693]:0x829504
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::tasks::<impl dioxus_core::runtime::Runtime>::handle_task_wakeup::h490d8ea0acaae079@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[1846]:0x4216f0
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::poll_tasks::hadbe31ce12026b80@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[534]:0x1dfc40
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::process_events::h040450ae91a4d672@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[738]:0x27e39e
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::wait_for_work::{{closure}}::{{closure}}::h1c46ba05bb5a708a@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3165]:0x512d65
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::wait_for_work::{{closure}}::h4d643fdcee6c5d1f@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[800]:0x2a60a7
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<futures_util::future::future::fuse::Fuse<Fut> as core::future::future::Future>::poll::hc0da195f6e64fc2b@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[6969]:0x68c4df
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<core::pin::Pin<P> as core::future::future::Future>::poll::hadf2c52ddefaad9e@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[17700]:0x843778
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<core::pin::Pin<P> as core::future::future::Future>::poll::h562b42a9de4244e7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[17693]:0x8434db
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_util::future::future::FutureExt::poll_unpin::hcfb9c82339b44eb7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[19526]:0x86c96a
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::run::{{closure}}::{{closure}}::{{closure}}::hf003091e11f912c3@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[5094]:0x5f53d1
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut::h6e9852333d87fe76@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[18308]:0x851fa4
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::run::{{closure}}::{{closure}}::h66004d6db5b7b838@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[1266]:0x37e413
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<futures_util::future::poll_fn::PollFn<F> as core::future::future::Future>::poll::h80de519d6eebcbb5@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[19204]:0x865ce7
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::run::{{closure}}::h0a9eed6897b591b7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[484]:0x1a83cc
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::launch::launch_virtual_dom::{{closure}}::h41798842f442b82a@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[2542]:0x4ac7fe
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::task::singlethread::Task::run::h13ecf366154bc9f0@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3433]:0x539209
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::QueueState::run_all::h70f14342fe4233ff@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3145]:0x50fea8
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::Queue::new::{{closure}}::h560a93fd1e9870d2@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[16652]:0x8283aa
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<dyn core::ops::function::FnMut<(A,)>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::hd78e7de9b3f87e79@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[10719]:0x75f300
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.closure1784 externref shim@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[31206]:0x916b91
__wbg_adapter_78@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:365:10
real@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:187:20
VoidFunction*__wbg_get_imports/imports.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5/<@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:2316:23
logError@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:89:18
__wbg_get_imports/imports.wbg.__wbg_queueMicrotask_97d92b4fcc8a61c5@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:2315:77
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.__wbg_queueMicrotask_97d92b4fcc8a61c5 externref shim@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[31797]:0x918d09
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::queueMicrotask::hc5abcc71589e7094@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[21008]:0x88a8e1
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::Queue::schedule_task::h611ccb5a25c86fe7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[5230]:0x60231a
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::Queue::push_task::h8a13acd7498c4911@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[24617]:0x8c7624
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::task::singlethread::Task::force_wake::{{closure}}::h72aa79143235a2c1@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[24618]:0x8c7661
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::Queue::with::h485020ac8fadb8ac@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[23418]:0x8b488c
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::task::singlethread::Task::force_wake::h70af2b8b00c80936@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[27847]:0x8f35dc
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::task::singlethread::Task::wake::h505af3cc2e417564@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[10572]:0x75893a
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::task::singlethread::Task::into_raw_waker::raw_wake::h06ef41bca13a0c41@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[23420]:0x8b4916
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::task::wake::Waker::wake::h26f801928613f7e3@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[14358]:0x7e7219
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_core::task::__internal::atomic_waker::AtomicWaker::wake::h377f9e5f6be63292@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[6573]:0x670453
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_channel::mpsc::UnboundedSenderInner<T>::queue_push_and_signal::haf8db4a3d6fd6914@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[19714]:0x870813
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_channel::mpsc::UnboundedSender<T>::do_send_nb::hd32d811b4c6c0887@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[5581]:0x621c51
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_channel::mpsc::UnboundedSender<T>::unbounded_send::hdfaf01d3fe5fbf5f@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[25746]:0x8d7b22
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<dioxus_core::tasks::LocalTaskHandle as futures_task::arc_wake::ArcWake>::wake_by_ref::h4c457ee2c7a80c61@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[13143]:0x7beaf2
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_task::arc_wake::ArcWake::wake::h73dd946f71743640@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[18222]:0x8500c7
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_task::waker::wake_arc_raw::he134ab3428f84b10@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[21679]:0x89722b
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::task::wake::Waker::wake::hc3d9fa88c42b1f65@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[14273]:0x7e469c
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_channel::oneshot::Inner<T>::drop_tx::hb31b9ee74101f764@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[1715]:0x402776
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<futures_channel::oneshot::Sender<T> as core::ops::drop::Drop>::drop::hc1471c1d04f96cb4@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[24839]:0x8cab0d
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::ptr::drop_in_place<futures_channel::oneshot::Sender<core::result::Result<wasm_bindgen::JsValue,wasm_bindgen::JsValue>>>::hd5181360cafc3e24@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[25315]:0x8d18b9
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_channel::oneshot::Sender<T>::send::hdad7ecb7fcaa3dd4@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[12566]:0x7aa28e
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<dioxus_web::file_engine::WebFileEngine as dioxus_html::file_data::FileEngine>::read_file_to_string::{{closure}}::{{closure}}::h6589c88017a16e22@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[9547]:0x726673
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<dyn core::ops::function::FnMut<()>+Output = R as wasm_bindgen::closure::WasmClosure>::describe::invoke::h673fc5d5691b3c85@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[12190]:0x79c498
__wbg_adapter_63@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:335:10
real@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:187:20
EventHandlerNonNull*__wbg_get_imports/imports.wbg.__wbg_setonload_1302417ca59f658b/<@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:2534:9
logError@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:89:18
__wbg_get_imports/imports.wbg.__wbg_setonload_1302417ca59f658b@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus.js:2533:72
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.__wbg_setonload_1302417ca59f658b externref shim@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[31444]:0x917ca3
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.web_sys::features::gen_FileReader::FileReader::set_onload::h103e01240f31bd92@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[15885]:0x813971
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<dioxus_web::file_engine::WebFileEngine as dioxus_html::file_data::FileEngine>::read_file_to_string::{{closure}}::h87d551daf2946dc7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[910]:0x2e2ff8
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<core::pin::Pin<P> as core::future::future::Future>::poll::h2267a6ead59ccaca@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[16101]:0x819970
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.solfunmeme_dioxus::playground::embedding::EmbeddingApp::{{closure}}::{{closure}}::hc2e21a74df566889@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[540]:0x1e4fd1
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.solfunmeme_dioxus::playground::embedding::EmbeddingApp::{{closure}}::{{closure}}::hf5c04643e3598636@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[2001]:0x442cb9
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<F as dioxus_core::events::SpawnIfAsync<dioxus_core::events::AsyncMarker>>::spawn::{{closure}}::hd7ee48db4fb81d1b@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3827]:0x56c1e1
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::tasks::<impl dioxus_core::runtime::Runtime>::handle_task_wakeup::{{closure}}::h2d01714c68bce8ea@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[2295]:0x47debd
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::runtime::Runtime::with_scope_on_stack::hccd6ac0a01822a88@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[16693]:0x829504
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::tasks::<impl dioxus_core::runtime::Runtime>::handle_task_wakeup::h490d8ea0acaae079@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[1846]:0x4216f0
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::poll_tasks::hadbe31ce12026b80@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[534]:0x1dfc40
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::process_events::h040450ae91a4d672@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[738]:0x27e39e
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::wait_for_work::{{closure}}::{{closure}}::h1c46ba05bb5a708a@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3165]:0x512d65
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_core::virtual_dom::VirtualDom::wait_for_work::{{closure}}::h4d643fdcee6c5d1f@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[800]:0x2a60a7
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<futures_util::future::future::fuse::Fuse<Fut> as core::future::future::Future>::poll::hc0da195f6e64fc2b@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[6969]:0x68c4df
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<core::pin::Pin<P> as core::future::future::Future>::poll::hadf2c52ddefaad9e@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[17700]:0x843778
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<core::pin::Pin<P> as core::future::future::Future>::poll::h562b42a9de4244e7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[17693]:0x8434db
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.futures_util::future::future::FutureExt::poll_unpin::hcfb9c82339b44eb7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[19526]:0x86c96a
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::run::{{closure}}::{{closure}}::{{closure}}::hf003091e11f912c3@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[5094]:0x5f53d1
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut::h6e9852333d87fe76@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[18308]:0x851fa4
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::run::{{closure}}::{{closure}}::h66004d6db5b7b838@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[1266]:0x37e413
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.<futures_util::future::poll_fn::PollFn<F> as core::future::future::Future>::poll::h80de519d6eebcbb5@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[19204]:0x865ce7
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::run::{{closure}}::h0a9eed6897b591b7@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[484]:0x1a83cc
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.dioxus_web::launch::launch_virtual_dom::{{closure}}::h41798842f442b82a@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[2542]:0x4ac7fe
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::task::singlethread::Task::run::h13ecf366154bc9f0@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3433]:0x539209
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::QueueState::run_all::h70f14342fe4233ff@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[3145]:0x50fea8
solfunmeme_dioxus-e2950b5a8ecdfd23.wasm.wasm_bindgen_futures::queue::Queue::new::{{closure}}::h560a93fd1e9870d2@http://192.168.1.82:8080/assets/dioxus/solfunmeme-dioxus_bg.wasm:wasm-function[16652]:0x8283aa


patch_console.js:1:737

*/


#[test]
fn test() {
//    let first = dioxus_ssr::render_element(EmbeddingApp);
    //println!("{}",first)
}
