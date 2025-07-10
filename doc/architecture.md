The Solfunmeme Dioxus project is organized into a comprehensive ecosystem of specialized crates, each serving distinct purposes within the Code-Math Manifold framework.

### Core Application Crates
- **`solfunmeme_app`**: Main application entry point orchestrating UI components, data models, and blockchain interactions
- **`workflow_manager`**: Central hub for defining, registering, and executing processing workflows
- **`task_manager`**: Manages project state, task tracking, and coordination with semantic RDF/Turtle integration
- **`playground_lib`**: Interactive sandbox environment for experimenting with project features

### Data & Analysis Core
- **`core_data_lib`**: Fundamental data structures (Meme, ZOS, ReifiedState) used across the project
- **`shared_analysis_types`**: Common data structures for code analysis (CodeChunk, AnalyzedFunction, etc.)
- **`shared_types_lib`**: General-purpose shared types and utilities
- **`model_lib`**: Core data models for user data, project information, and system states
- **`solfunmeme_core_logic`**: Central business logic for processing, analyzing, and transforming data

### Code Analysis & Processing
- **`prepare_sources`**: Processes source code files into structured CodeChunks
- **`solfunmeme_extractor`**: Extracts code snippets, functions, and relevant information from source files
- **`solfunmeme_function_analysis`**: Detailed analysis of individual functions with AST traversal and semantic summaries
- **`solfunmeme_input_fs`**: Filesystem input layer for reading code files and converting to CodeChunks
- **`solfunmeme_search_tantivy`**: Full-text search capabilities using Tantivy search engine
- **`solfunmeme_generated`**: Container for automatically generated code

### Mathematical & Embedding Systems
- **`solfunmeme_clifford`**: Clifford Algebra operations for converting BERT embeddings to multivectors and generating sieve addresses
- **`solfunmeme_embedding`**: BERT embedding generation and emoji multivector management
- **`orbital_sim_lib`**: Orbital mechanics simulation for representing abstract relationships
- **`emoji_matrix_lib`**: Matrix operations on emoji grids for visual representations

### Semantic & RDF Processing
- **`rdf_output`**: Converts structured data into RDF triples for semantic representation
- **`rdf_processing_lib`**: Utilities for parsing, querying, and serializing RDF data
- **`jsonld_plugin`**: Generic JSON-LD file I/O operations
- **`json_ld_plugin`**: JSON-LD compaction, expansion, flattening, and framing operations

### UI & Component System
- **`component_builder_lib`**: Programmatic UI component building and composition
- **`component_emoji_lib`**: Emoji representation management within UI components
- **`component_registry_lib`**: Centralized registry for dynamic component discovery and instantiation
- **`views_lib`**: Reusable UI components and page views
- **`signals_lib`**: Event management and reactive programming for UI state changes

### Blockchain & Solana Integration
- **`gitaccount`**: Pure Rust library for content-addressed, on-chain Git repositories with L1/L2 hybrid model
- **`solana_integration_lib`**: Core Solana blockchain integration (clusters, transactions, accounts)
- **`solfunmeme_solana_data`**: Solana-specific data models and bootstrap logic
- **`solfunmeme_wallet_integration`**: Solana wallet integration for authentication and transaction signing
- **`agave_solana_validator_plugin`**: Local Solana validator management
- **`solana_airdrop_service_plugin`**: Test SOL airdrop functionality for development

### NLP & Language Processing Plugins
- **`bm25_plugin`**: BM25 scoring for document ranking and relevance
- **`eliza_rs_plugin`**: ELIZA-like chatbot for conversational AI
- **`keyword_extraction_rs_plugin`**: Keyword extraction from text content
- **`layered_nlp_plugin`**: Multi-layer NLP processing (tokenization, POS tagging, NER)
- **`llms_from_scratch_rs_plugin`**: Basic Large Language Model operations
- **`nlprule_plugin`**: Grammar and style checking for text
- **`rust_sbert_plugin`**: Sentence-BERT embeddings for semantic search
- **`rust_sentence_transformers_plugin`**: Sentence encoding into vector embeddings
- **`vaporetto_plugin`**: Japanese morphological analysis and tokenization
- **`vibrato_plugin`**: Fast Japanese tokenization
- **`vtext_plugin`**: Text tokenization and vectorization (TF-IDF)

### Search & Indexing
- **`quickwit_plugin`**: Distributed search and indexing capabilities
- **`model2vec_rs_plugin`**: Model-to-vector conversion for embeddings
- **`tongrams_rs_plugin`**: N-gram language model querying
- **`solfunmeme_search_tantivy`**: Provides full-text search capabilities for code chunks using Tantivy, managing the index and returning comprehensive `SearchResult` objects.
- **`solfunmeme_tantivy_report`**: A crate for generating reports from the search index.
- **`solfunmeme_indexer`**: Orchestrates the entire codebase indexing and report generation process, utilizing `solfunmeme_input_fs` and `solfunmeme_search_tantivy`.

### Code Intelligence
- **`prepare_sources`**: Processes source code files into structured `CodeChunk` instances, preparing them for analysis and indexing.
- **`solfunmeme_extractor`**: Extracts code snippets, functions, and relevant information from source files.
- **`solfunmeme_function_analysis`**: **(Centralized Data Models)** Serves as the primary location for core data structures like `CodeChunk`, `AnalyzedFunction`, and `ClosestEmojiInfo`, along with core analysis logic.
- **`solfunmeme_input_fs`**: Provides the filesystem input layer, reading code files and converting their content into `CodeChunk` instances.

### Storage & I/O
- **`s3_plugin`**: Amazon S3 integration for cloud storage operations
- **`zip_plugin`**: Zip archive creation and extraction
- **`git_plugin`**: Git repository operations (clone, pull, commit, push)
- **`extractous_plugin`**: HTML text extraction from web content

### Scripting & Automation
- **`rhai_plugin`**: Rhai scripting engine integration
- **`steel_plugin`**: Steel Scheme-like scripting language integration
- **`gline_rs_plugin`**: Graphical line drawing algorithm processing

### Emoji & Workflow System
- **`emoji_lang_plugin`**: Emoji-based language for defining workflows with semantic mappings
- **`emoji_workflow_macro`**: Procedural macro for annotating Rust functions with emoji strings

### Micro-Component Protocol
- **`rrust_kontekst_base`**: Foundational types and traits for the Micro-Component Protocol (MCP)