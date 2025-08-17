#!/bin/bash

# mTLS Proxy Development Runner
# This script starts both the proxy server and mock server in development mode

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Function to cleanup background processes on exit
cleanup() {
    print_status "Shutting down servers..."
    if [ ! -z "$PROXY_PID" ]; then
        kill $PROXY_PID 2>/dev/null || true
    fi
    if [ ! -z "$MOCK_PID" ]; then
        kill $MOCK_PID 2>/dev/null || true
    fi
    print_success "Servers stopped"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Check if Rust is installed
check_rust() {
    print_status "Checking Rust installation..."
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    print_success "Rust is installed"
}

# Check if certificates exist
check_certificates() {
    print_status "Checking certificates..."
    
    local certs_missing=false
    
    if [ ! -f "certs/client.crt" ]; then
        print_warning "Client certificate not found at certs/client.crt"
        certs_missing=true
    fi
    
    if [ ! -f "certs/client.key" ]; then
        print_warning "Client private key not found at certs/client.key"
        certs_missing=true
    fi
    
    if [ ! -f "certs/server.crt" ]; then
        print_warning "Server certificate not found at certs/server.crt"
        certs_missing=true
    fi
    
    if [ ! -f "certs/server.key" ]; then
        print_warning "Server private key not found at certs/server.key"
        certs_missing=true
    fi
    
    if [ "$certs_missing" = true ]; then
        print_warning "Some certificates are missing. You may need to generate them first:"
        echo "  ./scripts/generate_certs.sh"
        echo "  or"
        echo "  cd mock-server && ./generate_certs.sh"
    else
        print_success "All certificates found"
    fi
}

# Build both projects
build_projects() {
    print_status "Building projects..."
    
    # Build main proxy server
    print_status "Building mTLS proxy server..."
    cargo build --release
    print_success "mTLS proxy server built"
    
    # Build mock server
    print_status "Building mock server..."
    cd mock-server
    cargo build --release
    cd ..
    print_success "Mock server built"
}

# Start mock server
start_mock_server() {
    print_status "Starting mock server..."
    cd mock-server
    cargo run --release &
    MOCK_PID=$!
    cd ..
    
    # Wait a moment for the server to start
    sleep 2
    
    if kill -0 $MOCK_PID 2>/dev/null; then
        print_success "Mock server started (PID: $MOCK_PID)"
        print_status "Mock server running on https://127.0.0.1:8444"
    else
        print_error "Failed to start mock server"
        exit 1
    fi
}

# Start proxy server
start_proxy_server() {
    print_status "Starting mTLS proxy server..."
    RUST_ENV=development cargo run --release &
    PROXY_PID=$!
    
    # Wait a moment for the server to start
    sleep 2
    
    if kill -0 $PROXY_PID 2>/dev/null; then
        print_success "mTLS proxy server started (PID: $PROXY_PID)"
        print_status "Proxy server running on https://127.0.0.1:8444"
    else
        print_error "Failed to start proxy server"
        exit 1
    fi
}

# Main execution
main() {
    print_status "Starting mTLS Proxy Development Environment"
    echo ""
    
    check_rust
    check_certificates
    build_projects
    
    echo ""
    print_status "Starting servers..."
    
    start_mock_server
    start_proxy_server
    
    echo ""
    print_success "Development environment is ready!"
    echo ""
    print_status "Available endpoints:"
    echo "  - Mock server: https://127.0.0.1:8444"
    echo "  - Proxy server: http://127.0.0.1:8440 (proxies to mock server)"
    echo ""
    print_status "Test requests can be made using the test_requests.http file"
    echo ""
    print_status "Press Ctrl+C to stop all servers"
    echo ""
    
    # Wait for user to stop
    wait
}

# Run main function
main
