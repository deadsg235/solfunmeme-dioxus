# AI Agent Directives for `solfunmeme_rdf_utils`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `solfunmeme_rdf_utils` crate.

## Core Principles

When working within `solfunmeme_rdf_utils`, AI agents should prioritize:

1.  **Simplified RDF Interaction:** Provide a high-level, simplified API for common RDF operations, abstracting away the complexities of the underlying `sophia` library.
2.  **Consistency and Reusability:** Ensure consistent patterns for RDF graph manipulation and term creation, promoting reusability across the project.
3.  **Semantic Clarity:** Facilitate clear and intuitive representation of semantic data.

## Operational Guidelines

The `solfunmeme_rdf_utils` crate now serves as the primary interface for all RDF-related tasks within the `solfunmeme-dioxus` project. Direct usage of `sophia` types (e.g., `sophia_api::term::SimpleTerm`, `sophia_iri::Iri`) should be minimized in other crates and instead, the helper functions and structs provided here should be utilized.

### Key Components and Usage:

#### 1. `RdfGraph`

The `RdfGraph` struct is the central component for managing RDF data. It wraps `sophia_inmem::graph::FastGraph` and provides simplified methods for adding and querying triples.

**Example Usage:**

```rust
use solfunmeme_rdf_utils::rdf_graph::RdfGraph;
use solfunmeme_rdf_utils::namespace_manager::NamespaceManager;
use solfunmeme_rdf_utils::term_factory;

fn main() -> anyhow::Result<()> {
    let mut graph = RdfGraph::new();

    // Add namespaces
    graph.namespaces.add_namespace("ex", "http://example.org/ontology/")?;
    graph.namespaces.add_namespace("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")?;

    // Add a simple triple using string IRIs
    graph.add_triple(
        "http://example.org/subject1",
        "http://example.org/predicate1",
        "http://example.org/object1",
    )?;

    // Add a literal triple
    graph.add_literal_triple(
        "http://example.org/subject2",
        "http://example.org/predicate2",
        "Hello, World!",
        "http://www.w3.org/2001/XMLSchema#string",
    )?;

    // Serialize to Turtle string
    let turtle_string = graph.serialize_to_turtle_string()?;
    println!("{}", turtle_string);

    Ok(())
}
```

#### 2. `NamespaceManager`

The `NamespaceManager` helps in defining and retrieving IRIs and terms based on prefixes, reducing repetitive string concatenation and direct `IriRef` handling.

**Example Usage (within `RdfGraph` or standalone):**

```rust
use solfunmeme_rdf_utils::namespace_manager::NamespaceManager;

fn main() -> anyhow::Result<()> {
    let mut ns = NamespaceManager::new();
    ns.add_namespace("ex", "http://example.org/ontology/")?;
    ns.add_namespace("xsd", "http://www.w3.org/2001/XMLSchema#")?;

    let function_term = ns.get_term("ex", "Function")?;
    let string_type_iri = ns.get_iri("xsd", "string")?;

    println!("Function Term: {:?}", function_term);
    println!("String Type IRI: {:?}", string_type_iri);

    Ok(())
}
```

#### 3. `term_factory`

The `term_factory` module provides convenience functions for creating `sophia_api::term::SimpleTerm` instances from Rust native types, abstracting away the direct `sophia` constructors and lifetime management.

**Available Functions:**

*   `iri_term(iri_string: &str) -> anyhow::Result<SimpleTerm<'static>>`: Creates an IRI term.
*   `literal_term(value: &str) -> SimpleTerm<'static>`: Creates an untyped literal term.
*   `literal_term_typed(value: &str, datatype_iri: &str) -> anyhow::Result<SimpleTerm<'static>>`: Creates a typed literal term.
*   `bnode_term(id: &str) -> SimpleTerm<'static>`: Creates a blank node term.

**Example Usage:**

```rust
use solfunmeme_rdf_utils::term_factory;

fn main() -> anyhow::Result<()> {
    let my_iri = term_factory::iri_term("http://example.org/myResource")?;
    let my_literal = term_factory::literal_term("some value");
    let my_typed_literal = term_factory::literal_term_typed("123", "http://www.w3.org/2001/XMLSchema#integer")?;
    let my_bnode = term_factory::bnode_term("b1");

    println!("IRI: {:?}", my_iri);
    println!("Literal: {:?}", my_literal);
    println!("Typed Literal: {:?}", my_typed_literal);
    println!("BNode: {:?}", my_bnode);

    Ok(())
}
```

#### 4. `GraphBuilder`

The `GraphBuilder` provides a fluent interface for constructing `RdfGraph` instances, especially useful for complex graph creation scenarios.

**Example Usage:**

```rust
use solfunmeme_rdf_utils::rdf_graph::GraphBuilder;

fn main() -> anyhow::Result<()> {
    let graph = GraphBuilder::new()
        .with_namespace("ex", "http://example.org/data/")?
        .add_triple("http://example.org/data/subjectA", "http://example.org/data/predicateA", "http://example.org/data/objectA")?
        .add_literal_triple("http://example.org/data/subjectB", "http://example.org/data/predicateB", "42", "http://www.w3.org/2001/XMLSchema#integer")?
        .build();

    let turtle_string = graph.serialize_to_turtle_string()?;
    println!("{}", turtle_string);

    Ok(())
}
```

### Why this simplifies `sophia` usage:

The primary goal of `solfunmeme_rdf_utils` is to reduce the boilerplate and cognitive load associated with direct `sophia` API usage.

*   **Abstracted Term Creation:** Instead of manually constructing `SimpleTerm` with `Iri::new_unchecked` or `SimpleTerm::new_literal_dt`, `term_factory` provides simple functions that handle the underlying `sophia` types and error handling.
*   **Simplified Triple Addition:** `RdfGraph::add_triple` and `RdfGraph::add_literal_triple` now directly accept `&str` for IRIs, removing the need for manual term conversion before adding triples.
*   **Centralized Namespace Management:** `NamespaceManager` ensures consistent IRI generation and avoids hardcoding namespace URIs throughout the codebase.
*   **Fluent Graph Construction:** The `GraphBuilder` pattern allows for a more readable and chainable way to build RDF graphs.

By using these abstractions, other crates in the project can interact with RDF data at a higher level, focusing on the semantic content rather than the intricacies of the RDF library.

### Refactoring Impact:

The refactoring efforts involved updating several crates (e.g., `rdf_processing_lib`, `semweb_lib`, `solfunmeme_lean4`) to leverage this new simplified API. This has resulted in cleaner code, fewer direct `sophia` imports outside of `solfunmeme_rdf_utils`, and a more maintainable RDF layer for the project.
