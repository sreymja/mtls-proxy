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
