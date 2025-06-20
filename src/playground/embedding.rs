use std::sync::Arc;
use dioxus::prelude::*;
use dioxus::{html::HasFileData, prelude::dioxus_elements::FileEngine};
use serde_json::{Value, json}; // For Wikidata JSON
//use plotters::prelude::*; // For visualization
use std::fs::write; // For saving generated program

const STYLE: Asset = asset!("/assets/file_upload.css");

// Simulated dependencies (replace with actual imports)

#[derive(Clone)]
struct Multivector { scalar: f32, vector: [f32; 3] }
fn rust_bert_embed(text: &str) -> Vec<f32> { vec![0.1, 0.2, 0.3, /* 384D */] } // Placeholder
fn pca_reduce(embedding: &[f32]) -> [f32; 3] { [embedding[0], embedding[1], embedding[2]] } // Placeholder

#[derive(Clone)]
pub struct AnnotatedWord {
    word: String,
    primary_emoji: String,
    secondary_emoji: String,
    wikidata: Option<String>,
    embedding: Vec<f32>,
    multivector: Multivector,
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
    let annotations = vec![
        {}
    ];
    for anno in annotations {{
        println!("{{:?}}", anno);
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
            if let Some(contents) = file_engine.read_file_to_string(file_name).await {
                // Parse contents (simple word splitting)
                let words = contents.split_whitespace().collect::<Vec<_>>();
                let annotations = words.iter().map(|&w| annotate_word(w, &wikidata_data)).collect::<Vec<_>>();
                
                // Generate program
                let generated_program = generate_program(&annotations);
                write("generated.rs", &generated_program).unwrap(); // Save to file

                // Visualize
                visualize_multivectors(&annotations);
                visualize_trigger.set(true);

                files_uploaded.write().push(UploadedFile {
                    name: file_name.clone(),
                    contents: contents.clone(),
                    annotations,
                    generated_program,
                });
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
				        rsx! { span { " | Wi
kidata: {wd}" } }
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
