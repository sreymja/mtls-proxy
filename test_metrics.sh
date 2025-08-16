#!/bin/bash

echo "Testing mTLS Proxy Metrics Endpoint"
echo "==================================="

# Start the proxy server in the background
echo "Starting proxy server..."
cargo run --bin mtls-proxy -- --port 8081 &
PROXY_PID=$!

# Wait for server to start
sleep 3

# Test metrics endpoint
echo "Testing /metrics endpoint..."
curl -s http://localhost:8081/metrics

echo ""
echo "Testing /health endpoint..."
curl -s http://localhost:8081/health

echo ""
echo "Testing /ui endpoint..."
curl -s http://localhost:8081/ui | head -20

# Stop the proxy server
echo ""
echo "Stopping proxy server..."
kill $PROXY_PID
wait $PROXY_PID 2>/dev/null

echo "Test completed!"
