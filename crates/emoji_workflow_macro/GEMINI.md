# AI Agent Directives for `emoji_workflow_macro`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `emoji_workflow_macro` submodule.

## Core Principles

When working within `emoji_workflow_macro`, AI agents should prioritize:

1.  **Correctness:** Ensure that the macro correctly parses input, generates valid Rust code, and integrates seamlessly with the `workflow_manager` and `emoji_lang_plugin`.
2.  **Usability:** Design the macro to be intuitive and easy for developers to use, providing clear error messages when necessary.
3.  **Efficiency:** Optimize the macro's performance to minimize compilation times.

## Operational Guidelines

*   **Attribute Parsing:** Implement robust parsing of macro attributes, extracting emoji strings and other relevant metadata.
*   **AST Transformation:** Use `syn` and `quote` effectively to transform the annotated Rust code, inserting workflow registration logic.
*   **Error Reporting:** Provide informative and actionable error messages to guide developers in correct macro usage.
*   **Compatibility:** Ensure compatibility with different Rust versions and compiler features.
*   **Extensibility:** Design the macro to be easily extensible for new annotation types or code generation patterns.
