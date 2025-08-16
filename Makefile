# Makefile for mTLS Proxy development
# Run 'make help' to see all available commands

.PHONY: help test test-all test-unit test-integration test-performance test-security format lint audit deny clean setup

# Default target
help:
	@echo "Available commands:"
	@echo "  make setup      - Install dependencies and generate certificates"
	@echo "  make test       - Run all tests (unit, integration, performance, security)"
	@echo "  make test-core  - Run core tests (unit tests only - most reliable)"
	@echo "  make test-unit  - Run unit tests only"
	@echo "  make test-integration - Run integration tests only"
	@echo "  make test-performance - Run performance tests only"
	@echo "  make test-security - Run security tests only"
	@echo "  make format     - Check code formatting"
	@echo "  make lint       - Run clippy linting"
	@echo "  make audit      - Run cargo audit"
	@echo "  make deny       - Run cargo deny"
	@echo "  make ci         - Run all CI checks (format, lint, test, audit, deny, docker)"
	@echo "  make docker-build - Test Docker build"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make help       - Show this help message"

# Setup development environment
setup:
	@echo "🔧 Setting up development environment..."
	@rustup update
	@cargo --version
	@if [ ! -f "certs/client.crt" ] || [ ! -f "certs/client.key" ] || [ ! -f "certs/ca.crt" ]; then \
		echo "📜 Generating test certificates..."; \
		chmod +x scripts/generate_certs.sh; \
		./scripts/generate_certs.sh; \
	fi
	@echo "✅ Setup complete!"

# Run all tests
test: test-unit test-integration test-performance test-security
	@echo "🎉 All tests passed!"

# Run core tests (unit tests only - most reliable)
test-core:
	@echo "🧪 Running core tests (unit tests only)..."
	@cargo test --lib
	@echo "🎉 Core tests passed!"

# Run unit tests
test-unit:
	@echo "🧪 Running unit tests..."
	@cargo test --lib

# Run integration tests
test-integration:
	@echo "🔗 Running integration tests..."
	@cargo test --test integration_test

# Run performance tests
test-performance:
	@echo "⚡ Running performance tests..."
	@cargo test --test performance_test

# Run security tests
test-security:
	@echo "🔒 Running security tests..."
	@cargo test --test security_test

# Check code formatting
format:
	@echo "📝 Checking code formatting..."
	@cargo fmt --all -- --check

# Run clippy linting
lint:
	@echo "🔍 Running clippy linting..."
	@cargo clippy --all-targets --all-features -- -D warnings

# Run cargo audit
audit:
	@echo "🔍 Running cargo audit..."
	@cargo audit

# Run cargo deny
deny:
	@echo "🚫 Running cargo deny..."
	@if command -v cargo-deny &> /dev/null; then \
		cargo deny check; \
	else \
		echo "⚠️  cargo-deny not installed. Install with: cargo install cargo-deny"; \
	fi

# Test Docker build
docker-build:
	@echo "🐳 Testing Docker build..."
	@if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then \
		docker build -t mtls-proxy-test . && \
		echo "✅ Docker build successful!" && \
		docker rmi mtls-proxy-test 2>/dev/null || true; \
	else \
		echo "⚠️  Docker not available. Skipping Docker build test."; \
	fi

# Run all CI checks
ci: format lint test audit deny docker-build
	@echo "🎉 All CI checks passed!"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@echo "✅ Clean complete!"

# Install cargo-deny if not present
install-deny:
	@if ! command -v cargo-deny &> /dev/null; then \
		echo "📦 Installing cargo-deny..."; \
		cargo install cargo-deny; \
	else \
		echo "✅ cargo-deny already installed"; \
	fi
