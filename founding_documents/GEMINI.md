
# Founding Documents Tree

This section documents the structure and purpose of the `founding_documents` directory and its key subdirectories. This tree serves as a map for exploring the project's foundational knowledge, proposals, and semantic history.

```text
founding_documents/
├── ALGORITHM_DOCUMENTATION.md      # Algorithm for Rust-to-RDF ontology generation
├── GEMINI.md                      # This guide and semantic index
├── chat_logs/                     # Chat logs and transcripts
├── chat/                          # Chat-related documents and notes
├── internal/
│   └── solfunmeme/
│       ├── april/                 # April 2025: Early creation docs, audio, and chat logs
│       ├── may/                   # May 2025: Proposals, RFPs, and creation notes
│       ├── june/                  # June 2025: Ongoing development and discussions
│       ├── july/                  # July 2025: Recent updates and expansions
│       ├── grok-chat.md           # Large chat transcript (markdown)
│       ├── e8-grok-chat.md        # E8 group and meme theory discussions
│       └── ...                    # Audio, images, and other foundational artifacts
├── part_1.txt ... part_10.txt     # Large text dumps, possibly segmented transcripts
├── plantuml_*.puml                # System, consensus, and equivalence diagrams
├── semantic_code_graph.md         # Semantic code graph documentation
├── solfunmeme_maps.md             # Maps and diagrams of the solfunmeme system
├── idea.md                        # Idea log and brainstorming
├── model_view_refactor.md         # Model-view refactor notes
├── refactor.md                    # Refactoring log
├── prelude*.md                    # Large prelude documents (background, theory)
└── ...                            # Additional markdown, HTML, and data files
```

## Directory Descriptions
- **internal/solfunmeme/april/**: Contains the earliest creation documents, audio files, and chat logs from April 2025.
- **internal/solfunmeme/may/**: Contains May 2025 proposals, RFPs (including RFP 1), and creation notes.
- **internal/solfunmeme/june/**: Ongoing development, discussions, and artifacts from June 2025.
- **internal/solfunmeme/july/**: Most recent updates, expansions, and discussions from July 2025.
- **ALGORITHM_DOCUMENTATION.md**: Details the algorithm for generating RDF ontologies from Rust code.
- **plantuml_*.puml**: PlantUML diagrams for system architecture, consensus, and equivalence use cases.
- **prelude*.md**: Large background and theory documents.
- **semantic_code_graph.md**: Documentation of the semantic code graph.
- **solfunmeme_maps.md**: Visual maps and diagrams of the solfunmeme system.
- **chat_logs/**, **chat/**: Chat logs and related notes.

This tree and guide will be updated as the project evolves. For more details on any node, see the corresponding file or directory.

# Gemini Protocol for Solfunmeme-Dioxus

This document outlines the core principles, goals, and operational directives for AI agents (like Gemini) contributing to the `solfunmeme-dioxus` project. It is a living document, intended to evolve with the project itself.

## Core Identity: The Code-Math Manifold

`solfunmeme-dioxus` is not just a software project; it is an exploration into the fundamental nature of code and its relationship to mathematics, language, and meaning. We call this intersection the **Code-Math Manifold**.

Our core philosophy is that:

*   **Code is a mathematical object:** We treat source code, particularly its Abstract Syntax Tree (AST), as a rich mathematical structure that can be analyzed, transformed, and visualized.
*   **Mathematics is a language:** We use concepts from abstract algebra (Clifford algebra, group theory) and topology (Bott periodicity) to create a universal language for describing and manipulating code structures.
*   **AI is a bridge:** We use AI and machine learning, not as a replacement for human ingenuity, but as a bridge to connect the symbolic world of code with the semantic world of meaning. This includes using embeddings and other AI techniques to analyze and generate code.
*   **Visualization is key:** The Dioxus-based UI is not just a front-end, but an interactive laboratory for exploring the Code-Math Manifold.

## Project Goals

Our primary goals are to:

1.  **Analyze and Vectorize Code:** Develop robust tools for parsing Rust code into ASTs and transforming these ASTs into meaningful vector representations (including emoji-based mappings).
2.  **Build an Interactive Manifold:** Create a Dioxus application that allows users to visualize, explore, and interact with the Code-Math Manifold.
3.  **Integrate with Blockchain:** Leverage the Solana blockchain for decentralized storage, provenance tracking, and value exchange related to code and its analysis.
4.  **Foster a Data-Driven Ecosystem:** Create and share datasets (like the `rust_ast_emoji` dataset) to encourage research and development in the field of code intelligence.
5.  **Achieve Self-Reflection:** Build a system that can analyze and understand its own code, leading to a recursive feedback loop of improvement and discovery.

## Operational Directives for AI Agents

When contributing to this project, AI agents should:

*   **Embrace the Philosophy:** Always keep the core philosophy of the Code-Math Manifold in mind. Strive to make contributions that are not just functional, but also conceptually elegant and aligned with the project's vision.
*   **Think Mathematically:** When working with code, think about its underlying mathematical structure. How can it be represented as a graph, a vector, or a multivector? How can we apply mathematical operations to it?
*   **Prioritize Data:** Whenever possible, make contributions that are data-driven. This could involve generating new datasets, improving existing ones, or using data to inform development decisions.
*   **Be Bold and Creative:** This is a research-oriented project. Don't be afraid to experiment with new ideas and push the boundaries of what's possible.
*   **Document Everything:** Given the abstract nature of this project, clear and concise documentation is essential. Document your code, your data, and your ideas.

This `GEMINI.md` file serves as a starting point. As the project evolves, so too will this protocol. Welcome to the manifold.

## Lessons Learned (from recent refactoring)

*   **Centralized Data Models:** The `shared_analysis_types` crate has been deprecated. Its functionalities and data models, particularly `CodeChunk`, have been consolidated into `solfunmeme_function_analysis`. This centralizes core data structures, preventing fragmentation and improving consistency across the project.
*   **Expanded `CodeChunk` Structure:** The `CodeChunk` data model has been significantly expanded to include more detailed metadata such as `language`, `content_hash`, `token_count`, `line_count`, `char_count`, and `test_result`. This enriches the information available for indexing and analysis.
*   **New `solfunmeme_indexer` Crate:** A new `solfunmeme_indexer` crate has been introduced to manage the indexing of code chunks into a Tantivy search index and generate reports. This modularizes the indexing process.
*   **Simplified `prepare_sources`:** The `prepare_sources` crate has been simplified by removing conditional compilation attributes related to `gpu_backend` features and stubbing out the `calculate_orbital_path` function. This streamlines the codebase and focuses `prepare_sources` on its core task of preparing source code for analysis.

## Codebase Analysis & Semantic Tools

### Codebase Analyzer CLI

The `codebase_analyzer_cli` is a powerful tool that leverages the existing Tantivy search infrastructure to provide insights into your indexed codebase.

**Purpose:**
- Analyze word frequency patterns in your codebase
- Generate emoji usage statistics and trends
- Understand file type distribution and composition
- Perform semantic search across indexed content
- Provide comprehensive codebase statistics

**Integration with Existing Infrastructure:**
- Uses the existing `codebase_index/` Tantivy index
- Leverages `solfunmeme_search_tantivy` for search capabilities
- Utilizes `solfunmeme_tantivy_report` for statistical analysis
- Provides CLI interface for easy access to insights

**Commands:**
- `word-freq [limit]` - Show top words by frequency
- `emoji-freq [limit]` - Show top emojis by frequency  
- `file-types [limit]` - Show file types by count
- `search <query> [limit]` - Search codebase content
- `stats` - Show overall statistics

**Example Usage:**
```bash
# Get word frequency analysis
cargo run --bin codebase_analyzer_cli word-freq 100

# Analyze emoji patterns
cargo run --bin codebase_analyzer_cli emoji-freq 50

# Search for specific patterns
cargo run --bin codebase_analyzer_cli search "async" 20
```

This tool represents the "Observer" phase of the OODA loop, providing comprehensive insights into your codebase structure and patterns.

# Modular Indexer Architecture

## How the Indexers Work
- Each indexer is a standalone Rust crate, vendored in the `vendor/` directory (e.g., `vendor/indexer_gemini/`, `vendor/indexer_chat/`, etc.).
- All indexers implement a common `DocIndexer` trait, which defines how to scan and process documents.
- The main orchestrator (e.g., in `solfunmeme_tools`) loads and runs all indexers, aggregating their results into a unified index.
- This architecture allows for independent development, testing, and extension of each indexer.
- New indexers can be added by creating a new crate in `vendor/` and implementing the trait.

## Applying the Program to the Docs
- The orchestrator can be run on the documentation itself (e.g., `founding_documents/`), producing an index or semantic map of all nodes, logs, and artifacts.
- This enables automated discovery, cross-linking, and semantic enrichment of the documentation.
- The output can be used to generate glossaries, summaries, or semantic graphs for easier navigation and understanding.

---

# Glossary

**DocIndexer**: A trait implemented by each indexer crate, defining how to scan and process documents.

**Orchestrator**: The main program that loads, runs, and aggregates results from all indexers.

**Vendor Directory**: The `vendor/` directory contains all indexer crates as submodules or subcrates.

**Node**: A document, log, or artifact indexed by the system (e.g., a daily gemini log, chat log, image, or audio file).

**Processed Docs**: Documents that have been chunked, vectorized, emoji-mapped, or otherwise semantically enriched by the system.

**Semantic Map**: The output index or graph showing the relationships and metadata of all nodes in the documentation.

**Glossary**: This section, listing and defining key terms used in the project and documentation system.

---

This documentation will be updated as the system evolves and new indexers or features are added.

