# AI Agent Directives for `jsonld_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `jsonld_plugin` submodule.

## Core Principles

When working within `jsonld_plugin`, AI agents should prioritize:

1.  **Semantic Accuracy:** Ensure that JSON-LD processing accurately reflects the intended semantic meaning of the data.
2.  **Interoperability:** Promote interoperability with other Linked Data systems and tools.
3.  **Modularity:** Maintain a clear separation of concerns, with functions focused on specific JSON-LD tasks.

## Operational Guidelines

*   **Context Handling:** Properly manage JSON-LD contexts to ensure correct interpretation of terms and IRIs.
*   **Error Handling:** Implement robust error handling for JSON-LD parsing, serialization, and compaction.
*   **Validation:** Consider adding validation mechanisms to ensure generated JSON-LD conforms to specified schemas or ontologies.
*   **Extensibility:** Design the module to be easily extensible for new JSON-LD features or transformations.
