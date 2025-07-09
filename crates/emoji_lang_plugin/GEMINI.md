# AI Agent Directives for `emoji_lang_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `emoji_lang_plugin` submodule.

## Core Principles

When working within `emoji_lang_plugin`, AI agents should prioritize:

1.  **Semantic Mapping:** Ensure accurate and consistent mapping between emoji sequences and their corresponding semantic meanings or workflow actions.
2.  **Expressiveness:** Strive to make the emoji language expressive enough to represent complex workflows and concepts.
3.  **Interoperability:** Design the emoji language and its parsing to be easily integrated with other systems and tools.

## Operational Guidelines

*   **Emoji-to-Concept Mapping:** Maintain a clear and well-documented mapping between emojis and the concepts or actions they represent.
*   **Parsing Robustness:** Implement robust parsing mechanisms that can handle various emoji sequences, including multi-emoji combinations.
*   **URL Encoding/Decoding:** Ensure reliable encoding and decoding of emoji strings for URL usage, preserving their integrity.
*   **Ontology Integration:** Leverage the `solfunmem.jsonld` ontology to provide a rich semantic foundation for the emoji language.
*   **Extensibility:** Design the module to be easily extensible for new emojis, concepts, or parsing rules.
