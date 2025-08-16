#!/bin/bash

# Mock GPT-4o-mini API Server Setup Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

# Check if Rust is installed
check_rust() {
    print_status "Checking Rust installation..."
    if command -v rustc &> /dev/null; then
        RUST_VERSION=$(rustc --version | cut -d' ' -f2)
        print_success "Rust $RUST_VERSION is installed"
    else
        print_error "Rust is not installed. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
}

# Build the project
build_project() {
    print_status "Building the mock GPT server..."
    cargo build --release
    print_success "Build completed successfully"
}

# Create necessary directories
create_directories() {
    print_status "Creating necessary directories..."
    mkdir -p certs logs config
    print_success "Directories created"
}

# Generate test certificates
generate_certificates() {
    print_status "Generating test certificates..."
    
    # Create a simple certificate generation script
    cat > generate_certs.sh << 'EOF'
#!/bin/bash
set -e

# Generate CA certificate and key
openssl genrsa -out certs/ca.key 2048
openssl req -new -x509 -days 365 -key certs/ca.key -out certs/ca.crt -subj "/C=US/ST=CA/L=San Francisco/O=Test CA/CN=Test CA"

# Generate server certificate and key
openssl genrsa -out certs/server.key 2048
openssl req -new -key certs/server.key -out certs/server.csr -subj "/C=US/ST=CA/L=San Francisco/O=Test Server/CN=localhost"
openssl x509 -req -in certs/server.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/server.crt -days 365

# Generate client certificate and key
openssl genrsa -out certs/client.key 2048
openssl req -new -key certs/client.key -out certs/client.csr -subj "/C=US/ST=CA/L=San Francisco/O=Test Client/CN=test-client"
openssl x509 -req -in certs/client.csr -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial -out certs/client.crt -days 365

# Clean up CSR files
rm certs/*.csr certs/*.srl

echo "Test certificates generated successfully!"
EOF

    chmod +x generate_certs.sh
    ./generate_certs.sh
    print_success "Test certificates generated"
}

# Create local configuration
create_local_config() {
    if [ ! -f "config/local.toml" ]; then
        print_status "Creating local configuration file..."
        cat > config/local.toml << EOF
# Local configuration overrides
# This file is not committed to version control

[server]
# host = "0.0.0.0"  # Uncomment to bind to all interfaces
# port = 8443

[tls]
# cert_path = "certs/server.crt"
# key_path = "certs/server.key"
# ca_cert_path = "certs/ca.crt"
# require_client_cert = true

[responses]
# default_delay_ms = 100
# error_rate_percent = 0
# streaming_enabled = true

[models]
# available = ["gpt-4o-mini", "gpt-4o", "gpt-3.5-turbo"]
EOF
        print_success "Local configuration file created at config/local.toml"
        print_warning "Please edit config/local.toml with your specific settings"
    else
        print_status "Local configuration file already exists"
    fi
}

# Show usage instructions
show_usage() {
    echo
    print_status "Setup completed! Here's how to use the mock GPT server:"
    echo
    echo "1. Edit your configuration:"
    echo "   nano config/local.toml"
    echo
    echo "2. Run the mock server:"
    echo "   ./target/release/mock-gpt-server"
    echo
    echo "3. Test the server:"
    echo "   curl -k https://localhost:8443/health"
    echo "   curl -k https://localhost:8443/v1/models"
    echo
    echo "4. Test with mTLS proxy:"
    echo "   # Update proxy config to point to mock server"
    echo "   # Then test through the proxy"
    echo
    echo "5. Test scenarios:"
    echo "   # Fast responses: MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS=10 ./target/release/mock-gpt-server"
    echo "   # Error scenarios: MOCK_GPT_RESPONSES_ERROR_RATE_PERCENT=10 ./target/release/mock-gpt-server"
    echo "   # Slow responses: MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS=5000 ./target/release/mock-gpt-server"
    echo
}

# Main setup function
main() {
    echo "=== Mock GPT-4o-mini API Server Setup ==="
    echo
    
    check_rust
    build_project
    create_directories
    generate_certificates
    create_local_config
    
    show_usage
}

# Run main function
main "$@"
