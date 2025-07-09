# `solfunmeme_input_fs`

This crate provides functionalities for reading code files from the filesystem and converting their content into structured `CodeChunk`s.

## Purpose

It serves as the primary input layer for the code analysis pipeline, abstracting away the complexities of file system interaction and providing a standardized way to ingest source code for further processing.

## Core Functionalities

-   **Read Code Chunks**: Reads files from a specified path (or current directory) and generates a vector of `CodeChunk` instances, each representing a segment of the code.

## Usage

```rust
use solfunmeme_input_fs::read_code_chunks;

fn main() {
    // Read code chunks from the current directory (conceptual)
    let code_chunks = read_code_chunks(None, Some(10)).expect("Failed to read code chunks");
    println!("Read {} code chunks.", code_chunks.len());

    for chunk in code_chunks {
        println!("Path: {}, Lines: {}-{}", chunk.path, chunk.line_start, chunk.line_end);
        // println!("Content:\n{}", chunk.content);
    }
}
```
