# AI Agent Directives for `workflow_manager`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `workflow_manager` submodule.

## Core Principles

When working within `workflow_manager`, AI agents should prioritize:

1.  **Flexibility:** Design the workflow management system to be highly adaptable to various types of processing tasks and execution environments.
2.  **Reliability:** Ensure that workflows execute consistently and handle failures gracefully.
3.  **Modularity:** Maintain a clear separation of concerns, with distinct components for workflow definition, registration, and execution.

## Operational Guidelines

*   **Workflow Definition:** Provide clear and concise mechanisms for defining workflows, including their steps, dependencies, and parameters.
*   **Error Handling:** Implement robust error handling within workflows, allowing for retries, fallbacks, or notifications.
*   **Monitoring and Logging:** Integrate logging and monitoring capabilities to track workflow progress and identify issues.
*   **Extensibility:** Design the module to be easily extensible for new workflow types, execution strategies, or integration points.
