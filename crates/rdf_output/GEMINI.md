# AI Agent Directives for `rdf_output`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `rdf_output` submodule.

## Core Principles

When working within `rdf_output`, AI agents should prioritize:

1.  **Semantic Fidelity:** Ensure that the conversion of data into RDF accurately preserves its semantic meaning and relationships.
2.  **RDF Best Practices:** Adhere to established RDF and Linked Data best practices, including proper URI usage, vocabulary selection, and graph construction.
3.  **Modularity:** Maintain a clear separation of concerns, with functions focused on specific RDF generation tasks.

## Operational Guidelines

*   **URI Design:** Use clear, dereferenceable, and consistent URIs for resources and properties.
*   **Vocabulary Usage:** Prefer existing, well-established RDF vocabularies (e.g., Dublin Core, FOAF, Schema.org) where appropriate. Define new terms only when necessary and document them clearly.
*   **Graph Construction:** Ensure that generated RDF graphs are well-formed, consistent, and avoid redundant triples.
*   **Error Handling:** Implement robust error handling for data conversion and RDF serialization processes.
*   **Extensibility:** Design the module to be easily extensible for new data sources or RDF serialization formats.
