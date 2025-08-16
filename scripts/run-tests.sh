#!/bin/bash

# Script to run all CI tests locally
# Usage: ./scripts/run-tests.sh [test-type]
# Test types: all, unit, integration, performance, security, ci

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to run tests
run_tests() {
    local test_type=$1
    
    case $test_type in
        "unit")
            print_status "Running unit tests..."
            cargo test --lib
            ;;
        "integration")
            print_status "Running integration tests..."
            cargo test --test integration_test
            ;;
        "performance")
            print_status "Running performance tests..."
            cargo test --test performance_test
            ;;
        "security")
            print_status "Running security tests..."
            cargo test --test security_test
            ;;
        "all")
            print_status "Running all tests..."
            cargo test --lib
            cargo test --test integration_test
            cargo test --test performance_test
            cargo test --test security_test
            cargo test --all-features
            cargo test --all-features -- --nocapture
            ;;
        "ci")
            print_status "Running full CI pipeline..."
            
            print_status "Checking code formatting..."
            cargo fmt --all -- --check
            
            print_status "Running clippy linting..."
            cargo clippy --all-targets --all-features -- -D warnings
            
            print_status "Running all tests..."
            cargo test --lib
            cargo test --test integration_test
            cargo test --test performance_test
            cargo test --test security_test
            cargo test --all-features
            cargo test --all-features -- --nocapture
            
            print_status "Running cargo audit..."
            cargo audit
            
            print_status "Running cargo deny..."
            if command -v cargo-deny &> /dev/null; then
                cargo deny check
            else
                print_warning "cargo-deny not installed. Skipping dependency checks."
            fi
            ;;
        *)
            print_error "Unknown test type: $test_type"
            echo "Usage: $0 [unit|integration|performance|security|all|ci]"
            exit 1
            ;;
    esac
}

# Main script
main() {
    echo "🚀 Running tests..."
    echo "================================"
    
    # Check if we're in the right directory
    if [ ! -f "Cargo.toml" ]; then
        print_error "Cargo.toml not found. Please run this from the project root."
        exit 1
    fi
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo not found. Please install Rust first."
        exit 1
    fi
    
    # Check for certificates
    if [ ! -f "certs/client.crt" ] || [ ! -f "certs/client.key" ] || [ ! -f "certs/ca.crt" ]; then
        print_warning "Certificate files not found. Generating test certificates..."
        if [ -f "scripts/generate_certs.sh" ]; then
            chmod +x scripts/generate_certs.sh
            ./scripts/generate_certs.sh
        else
            print_error "Certificate generation script not found."
            exit 1
        fi
    fi
    
    # Run tests based on argument
    local test_type=${1:-"all"}
    run_tests "$test_type"
    
    print_success "Tests completed successfully! 🎉"
    echo "================================"
}

# Run main function with all arguments
main "$@"
