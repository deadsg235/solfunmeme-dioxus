# AI Agent Directives for `zip_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `zip_plugin` submodule.

## Core Principles

When working within `zip_plugin`, AI agents should prioritize:

1.  **Data Integrity:** Ensure that data is compressed and decompressed without corruption or loss.
2.  **Efficiency:** Optimize compression and decompression operations for speed and resource usage.
3.  **Modularity:** Maintain a clear separation of concerns, with functions focused on specific Zip archive tasks.

## Operational Guidelines

*   **Error Handling:** Implement robust error handling for all Zip operations, providing informative messages.
*   **Security:** Be mindful of potential security vulnerabilities related to Zip archives (e.g., Zip bombs, path traversal).
*   **Compatibility:** Ensure compatibility with common Zip archive formats and standards.
*   **Extensibility:** Design the module to be easily extensible for new compression methods or archive features.
