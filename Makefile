# Solfunmeme Dioxus Makefile

.PHONY: test coverage clean build doc-tests integration-tests unit-tests

# Default target
all: test coverage

# Run all tests
test: unit-tests integration-tests doc-tests
	@echo "✅ All tests completed"

# Run unit tests only
unit-tests:
	@echo "🧪 Running unit tests..."
	cargo test --lib

# Run integration tests
integration-tests:
	@echo "🔗 Running integration tests..."
	cargo test --test integration_tests

# Generate and run documentation tests
doc-tests:
	@echo "📚 Generating documentation tests..."
	cargo run --bin doc_test_generator
	@echo "📖 Running documentation tests..."
	cargo test --test doc_tests

# Run comprehensive test runner
test-runner:
	@echo "🚀 Running comprehensive test suite..."
	cargo run --bin test_runner

# Generate code coverage report
coverage:
	@echo "📊 Generating code coverage report..."
	cargo tarpaulin --out Html --output-dir coverage/
	@echo "📈 Coverage report generated in coverage/"

# Alternative coverage with llvm-cov
coverage-llvm:
	@echo "📊 Generating coverage with llvm-cov..."
	set RUSTFLAGS=-C instrument-coverage && cargo build
	cargo llvm-cov --html --output-dir coverage-llvm/

# Clean build artifacts and coverage reports
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	if exist coverage rmdir /s /q coverage
	if exist coverage-llvm rmdir /s /q coverage-llvm
	if exist tests\doc_tests.rs del tests\doc_tests.rs

# Build the project
build:
	@echo "🔨 Building project..."
	cargo build

# Build for release
build-release:
	@echo "🚀 Building release version..."
	cargo build --release

# Run clippy for linting
lint:
	@echo "🔍 Running clippy linter..."
	cargo clippy -- -D warnings

# Format code
fmt:
	@echo "✨ Formatting code..."
	cargo fmt

# Check formatting
fmt-check:
	@echo "🔍 Checking code formatting..."
	cargo fmt -- --check

# Run security audit
audit:
	@echo "🔒 Running security audit..."
	cargo audit

# Generate documentation
docs:
	@echo "📚 Generating documentation..."
	cargo doc --no-deps --open

# Run benchmarks (if any)
bench:
	@echo "⚡ Running benchmarks..."
	cargo bench

# Full CI pipeline
ci: fmt-check lint test coverage audit
	@echo "✅ CI pipeline completed successfully"

# Development setup
dev-setup:
	@echo "🛠️  Setting up development environment..."
	rustup component add clippy rustfmt
	cargo install cargo-tarpaulin cargo-audit cargo-llvm-cov
	@echo "✅ Development environment ready"

# Help target
help:
	@echo "Available targets:"
	@echo "  test           - Run all tests"
	@echo "  unit-tests     - Run unit tests only"
	@echo "  integration-tests - Run integration tests"
	@echo "  doc-tests      - Generate and run doc tests"
	@echo "  test-runner    - Run comprehensive test suite"
	@echo "  coverage       - Generate code coverage report"
	@echo "  coverage-llvm  - Generate coverage with llvm-cov"
	@echo "  build          - Build the project"
	@echo "  build-release  - Build release version"
	@echo "  lint           - Run clippy linter"
	@echo "  fmt            - Format code"
	@echo "  fmt-check      - Check code formatting"
	@echo "  audit          - Run security audit"
	@echo "  docs           - Generate documentation"
	@echo "  clean          - Clean build artifacts"
	@echo "  ci             - Run full CI pipeline"
	@echo "  dev-setup      - Set up development environment"
	@echo "  help           - Show this help message"
