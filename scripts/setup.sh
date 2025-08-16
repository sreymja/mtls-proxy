#!/bin/bash

# mTLS Proxy Server Setup Script
# This script helps set up and run the mTLS proxy server

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
    print_status "Building the mTLS proxy server..."
    cargo build --release
    print_success "Build completed successfully"
}

# Create necessary directories
create_directories() {
    print_status "Creating necessary directories..."
    mkdir -p certs logs config
    print_success "Directories created"
}

# Check certificates
check_certificates() {
    print_status "Checking certificates..."
    
    if [ ! -f "certs/client.crt" ]; then
        print_warning "Client certificate not found at certs/client.crt"
        echo "Please add your client certificate to certs/client.crt"
    else
        print_success "Client certificate found"
    fi
    
    if [ ! -f "certs/client.key" ]; then
        print_warning "Client private key not found at certs/client.key"
        echo "Please add your client private key to certs/client.key"
    else
        print_success "Client private key found"
    fi
    
    if [ ! -f "certs/ca.crt" ]; then
        print_warning "CA certificate not found at certs/ca.crt"
        echo "This is optional, but recommended for production use"
    else
        print_success "CA certificate found"
    fi
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
# port = 8080

[tls]
# client_cert_path = "certs/client.crt"
# client_key_path = "certs/client.key"
# ca_cert_path = "certs/ca.crt"
# verify_hostname = true

[logging]
# log_dir = "logs"
# retention_days = 7  # Reduce for development
# compression_enabled = true

[target]
# base_url = "https://your-gpt-instance:443"
# timeout_secs = 60
EOF
        print_success "Local configuration file created at config/local.toml"
        print_warning "Please edit config/local.toml with your specific settings"
    else
        print_status "Local configuration file already exists"
    fi
}

# Set up log rotation
setup_log_rotation() {
    print_status "Setting up log rotation..."
    
    # Create logrotate configuration
    sudo tee /etc/logrotate.d/mtls-proxy > /dev/null << EOF
$(pwd)/logs/*.log {
    daily
    missingok
    rotate 7
    compress
    delaycompress
    notifempty
    create 644 $(whoami) $(whoami)
    postrotate
        # Restart the proxy if it's running
        if pgrep -f mtls-proxy > /dev/null; then
            echo "Log rotated, consider restarting mtls-proxy"
        fi
    endscript
}
EOF
    
    print_success "Log rotation configured"
}

# Create systemd service
create_systemd_service() {
    print_status "Creating systemd service..."
    
    SERVICE_FILE="/etc/systemd/system/mtls-proxy.service"
    
    sudo tee $SERVICE_FILE > /dev/null << EOF
[Unit]
Description=mTLS Proxy Server
After=network.target

[Service]
Type=simple
User=$(whoami)
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/target/release/mtls-proxy
Restart=always
RestartSec=5
Environment=RUST_LOG=info

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$(pwd)/logs $(pwd)/certs

[Install]
WantedBy=multi-user.target
EOF
    
    sudo systemctl daemon-reload
    print_success "Systemd service created at $SERVICE_FILE"
    print_warning "To enable the service, run: sudo systemctl enable mtls-proxy"
    print_warning "To start the service, run: sudo systemctl start mtls-proxy"
}

# Show usage instructions
show_usage() {
    echo
    print_status "Setup completed! Here's how to use the mTLS proxy:"
    echo
    echo "1. Edit your configuration:"
    echo "   nano config/local.toml"
    echo
    echo "2. Add your certificates to the certs/ directory:"
    echo "   - certs/client.crt (your client certificate)"
    echo "   - certs/client.key (your client private key)"
    echo "   - certs/ca.crt (CA certificate, optional)"
    echo
    echo "3. Run the proxy server:"
    echo "   ./target/release/mtls-proxy"
    echo
    echo "4. Test the proxy:"
    echo "   python3 examples/test_proxy.py"
    echo
    echo "5. Monitor logs:"
    echo "   tail -f logs/proxy.log"
    echo "   sqlite3 logs/proxy_logs.db 'SELECT * FROM requests ORDER BY timestamp DESC LIMIT 10;'"
    echo
    echo "6. For production deployment:"
    echo "   sudo systemctl enable mtls-proxy"
    echo "   sudo systemctl start mtls-proxy"
    echo
}

# Main setup function
main() {
    echo "=== mTLS Proxy Server Setup ==="
    echo
    
    check_rust
    build_project
    create_directories
    check_certificates
    create_local_config
    
    # Ask if user wants systemd service
    read -p "Do you want to create a systemd service for production deployment? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        setup_log_rotation
        create_systemd_service
    fi
    
    show_usage
}

# Run main function
main "$@"
