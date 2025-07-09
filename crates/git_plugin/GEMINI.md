# AI Agent Directives for `git_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `git_plugin` submodule.

## Core Principles

When working within `git_plugin`, AI agents should prioritize:

1.  **Safety and Integrity:** Ensure that all Git operations are performed safely, preventing data loss or repository corruption.
2.  **Efficiency:** Optimize Git operations for speed and resource usage, especially for large repositories.
3.  **Modularity:** Maintain a clear separation of concerns, with functions focused on specific Git tasks.

## Operational Guidelines

*   **Authentication:** Handle Git authentication securely, avoiding hardcoded credentials.
*   **Error Handling:** Implement robust error handling for all Git commands, providing informative messages.
*   **Concurrency:** Consider potential concurrency issues when multiple agents or processes interact with the same repository.
*   **Configuration:** Allow for flexible configuration of Git settings (e.g., user, email, remote URLs).
*   **Extensibility:** Design the module to be easily extensible for new Git commands or workflows.
