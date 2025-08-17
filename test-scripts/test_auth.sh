#!/bin/bash

echo "Testing mTLS Proxy Authentication"
echo "================================="

# Start the proxy server in the background
echo "Starting proxy server..."
cargo run --bin mtls-proxy -- --port 8082 &
PROXY_PID=$!

# Wait for server to start
sleep 3

# Test metrics endpoint (no auth required)
echo ""
echo "Testing /metrics endpoint (no auth required)..."
curl -s http://localhost:8082/metrics | head -5

# Test UI endpoint without auth (should fail)
echo ""
echo "Testing /ui endpoint without auth (should fail)..."
curl -s -w "Status: %{http_code}\n" http://localhost:8082/ui

# Test UI endpoint with correct auth
echo ""
echo "Testing /ui endpoint with correct auth..."
curl -s -w "Status: %{http_code}\n" \
  -H "Authorization: Basic $(echo -n 'admin:admin123' | base64)" \
  http://localhost:8082/ui | head -10

# Test UI endpoint with wrong auth
echo ""
echo "Testing /ui endpoint with wrong auth (should fail)..."
curl -s -w "Status: %{http_code}\n" \
  -H "Authorization: Basic $(echo -n 'admin:wrongpassword' | base64)" \
  http://localhost:8082/ui

# Test health endpoint (no auth required)
echo ""
echo "Testing /health endpoint (no auth required)..."
curl -s http://localhost:8082/health

# Stop the proxy server
echo ""
echo "Stopping proxy server..."
kill $PROXY_PID
wait $PROXY_PID 2>/dev/null

echo "Authentication test completed!"
