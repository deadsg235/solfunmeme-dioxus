# `solfunmeme_search_tantivy`

This crate provides functionalities for creating and managing search indexes using the Tantivy search engine, specifically tailored for the Solfunmeme project.

## Purpose

It enables efficient full-text search capabilities over processed code, documents, and other textual data within the Solfunmeme ecosystem, allowing for fast retrieval of relevant information.

## Core Functionalities

-   **Create Index**: Initialize a new Tantivy search index.
-   **Add Document**: Add documents (e.g., `CodeChunk`s) to the index.
-   **Search**: Perform full-text searches against the index and retrieve matching documents.
-   **Get Stats**: Retrieve statistics about the index.

## Usage (Conceptual)

```rust
// use solfunmeme_search_tantivy::SearchIndex;
// use shared_analysis_types::CodeChunk;

// fn main() {
//     let index_path = "./my_search_index";
//     let mut search_index = SearchIndex::new(index_path).expect("Failed to create search index");

//     let chunk = CodeChunk {
//         path: "src/main.rs".to_string(),
//         content: "fn main() { println!(\"Hello\"); }".to_string(),
//         emoji: "📄".to_string(),
//         line_start: 1,
//         line_end: 3,
//         chunk_type: "code".to_string(),
//     };

//     // Example: Add a document to the index
//     // search_index.add_chunk(&chunk).expect("Failed to add chunk");
//     // search_index.commit().expect("Failed to commit");

//     // Example: Search the index
//     // let results = search_index.search("Hello", 10).expect("Failed to search");
//     // println!("Search results (conceptual): {:?}", results);
// }
```

```