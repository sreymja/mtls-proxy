#!/bin/bash

# Test script for mTLS proxy
# This script tests the basic functionality of the proxy server

set -e

echo "=== Testing mTLS Proxy ==="

# Check if certificates exist
if [ ! -f "certs/client.crt" ] || [ ! -f "certs/client.key" ]; then
    echo "Error: Test certificates not found. Run ./scripts/generate_certs.sh first."
    exit 1
fi

# Build the proxy
echo "Building proxy server..."
cargo build --release

# Start the proxy server in the background
echo "Starting proxy server..."
./target/release/mtls-proxy &
PROXY_PID=$!

# Wait for the server to start
sleep 2

# Test health endpoint
echo "Testing health endpoint..."
HEALTH_RESPONSE=$(curl -s http://localhost:8440/ui/health)
if [[ $HEALTH_RESPONSE == *"Health Status"* ]]; then
    echo "✅ Health check passed"
else
    echo "❌ Health check failed: $HEALTH_RESPONSE"
fi

# Test UI endpoint
echo "Testing UI endpoint..."
UI_RESPONSE=$(curl -s http://localhost:8440/ui)
if [[ $UI_RESPONSE == *"mTLS Proxy Dashboard"* ]]; then
    echo "✅ UI endpoint passed"
else
    echo "❌ UI endpoint failed"
fi

# Test logs endpoint
echo "Testing logs endpoint..."
LOGS_RESPONSE=$(curl -s http://localhost:8440/ui/logs)
if [[ $LOGS_RESPONSE == *"Request Logs"* ]]; then
    echo "✅ Logs endpoint passed"
else
    echo "❌ Logs endpoint failed"
fi

# Test API logs endpoint
echo "Testing API logs endpoint..."
API_LOGS_RESPONSE=$(curl -s http://localhost:8440/ui/api/logs)
if [[ $API_LOGS_RESPONSE == *"requests"* ]]; then
    echo "✅ API logs endpoint passed"
else
    echo "❌ API logs endpoint failed"
fi

# Test proxy endpoint (this will fail without a target server, but should return an error)
echo "Testing proxy endpoint..."
PROXY_RESPONSE=$(curl -s -w "%{http_code}" http://localhost:8440/test -o /dev/null)
if [ "$PROXY_RESPONSE" = "502" ] || [ "$PROXY_RESPONSE" = "504" ]; then
    echo "✅ Proxy endpoint returned expected error code: $PROXY_RESPONSE"
else
    echo "⚠️  Proxy endpoint returned unexpected code: $PROXY_RESPONSE"
fi

# Stop the proxy server
echo "Stopping proxy server..."
kill $PROXY_PID
wait $PROXY_PID 2>/dev/null || true

echo "=== Test completed ==="
