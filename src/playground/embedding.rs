use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
//use serde_json::{Value, json}; // For Wikidata JSON
use serde_json::{Value}; // For Wikidata JSON
//use plotters::prelude::*; // For visualization

#[cfg(not(target_arch = "wasm32"))]
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


#[test]
fn test() {
//    let first = dioxus_ssr::render_element(EmbeddingApp);
    //println!("{}",first)
}
