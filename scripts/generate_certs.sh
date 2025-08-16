#!/bin/bash

# Generate test certificates for mTLS proxy development
# This script creates a self-signed CA and client/server certificates

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CERT_DIR="$SCRIPT_DIR/../certs"

echo "=== Generating Test Certificates ==="

# Create certs directory if it doesn't exist
mkdir -p "$CERT_DIR"

# Generate CA private key and certificate
echo "Generating CA private key and certificate..."
openssl genrsa -out "$CERT_DIR/ca.key" 2048
openssl req -new -x509 -days 365 -key "$CERT_DIR/ca.key" -out "$CERT_DIR/ca.crt" \
    -subj "/C=US/ST=CA/L=San Francisco/O=Test CA/CN=test-ca.local"

# Generate server private key and certificate signing request
echo "Generating server private key and CSR..."
openssl genrsa -out "$CERT_DIR/server.key" 2048
openssl req -new -key "$CERT_DIR/server.key" -out "$CERT_DIR/server.csr" \
    -subj "/C=US/ST=CA/L=San Francisco/O=Test Server/CN=localhost"

# Sign server certificate with CA
echo "Signing server certificate..."
openssl x509 -req -days 365 -in "$CERT_DIR/server.csr" -CA "$CERT_DIR/ca.crt" \
    -CAkey "$CERT_DIR/ca.key" -CAcreateserial -out "$CERT_DIR/server.crt"

# Generate client private key and certificate signing request
echo "Generating client private key and CSR..."
openssl genrsa -out "$CERT_DIR/client.key" 2048
openssl req -new -key "$CERT_DIR/client.key" -out "$CERT_DIR/client.csr" \
    -subj "/C=US/ST=CA/L=San Francisco/O=Test Client/CN=test-client.local"

# Sign client certificate with CA
echo "Signing client certificate..."
openssl x509 -req -days 365 -in "$CERT_DIR/client.csr" -CA "$CERT_DIR/ca.crt" \
    -CAkey "$CERT_DIR/ca.key" -CAcreateserial -out "$CERT_DIR/client.crt"

# Set proper permissions
chmod 600 "$CERT_DIR"/*.key
chmod 644 "$CERT_DIR"/*.crt

# Clean up CSR files
rm "$CERT_DIR"/*.csr

echo "=== Certificate Generation Complete ==="
echo "Generated certificates in: $CERT_DIR"
echo ""
echo "Files created:"
echo "  - ca.crt: CA certificate"
echo "  - ca.key: CA private key"
echo "  - server.crt: Server certificate"
echo "  - server.key: Server private key"
echo "  - client.crt: Client certificate"
echo "  - client.key: Client private key"
echo ""
echo "Note: These are test certificates for development only!"
echo "Do not use in production environments."
