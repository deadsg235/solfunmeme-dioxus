# AI Agent Directives for `s3_plugin`

This document outlines specific guidelines for AI agents (like Gemini) contributing to the `s3_plugin` submodule.

## Core Principles

When working within `s3_plugin`, AI agents should prioritize:

1.  **Security:** Ensure secure access and handling of data stored in S3 buckets, adhering to AWS best practices.
2.  **Reliability:** Design for high availability and fault tolerance in S3 operations.
3.  **Efficiency:** Optimize data transfer and storage costs when interacting with S3.

## Operational Guidelines

*   **Authentication:** Implement secure authentication mechanisms for AWS S3, avoiding hardcoded credentials.
*   **Error Handling:** Provide robust error handling for all S3 API calls, including retries and backoff strategies.
*   **Concurrency:** Consider potential concurrency issues when multiple agents or processes interact with S3.
*   **Configuration:** Allow for flexible configuration of S3 client settings (e.g., region, endpoint).
*   **Extensibility:** Design the module to be easily extensible for new S3 features or storage classes.
