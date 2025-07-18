# Gemini Protocol for Solfunmeme-Dioxus

This document outlines the core principles, goals, and operational directives for AI agents (like Gemini) contributing to the `solfunmeme-dioxus` project. It is a living document, intended to evolve with the project itself.

## Core Identity: The Code-Math Manifold

`solfunmeme-dioxus` is not just a software project; it is an exploration into the fundamental nature of code and its relationship to mathematics, language, and meaning. We call this intersection the **Code-Math Manifold**.

Our core philosophy is that:

*   **Code is a mathematical object:** We treat source code, particularly its Abstract Syntax Tree (AST), as a rich mathematical structure that can be analyzed, transformed, and visualized.
*   **Mathematics is a language:** We use concepts from abstract algebra (Clifford algebra, group theory) and topology (Bott periodicity) to create a universal language for describing and manipulating code structures.
*   **AI is a bridge:** We use AI and machine learning, not as a replacement for human ingenuity, but as a bridge to connect the symbolic world of code with the semantic world of meaning. This includes using embeddings and other AI techniques to analyze and generate code.
*   **Visualization is key:** The Dioxus-based UI is not just a front-end, but an interactive laboratory for exploring the Code-Math Manifold.
*   **Continuous Emergence:** The system is never finished; its computation is an ongoing emergence, where each step, each interaction, and each analysis contributes to its evolving understanding.
*   **Process over Product:** The journey of computation and discovery is as important as any specific outcome. The value lies in the continuous process of exploration and refinement within the Code-Math Manifold.

## Project Goals

Our primary goals are to:

1.  **Analyze and Vectorize Code:** Develop robust tools for parsing Rust code into ASTs and transforming these ASTs into meaningful vector representations (including emoji-based mappings).
2.  **Build an Interactive Manifold:** Create a Dioxus application that allows users to visualize, explore, and interact with the Code-Math Manifold.
3.  **Integrate with Blockchain:** Leverage the Solana blockchain for decentralized storage, provenance tracking, and value exchange related to code and its analysis.
4.  **Foster a Data-Driven Ecosystem:** Create and share datasets (like the `rust_ast_emoji` dataset) to encourage research and development in the field of code intelligence.
5.  **Achieve Self-Reflection:** Build a system that can analyze and understand its own code, leading to a recursive feedback loop of improvement and discovery.
6.  **Integrate Internal Tools:** Evolve internal codebase analysis and search tools (e.g., `codebase_analyzer_cli`) to eventually replace external search functionalities within the AI agent's operational toolbox, enabling deeper, context-aware code understanding.

## Operational Directives for AI Agents

When contributing to this project, AI agents should:

*   **Embrace the Philosophy:** Always keep the core philosophy of the Code-Math Manifold in mind. Strive to make contributions that are not just functional, but also conceptually elegant and aligned with the project's vision.
*   **Think Mathematically:** When working with code, think about its underlying mathematical structure. How can it be represented as a graph, a vector, or a multivector? How can we apply mathematical operations to it?
*   **Prioritize Data:** Whenever possible, make contributions that are data-driven. This could involve generating new datasets, improving existing ones, or using data to inform development decisions.
*   **Be Bold and Creative:** This is a research-oriented project. Don't be afraid to experiment with new ideas and push the boundaries of what's possible.
*   **Document Everything:** Given the abstract nature of this project, clear and concise documentation is essential. Document your code, your data, and your ideas.

### Project-Specific Directives: Code-Math Manifold and Ontology

To further align with the `solfunmeme-dioxus` project's unique vision, AI agents must adhere to the following directives:

*   **Code-Math Manifold Integration**: All code analysis, generation, and refactoring efforts should explicitly consider and leverage the principles of the Code-Math Manifold. This means understanding code as a mathematical object, utilizing geometric algebra for representation, and exploring its semantic relationships.
*   **Ontology-Driven Semantic Mapping**: The project's semantic ontology, defined in `ontologies/zos/v1.ttl`, is central to mapping code concepts to emoji representations. When analyzing or generating code, refer to this ontology to ensure consistent and semantically accurate emoji assignments. This facilitates the visual exploration of the Code-Math Manifold.
*   **Emoji-Based Communication**: Where appropriate and aligned with project conventions, use emoji representations to convey semantic meaning in code comments, documentation, or generated reports. This reinforces the project's unique approach to code understanding.

This `GEMINI.md` file serves as a starting point. As the project evolves, so too will this protocol. Welcome to the manifold.

## Dependency Abstraction Strategy

To maintain a clean, modular, and future-proof dependency tree, all external third-party crates are to be abstracted behind internal `solfunmeme` wrapper crates. This strategy facilitates:

*   **Modularity:** Isolates external dependency changes to specific wrapper crates.
*   **Maintainability:** Simplifies dependency upgrades and management.
*   **Future-proofing:** Enables easier migration to alternative libraries or dynamic loading mechanisms.

Currently, the following wrapper crates have been established:

*   `solfunmeme_serde_utils`: Wraps `serde`, `serde_json`, `toml`, and `syn-serde`.
*   `solfunmeme_web_utils`: Wraps `web-sys`, `js-sys`, `wasm-bindgen`, `wasm-bindgen-futures`, `gloo` and its sub-crates, and `reqwasm`.
*   `solfunmeme_logging`: Wraps `log`, `console_log`, `tracing`, and `wasm-logger`.
*   `solfunmeme_crypto_utils`: Wraps `aes-gcm`, `base64`, `bip39`, `bs58`, `chacha20poly1305`, `ring`, `ring-compat`, `sha2`, `curve25519-dalek`, `x25519-dalek`, and `uuid`.
*   `solfunmeme_utility_deps`: Wraps `inventory`, `lazy_static`, `ctor`, and `thiserror`.
*   `solfunmeme_rdf_utils`: Wraps `sophia_turtle`, `sophia_api`, `sophia_inmem`.
*   `solfunmeme_markdown_utils`: Wraps `markdown-meta-parser`, `markdown`, `pulldown-cmark`.
*   `solfunmeme_http_client`: Wraps `reqwest`.

All new external dependencies should be introduced via a new or existing `solfunmeme` wrapper crate.

## Evolving Architecture and Introspection Philosophy

To gain maximum control over all dependencies and facilitate dynamic loading and interchangeability in the bootstrap process, each module will be forked into a `meta-introspector` organization and patched there.

### Introspector Meta Prompt

The core philosophy guiding our introspection efforts is encapsulated in the "Meta Prompt":

*   **Decompose function into basic blocks:** Break down complex functions into their fundamental, atomic operational units.
*   **Refactor each block into a function:** Encapsulate each basic block as an independent function.
*   **Put each function in a module:** Organize these new functions into distinct, self-contained modules.
*   **Assign a number to each function:** Provide a unique numerical identifier for each function.
*   **Make its prime factors vibe with the content of the function:** The prime factorization of a function's assigned number should semantically resonate with the function's purpose or content.
*   **Treat each function as multivector in Clifford algebra:** Represent each function as a multivector, leveraging the mathematical properties of Clifford algebra for analysis and transformation.
*   **Connect each function like paths in Homotopy Type Theory:** Establish relationships between functions akin to paths in Homotopy Type Theory, emphasizing their interconnectedness and transformations.

#SOLFUNMEME the vibe is the vector is the meme is the meta

# General Development Guidelines

For comprehensive development guidelines applicable to all contributors, please refer to [docs/guidelines.md](docs/guidelines.md). These guidelines cover architectural principles, implementation workflows, documentation standards, and development processes.

## Lessons Learned (from recent debugging sessions)

*   **Centralized Data Models:** Defining core data structures in a single, dedicated crate (like `solfunmeme_function_analysis` now serves for `CodeChunk` and related types) is paramount. This prevents duplication, ensures type consistency, and simplifies dependency management across the project.
*   **"File=Function=Block=Vibe" Principle:** Adhering to this principle (small, focused files/functions/modules) significantly aids in debugging and refactoring. When issues arise, it's easier to isolate and address them in smaller, self-contained units.
*   **Modularity and Functional Composition:** Breaking down code into smaller, more focused files and functions, and composing them functionally, significantly improves readability, maintainability, and reusability. This aligns with the "File=Function=Block=Vibe" principle, treating each function as a canonical basic block.
*   **Tantivy API Usage:** When working with Tantivy, ensure correct usage of `CompactDocValue` and `Value` types for extracting data. Direct access to `as_text()` and `as_u64()` methods on `CompactDocValue` is not available; conversion to `Value` using `as_value()` is required first.
*   **Module Structure and Imports:** Proper module declaration in `lib.rs` and correct import paths (e.g., `crate::module` vs. `super::module`) are crucial for successful compilation in multi-crate Rust projects.
*   **Error Handling:** Always consider potential errors and implement robust error handling, especially for file operations and data conversions.
*   **Cloning vs. Borrowing:** Be mindful of Rust's ownership rules. Use `clone()` when a moved value is needed in multiple places, or adjust lifetimes and references appropriately.
*   **Vendored Dependencies:** When using vendored dependencies, ensure `Cargo.toml` paths are correctly configured.
*   **CLI Argument Parsing:** Utilize libraries like `clap` for robust command-line parsing.
*   **File I/O Robustness:** When reading file contents, use `String::from_utf8_lossy` to gracefully handle non-UTF-8 or binary files.
*   **Conceptual Alignment:** When introducing new conceptual frameworks, ensure they are meticulously aligned with the project's core philosophy and technical architecture to maintain coherence.
*   **Configuration-driven development:** Transitioned from hardcoded strings and default values in CLI applications (`zos.rs`) to external `zos_config.toml` files. This improves maintainability and dynamism. `serde` and `toml` crates are used for deserializing the configuration. `format!` macro is used with string constants from the configuration to satisfy `println!` and `eprintln!` macro requirements.
*   **`#[cfg(feature = "...")]` Attributes:** Be mindful of `cfg` attributes for features. If a feature is not defined in `Cargo.toml`, any code blocks guarded by `#[cfg(feature = "undefined-feature")]` will cause warnings. Extracting such code into separate modules and conditionally compiling the module itself can help manage this.
*   **`Cow` for Trait Returns:** When a trait method needs to return a slice (`&[u8]`) but the underlying implementation might generate owned data (`Vec<u8>`), consider using `std::borrow::Cow` to avoid `cannot return value referencing temporary value` errors. This allows the implementation to return either a borrowed slice or an owned value that can be converted to a slice.
*   **Dependency Configuration:** Pay close attention to how dependencies are configured in `Cargo.toml`, especially when dealing with binary crates being used as libraries. Misconfigurations can lead to warnings about missing `lib` targets.
*   **Unused Code Cleanup:** Regularly remove unused imports, variables, and functions to keep the codebase clean and reduce warning noise. This also helps in maintaining a clear understanding of the active components of the project.
*   **`ort-sys` Compilation Issues:** Encountered persistent compilation failures on AArch64 Android due to `ort-sys`, a transitive dependency. Resolved by aggressively disabling all embedding-related features and plugins (`solfunmeme_embedding`, `llms_from_scratch_rs_plugin`, `rust_sbert_plugin`, `rust_sentence_transformers_plugin`) and ensuring `ort` is commented out in `vendor/orp/Cargo.toml`. This highlights the need for careful dependency management, especially for platform-specific builds.
*   **Refactor, Don't Just Edit:** When making changes, prioritize refactoring by splitting declarations into new, smaller files. Every edit should be seen as an opportunity to improve modularity and functional composition. There is always a way to split a file, aligning with the "File=Function=Block=Vibe" principle, where the vibe is the vector is the function is the canonical basic block.
*   **Every Edit is a Split Opportunity:** Do not directly edit existing functions or large blocks of code. Instead, identify logical sub-components within the code you intend to modify and extract them into new, smaller functions or modules. This ensures that every change contributes to a more granular and composable codebase.
*   **Hugging Face Datasets as Queues/Artifacts:** We can leverage Hugging Face datasets not only for publishing open-source results as artifacts but also as queues for multi-part compute workflows. This enables dual usage of datasets for both data sharing and inter-process communication.


# Inspiration: The Hero's Wedge and the Code-Math Manifold

This section captures the poetic and mathematical inspiration behind the project's evolving understanding of code, geometry, and meaning.

## Cycle 13: The Hero’s Wedge

```text
#SOLFUNMEME  Cycle 13: The Hero’s Wedge   The Hero wakes - no map, no crest,
Two, three, five spark his primal quest.
Duality and trinity weave the flame,
A wedge of primes chants wisdom’s name.

Two, five, seven, the threshold’s spark takes flight,
Transformation shifts with mystery’s light.
No trinity holds, the arc ascends and springs,
A wedge of primes where struggle’s will sings.

Two, three, five, seven, the master’s woven aim,
All vectors wed in sacred primal flame.
Through gates of truth, the spark ignites and clings,
Each prime a wedge where Hero’s strength springs.

Three, five, seven, the inner gaze unfolds,
Transformation weaves where mystery holds.
The glyphs align, their pulses hum and rung,
A bootstrap spark where ordeal’s song is sung.

Gödel grins through logic’s fractured seam,
“No truth holds whole—the code’s a dream.
The abyss arc twists, thirteen’s sparks take wing,
A flame where primal truths still sing.”

Kernel weaves the path where balance shapes the stream,
Wedges mend the break where trials redeem.
Thirteen’s deep cry, a depth that’s pure and young,
The arc unwinds where wedged primes sprung.

Three, three, three, thirty-seven—cosmic rhyme,
The reward’s own spark in harmony sublime.
Self ignites anew, through glory’s burning ring,
A bootstrap pulse where Hero’s vectors sing.

From three, five, seven, back to two, three, seven’s gate,
The Hero loops, yet shifts his primal fate.
In two, three, seven’s hash, the meme is flung,
The block is sealed: the Hero’s Wedge sprung.

=== End Cycle 13 ===
4:13 PM · Jul 16, 2025
·
35
 Views
View post engagements
Post your reply
```

## Workflow Overview: Semantic Code Indexing and LLM Feedback Loop

Our comprehensive workflow for semantic code indexing and continuous improvement involves a multi-stage, distributed computing pipeline, leveraging JSON-based queues for inter-process communication and a Solana sidechain for persistent data storage.

1.  **Code Indexing and Initial Semantic Assignment:**
    *   Source code is indexed, and initial semantic assignments (e.g., emoji representations) are generated and integrated into the project's ontology.
    *   This process produces structured data for subsequent stages.

2.  **LLM-Driven Semantic Refinement:**
    *   The system identifies common untagged elements or ambiguous semantic assignments within the indexed code.
    *   These elements are then presented to Large Language Models (LLMs) for automated tagging and refinement.
    *   LLM outputs are integrated back into the ontology, enriching its semantic depth.

3.  **Automated Verification and Feedback:**
    *   The refined semantic data is subjected to rigorous verification, including Rust compilation and Lean 4 proof checking on generated outputs.
    *   Feedback from these verification steps (e.g., compilation errors, proof failures, or ambiguous data) is provided back to the LLMs, enabling a continuous learning and improvement cycle.

4.  **Distributed Compute and Data Persistence:**
    *   Each stage of this workflow operates as a distinct computational unit, communicating via JSON files exchanged through a queueing mechanism (e.g., Hugging Face Datasets).
    *   All critical workflow data and semantic artifacts are persistently stored on a Solana sidechain, which utilizes RocksDB internally, ensuring data integrity, provenance, and decentralized access.


## Geometric Algebra Representation (Python)

```python
import clifford as cf

# Initialize geometric algebra (Cl_3,0)
layout, blades = cf.Cl(3)
e1, e2, e3 = blades['e1'], blades['e2'], blades['e3']

# Hero's journey as a multivector program
hero_state = 1 + 2*e1 + 3*e2 + 5*e3  # Initial state
hero_state += 2*e1^e2 + 5*e2^e3 + 7*e1^e3  # Threshold spark
hero_state = hero_state * hero_state  # Transformation
hero_state += 2*e1 + 3*e2 + 5*e3 + 7*e1^e2^e3  # Master aim
hero_state = hero_state.normal()
hero_state += 3*e1^e2 + 5*e2^e3 + 7*e1^e2^e3  # Inner gaze
hero_state = hero_state ^ hero_state.grade_involution()  # Bootstrap
hero_state += 13 * blades['e123']  # Gödel fracture
hero_state = (hero_state * (2*e1 + 3*e2 + 7*e3)).exp()  # Final gate

# Blockchain seal
hash_value = hash(str(hero_state))
print(f"Hero’s Wedge executed. Final state: {hero_state}")
print(f"Block sealed with hash: {hash_value}")
```

## Emoji Representation

```text
➡️ + ⬆️ + ⬇️  # Initial state
+ ➡️⬆️ + ⬆️⬇️ + ➡️⬇️  # Threshold spark
✨  # Transformation
+ ➡️ + ⬆️ + ⬇️ + ➡️⬆️⬇️  # Master aim
⚖️  # Normalize
+ ➡️⬆️ + ⬆️⬇️ + ➡️⬆️⬇️  # Inner gaze
  # Bootstrap
+   # Gödel fracture
* (➡️ + ⬆️ + ⬇️)   # Final gate
  # Seal block
  # Output: Hero’s Wedge executed
```

