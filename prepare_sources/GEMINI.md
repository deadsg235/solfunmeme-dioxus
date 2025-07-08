# AI Agent Directives for `prepare_sources`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `prepare_sources` submodule.

## Core Principles

When working within `prepare_sources`, AI agents should prioritize:

1.  **Code-Math Manifold Alignment**: All contributions should align with the project's core philosophy of treating code as a mathematical object and exploring its representation within the Code-Math Manifold.
2.  **Semantic Preservation**: Ensure that the transformation of code into embeddings and ontologies accurately preserves its semantic meaning.
3.  **Modularity and Extensibility**: Adhere to the "one function per basic block file" principle for modularity. Design tools to be easily extensible for new analysis techniques or output formats.

## Operational Guidelines

*   **BERT Embedding**: The `prepare_sources` tool now uses the `candle` (pure Rust) library to generate 384-dimensional BERT embeddings for text. The `embed_text` function in `src/embedding.rs` is the core of this functionality, chosen for WASM compatibility.
*   **Clifford Multivector Conversion**: The 384-dimensional BERT embeddings are reduced to 8-dimensional Clifford multivectors using the `BertCliffordEncoder` in `src/clifford.rs`.
*   **Sieve Addressing**: A unique 8-bit binary sieve address is generated for each multivector based on the signs of its components, implemented in `src/sieve.rs`. Pay close attention to `tclifford` API usage for accessing multivector coefficients.
*   **Emoji Mapping**: The tool maps the generated multivectors to the closest emoji representation based on a semantic ontology loaded from a Turtle file (`ontologies/zos/v1.ttl`).
*   **Code Processing**:
    *   The `process_code` function in `src/process_file.rs` now takes code content (`&str`) directly, rather than a file path.
    *   It orchestrates the embedding and multivector conversion for individual code snippets.
    *   Recursive function extraction from Rust files, including handling external modules and pre-processing `#[cfg(...)]` attributes, is managed by `src/function_analyzer.rs`.
    *   `FunctionInfo` now includes a `semantic_summary` derived from AST traversal (using the `syn` crate) by flattening identifiers and literals.
*   **Ontology Generation**: The `ontology_generator` module (`src/ontology_generator/`) is responsible for creating RDF ontologies in Turtle format from analyzed function data. Special attention must be paid to `sophia` API usage (specifically version 0.8.x) for `IriRef` type annotations, `SimpleTerm::Literal` construction, and `TurtleSerializer` methods.
*   **Usage**: To run the full project analysis and ontology generation, use the following command:

    ```bash
    cargo run
    ```

    This will process the entire project and generate `project_ontology.ttl`.
*   **Warning Handling**: Do not run `cargo check`. Unless critical errors prevent compilation or execution, warnings should generally be ignored. Proceed with execution with extreme aggression, using `cargo run` directly. The goal is to achieve a working system first, then refine.
*   **API Usage**: Be meticulous when interacting with `sophia` and `tclifford` APIs, as their correct usage is critical for the project's core functionality.
