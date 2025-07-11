# Solfunmeme-Dioxus Project

Welcome to the Solfunmeme-Dioxus codebase! This project is a modular, semantic, and extensible system for code, meme, and knowledge management on Solana and beyond.

---

## 🟢 OODA Dashboard: Crate Matrix (Observer Phase)

We are currently in the **Observer** phase of the OODA (Observe, Orient, Decide, Act) loop: indexing, introspecting, and mapping the system.

| Crate Name                  | Emoji  | Function / OODA Phase         | Description                                                      |
|-----------------------------|--------|-------------------------------|------------------------------------------------------------------|
| solfunmeme_tools            | 🧰     | Observer (Utility)            | General utilities, logging, error handling, string/data helpers   |
| solfunmeme_indexer          | 🗂️     | Observer (Indexing)           | Indexes code chunks, builds search indices, generates reports     |
| solfunmeme_core_logic       | 🧠     | Orient (Core Logic)           | Core business/data logic, main algorithms                        |
| solfunmeme_function_analysis| 🔬     | Observer (Analysis)           | Analyzes and extracts functions from code                        |
| solfunmeme_input_fs         | 📂     | Observer (Input)              | Handles file system input, code chunking                         |
| solfunmeme_search_tantivy   | 🔎     | Observer (Search)             | Full-text search and indexing with Tantivy                       |
| solfunmeme_state            | 🗃️     | Orient (State)                | Project state management                                         |
| solfunmeme_playground       | 🧪     | Orient (Experimentation)      | Interactive playground for testing and prototyping               |
| solfunmeme_models           | 🏗️     | Orient (Models)               | Data models and types                                            |
| solfunmeme_extractor_system | 🏷️     | Observer (Extraction)         | System for extracting and labeling code/data                     |
| solfunmeme_embedding        | 🧬     | Orient (Embedding)            | Embedding and vectorization tools                                |
| solfunmeme_clifford         | 🧮     | Orient (Math)                 | Clifford algebra and mathematical operations                     |
| solfunmeme_app              | 🖥️     | Act (App/UI)                  | Main application and user interface                              |
| solfunmeme_wallet_integration| 💳    | Act (Blockchain)              | Solana wallet and blockchain integration                         |
| solfunmeme_solana_data      | 🔗     | Act (Blockchain Data)         | On-chain data and Solana-specific logic                          |
| solfunmeme_views            | 🪟     | Act (UI Components)           | UI components and views                                          |
| solfunmeme_tantivy_report   | 📊     | Orient (Reporting)            | Reporting and analytics from search/index                        |
| solfunmeme_broken_tantivy   | 🛠️     | Orient (Experimental)         | Experimental/legacy Tantivy integration                          |
| task_manager                | ✅     | Orient (Task Management)      | Task and workflow management                                     |
| workflow_manager            | 🔄     | Orient (Workflow)             | Workflow orchestration                                           |
| ...                         | ...    | ...                           | ...                                                              |

> **Legend:**
> - 🟢 Observer: Indexing, introspection, analysis
> - 🟡 Orient: Data modeling, logic, experimentation
> - 🟠 Decide: (not yet implemented)
> - 🔴 Act: UI, blockchain, user-facing actions

---

For a full semantic index and glossary, see [crates/README.md](crates/README.md) and [founding_documents/GEMINI.md](founding_documents/GEMINI.md).

This dashboard will be updated as the project evolves and new crates or features are added.

**License:** AGPL-3.0

## Solfunmeme Dioxus 

See [Introduction to Solfunmeme Dioxus](doc/introduction.md) for an overview of the project's philosophy and goals.

## Overview
See [Project Overview](doc/overview.md) for a summary of the project's core concepts and goals. 

## Status
See [Project Status](doc/status.md) for current development status.

## Functionality
See [Project Functionality](doc/functionality.md) for details on features and supported data formats.

## Project Architecture
See [Project Architecture](doc/architecture.md) for a detailed breakdown of the project's crate structure.

## Languages
See [Supported Languages](doc/languages.md) for a list of programming languages and formats supported.

## Theories
See [Underlying Theories](doc/theories.md) for the theoretical foundations of the project.

#### Self hosting
#### Reproducible
#### Secure
#### Audited
#### Declarative
#### Emergent
#### Omni Combinator

## Low Value Activities
See [Low Value Activities](doc/low_value.md) for a discussion on activities considered low value for the project. 

## Systems
See [Systems Overview](doc/systems.md) for a list of systems and platforms integrated or considered.

## Ideas
See [Project Ideas](doc/ideas.md) for a collection of ideas and future directions.

## Next Steps
See [Next Steps & Roadmap](doc/next_steps.md) for the current development roadmap and immediate priorities.

## Older Stuff
See [Older Stuff](doc/older_stuff.md) for miscellaneous older notes and configurations.



## Development Setup
See [Development Setup](doc/development_setup.md) for instructions on setting up the development environment.


## Plan
See [Project Plan](doc/plan.md) for the detailed development plan.


## Multiple visualization
### ai convergence on models
#### PCA
#### Threading of dimensions
#### Area or surface of points of the spaces.
#### connecting solana code to memes to llms
#### read git history
sample source code into rust
be able to read git repor (store older git objects or pack files into exec.


## More Ideas
See [More Ideas](doc/more_ideas.md) for additional project ideas.

# remove 
unimplemented
"#FIXME"

## Code Coverage
See [Code Coverage](doc/code_coverage.md) for instructions on generating code coverage reports.


## Testing
See [Testing Guidelines](doc/testing.md) for information on testing and code coverage.

## Vendored Dependencies
This project may use vendored dependencies located in the `/vendor` directory. This is done to ensure stability and control over specific versions of libraries.

Currently vendored libraries:
*   `libloading`: Used by the `zos` command for dynamic plugin loading.

## Code Generation
See [Code Generation](doc/code_generation.md) for instructions on generating code.
 

# Solfunmeme-Dioxus: The Code-Math Manifold
See [The Code-Math Manifold](doc/code_math_manifold.md) for a deep dive into the project's core philosophy.

---



### bins

$ find -name bin
./crates/prepare_sources/src/bin
./crates/solfunmeme_generated/src/bin
./crates/solfunmeme_indexer/src/bin
./crates/solfunmeme_tools/src/bin
./crates/task_manager/src/bin
./node_modules/cssesc/bin
./node_modules/jiti/bin
./node_modules/nanoid/bin
./node_modules/resolve/bin
./node_modules/sucrase/bin
./node_modules/which/bin
./src/bin
./target/debug/build/torch-sys-ef246f433224a574/out/libtorch/libtorch/bin
./vendor/addr2line/src/bin
./vendor/agave/faucet/src/bin
./vendor/agave/install/src/bin
./vendor/agave/validator/src/bin
./vendor/agave-solana-validator/faucet/src/bin
./vendor/agave-solana-validator/install/src/bin
./vendor/agave-solana-validator/net-utils/src/bin
./vendor/agave-solana-validator/validator/src/bin
./vendor/BLAKE3/test_vectors/src/bin
./vendor/candle/candle-wasm-examples/bert/src/bin
./vendor/candle/candle-wasm-examples/blip/src/bin
./vendor/candle/candle-wasm-examples/llama2-c/src/bin
./vendor/candle/candle-wasm-examples/moondream/src/bin
./vendor/candle/candle-wasm-examples/phi/src/bin
./vendor/candle/candle-wasm-examples/segment-anything/src/bin
./vendor/candle/candle-wasm-examples/t5/src/bin
./vendor/candle/candle-wasm-examples/whisper/src/bin
./vendor/candle/candle-wasm-examples/yolo/src/bin
./vendor/data-encoding/bin
./vendor/formats/x509-cert/fuzz/src/bin
./vendor/gimli/crates/examples/src/bin
./vendor/gloo/examples/file-hash/src/bin
./vendor/gloo/examples/markdown/src/bin
./vendor/gloo/examples/prime/src/bin
./vendor/lean4/doc/bin
./vendor/lean4/src/bin
./vendor/lean4/stage0/src/bin
./vendor/llms-from-scratch-rs/src/bin
./vendor/miniz_oxide/benches/data/bin
./vendor/nlprule/nlprule/src/bin
./vendor/objc2/crates/header-translator/src/bin
./vendor/object/crates/examples/src/bin
./vendor/quickwit/quickwit/quickwit-lambda/src/bin
./vendor/quickwit/quickwit/quickwit-metastore-utils/src/bin
./vendor/rhai/src/bin
./vendor/rust/src/bootstrap/src/bin
./vendor/rust/src/doc/book/packages/mdbook-trpl/src/bin
./vendor/rust/src/doc/book/packages/tools/src/bin
./vendor/rust/src/gcc/contrib/reghunt/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_libcxx_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_ndk_tree/aarch64-linux-android/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_ndk_tree/arm-linux-androideabi/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_ndk_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_ndk_tree/i686-linux-android/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_ndk_tree/mips64el-linux-android/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_ndk_tree/mipsel-linux-android/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_ndk_tree/x86_64-linux-android/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_tree/aarch64-linux-android/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_tree/arm-linux-androideabi/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_android_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_avr_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_baremetal_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_cross_hurd_tree/usr/i686-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_cross_hurd_tree/usr/x86_64-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_cross_linux_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_cross_linux_tree/usr/i386-unknown-linux-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_cross_linux_tree/usr/x86_64-unknown-linux-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_darwin_toolchain/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_darwin_toolchain_no_libcxx/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_freebsd64_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_freebsd_libcxx_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_freebsd_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_fuchsia_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_linux_libcxxv2_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_linux_libcxx_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_linux_libstdcxx_libcxxv2_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_linux_libstdcxx_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_msp430_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_openbsd_libcxx_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_riscv32_nogcc_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_riscv32_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_riscv64_nogcc_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_riscv64_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/basic_ve_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA-macosx/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA-new/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA-nolibdevice/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA-symlinks/opt/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA-symlinks/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA-unknown/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA-windows/Program Files/NVIDIA GPU Computing Toolkit/CUDA/v8.0/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA_102/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA_111/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA_80/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/CUDA_90/usr/local/cuda/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/debian_per_target_tree/usr/lib/llvm-14/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/fake_install_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/gcc_version_parsing1/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/gcc_version_parsing2/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/gcc_version_parsing3/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/gcc_version_parsing4/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/gcc_version_parsing5/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/hexagon_tree/Tools/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/hipspv/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_cs_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_fsf_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/mips-img-linux-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/sysroot/el/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/sysroot/mips64r6/64/el/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/sysroot/mips64r6/64/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/sysroot/mips64r6/el/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/sysroot/mips64r6/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_tree/sysroot/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_v2_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_img_v2_tree/mips-img-linux-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_mti_tree/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/mips_mti_tree/mips-mti-linux-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_32bit_linux_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_32bit_linux_tree/usr/i386-unknown-linux/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_64bit_linux_tree/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_64bit_linux_tree/usr/x86_64-unknown-linux/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_csky_linux_sdk/csky-linux-gnuabiv2/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_loongarch_linux_sdk/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_loongarch_linux_sdk/loongarch64-unknown-linux-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_riscv_elf_sdk/riscv64-unknown-elf/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_riscv_linux_sdk/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/multilib_riscv_linux_sdk/riscv64-unknown-linux-gnu/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/ohos_native_tree/llvm/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/rhel_7_tree/opt/rh/devtoolset-7/root/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/rocm/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/rocm-invalid/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/rocm-spack/hip-4.0.0-5f63slrursbrvfe2txrrjkynbsywsob5/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/rocm-spack/llvm-amdgpu-4.0.0-ieagcs7inf7runpyfvepqkurasoglq4z/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/Windows/ARM/8.1/usr/bin
./vendor/rust/src/llvm-project/clang/test/Driver/Inputs/Windows/usr/bin
./vendor/rust/src/llvm-project/clang/test/Tooling/Inputs/mock-libcxx/bin
./vendor/rust/src/llvm-project/clang/tools/scan-build/bin
./vendor/rust/src/llvm-project/clang/tools/scan-build-py/bin
./vendor/rust/src/llvm-project/clang/tools/scan-view/bin
./vendor/rust/src/llvm-project/clang-tools-extra/test/clang-tidy/infrastructure/Inputs/mock-libcxx/bin
./vendor/rust/src/llvm-project/flang/test/Driver/Inputs/basic_cross_linux_tree/usr/bin
./vendor/rust/src/llvm-project/flang/test/Driver/Inputs/basic_cross_linux_tree/usr/i386-unknown-linux-gnu/bin
./vendor/rust/src/llvm-project/flang/test/Driver/Inputs/basic_cross_linux_tree/usr/x86_64-unknown-linux-gnu/bin
./vendor/rust/src/llvm-project/flang/test/Driver/Inputs/rocm/bin
./vendor/rust/src/llvm-project/lldb/test/API/functionalities/object-file/bin
./vendor/rust/src/tools/cargo/benches/benchsuite/src/bin
./vendor/rust/src/tools/cargo/src/bin
./vendor/rust/src/tools/compiletest/src/bin
./vendor/rust/src/tools/llvm-bitcode-linker/src/bin
./vendor/rust/src/tools/miri/src/bin
./vendor/rust/src/tools/rust-analyzer/crates/rust-analyzer/src/bin
./vendor/rust/src/tools/rust-installer/test/image1/bin
./vendor/rust/src/tools/rust-installer/test/image2/bin
./vendor/rust/src/tools/rust-installer/test/image3/bin
./vendor/rust/src/tools/rustc-perf/collector/compile-benchmarks/cargo-0.87.1/src/bin
./vendor/rust/src/tools/rustc-perf/collector/src/bin
./vendor/rust/src/tools/rustc-perf/database/src/bin
./vendor/rust/src/tools/rustc-perf/site/src/bin
./vendor/rust/src/tools/rustfmt/src/bin
./vendor/rust-phf/phf_generator/src/bin
./vendor/rust-sbert/src/bin
./vendor/rustls/examples/src/bin
./vendor/sharded-slab/bin
./vendor/sprs/suitesparse_bindings/suitesparse-src/SuiteSparse/bin
./vendor/tinyvec/fuzz/src/bin
./vendor/tokio/tests-integration/src/bin
./vendor/tracing/bin
./vendor/utils/blobby/src/bin
./vendor/vaporetto/examples/wasm/src/bin
./vendor/vcpkg-rs/test-data/no-status/installed/x64-windows/bin
./vendor/vcpkg-rs/test-data/normalized/installed/x64-windows/bin
./vendor/vcpkg-rs/test-data/normalized/installed/x64-windows/debug/bin
./vendor/vcpkg-rs/test-data/normalized/installed/x86-windows/bin
./vendor/vibrato/examples/wasm/src/bin
./vendor/wasm-bindgen/crates/cli/src/bin
./vendor/wit-bindgen/src/bin


## Vendor/Input Crates Matrix

Below are some of the key input and vendor crates used in this project. These provide foundational functionality, NLP, search, and integration capabilities.

| Vendor Crate                | Emoji  | Function / OODA Phase         | Description                                                      |
|-----------------------------|--------|-------------------------------|------------------------------------------------------------------|
| path-clean                  | 🧹     | Observer (Utility)            | Path normalization and cleaning utilities                        |
| agave-solana-validator      | 🛡️     | Act (Blockchain)              | Solana validator integration and testing tools                   |
| time                        | ⏰     | Observer (Time)               | Time and date utilities                                          |
| steel                       | 🏗️     | Orient (Math/Logic)           | Mathematical and logic programming utilities                     |
| rhai                        | 📜     | Orient (Scripting)            | Embedded scripting engine for automation                         |
| json-ld                     | 🧾     | Orient (Semantic)             | JSON-LD and semantic web processing                              |
| eliza-rs                    | 🤖     | Orient (NLP)                  | ELIZA chatbot and NLP tools                                      |
| rust-sentence-transformers  | 🧬     | Orient (Embedding)            | Sentence transformer embeddings                                  |
| keyword-extraction-rs       | 🏷️     | Observer (NLP)                | Keyword extraction utilities                                     |
| vibrato                     | 🎵     | Orient (NLP)                  | Text segmentation and tokenization                               |
| tantivy                     | 🔎     | Observer (Search)             | Full-text search engine                                          |
| quickwit                    | ⚡     | Observer (Search)             | Fast search/indexing engine                                      |
| sophia_rs                   | 🕸️     | Orient (Semantic)             | RDF and semantic web processing                                  |
| lean4                       | 📐     | Orient (Proof/Logic)          | Lean 4 theorem prover integration                                |
| ...                         | ...    | ...                           | ...                                                              |

> This is a representative sample. The `vendor/` directory contains many more crates for NLP, search, semantic web, blockchain, and utility functions.

---

## 🔍 Codebase Analysis Tools

### Codebase Analyzer CLI

The `codebase_analyzer_cli` provides powerful analysis capabilities for your indexed codebase using the existing Tantivy search infrastructure.

**Usage:**
```bash
# Show top words by frequency
cargo run --bin codebase_analyzer_cli word-freq 50

# Show top emojis by frequency  
cargo run --bin codebase_analyzer_cli emoji-freq 20

# Show file types by count
cargo run --bin codebase_analyzer_cli file-types 20

# Search codebase content
cargo run --bin codebase_analyzer_cli search "function" 10

# Show overall statistics
cargo run --bin codebase_analyzer_cli stats
```

**Features:**
- **Word Frequency Analysis:** Find most common terms in your codebase
- **Emoji Statistics:** Analyze emoji usage patterns across code
- **File Type Analysis:** Understand your project's file composition
- **Full-Text Search:** Search through indexed code content
- **Comprehensive Stats:** Get overview of indexed data

**Example Output:**
```
Top 20 words in codebase:
  1. function              - 1,234
  2. struct                - 987
  3. impl                  - 876
  4. pub                   - 654
  5. let                   - 543
  ...

Top 10 emojis in codebase:
  1. 🔧  - 45
  2. 📝  - 32
  3. 🚀  - 28
  4. 🐛  - 25
  5. ✨  - 22
  ...
```

This tool leverages your existing `codebase_index/` Tantivy index, providing fast analysis without reprocessing files.


