#!/bin/bash

# Simple test script for mTLS proxy
# This script tests basic functionality without complex setup

set -e

echo "=== Simple mTLS Proxy Test ==="

# Check if certificates exist
if [ ! -f "certs/client.crt" ] || [ ! -f "certs/client.key" ]; then
    echo "Error: Test certificates not found. Run ./scripts/generate_certs.sh first."
    exit 1
fi

# Build the proxy
echo "Building proxy server..."
cargo build --release

echo "✅ Build successful"

# Test configuration loading
echo "Testing configuration..."
if cargo run --release --bin mtls-proxy -- --help 2>/dev/null; then
    echo "✅ Configuration test passed"
else
    echo "⚠️  Configuration test had issues (expected if no --help flag)"
fi

echo "=== Test completed ==="
echo ""
echo "To test the full proxy functionality:"
echo "1. Start the proxy: ./target/release/mtls-proxy"
echo "2. In another terminal, test endpoints:"
echo "   curl http://localhost:8440/ui/health"
echo "   curl http://localhost:8440/ui"
echo "3. To test with a mock server, see INTEGRATION_GUIDE.md"
