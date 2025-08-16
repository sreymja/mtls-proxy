# mTLS Proxy + Mock Server Integration Guide

This guide explains how to set up and test the complete mTLS proxy system with the mock GPT-4o-mini API server for local development and testing.

## Architecture Overview

```
┌─────────────────┐    HTTP     ┌─────────────────┐    mTLS     ┌─────────────────┐
│   Client App    │ ──────────► │   mTLS Proxy    │ ──────────► │  Mock GPT-4o    │
│                 │             │   (Port 8080)   │             │  (Port 8443)    │
└─────────────────┘             └─────────────────┘             └─────────────────┘
```

## Prerequisites

- Rust 1.70+ installed
- OpenSSL for certificate generation
- Python 3.7+ for testing scripts

## Step-by-Step Setup

### 1. Clone and Setup Both Projects

```bash
# Clone the repository (if not already done)
git clone <repository-url>
cd ai-experiments/mtls-proxy

# Setup the mTLS proxy
./scripts/setup.sh

# Setup the mock server
cd mock-server
./scripts/setup.sh
```

### 2. Generate Shared Certificates

Both the proxy and mock server need to use the same CA certificate for mTLS authentication.

```bash
# Generate certificates in the mock-server directory
cd mock-server
./generate_certs.sh

# Copy certificates to the proxy directory
cp certs/ca.crt ../certs/
cp certs/client.crt ../certs/
cp certs/client.key ../certs/
```

### 3. Configure the Proxy

Update the proxy configuration to point to the mock server:

```bash
# Edit the proxy configuration
nano ../config/local.toml
```

Add or modify:
```toml
[target]
base_url = "https://localhost:8443"
timeout_secs = 60

[tls]
client_cert_path = "certs/client.crt"
client_key_path = "certs/client.key"
ca_cert_path = "certs/ca.crt"
verify_hostname = false  # For local testing
```

### 4. Configure the Mock Server

Update the mock server configuration:

```bash
# Edit the mock server configuration
nano config/local.toml
```

Add or modify:
```toml
[server]
host = "127.0.0.1"
port = 8443

[tls]
cert_path = "certs/server.crt"
key_path = "certs/server.key"
ca_cert_path = "certs/ca.crt"
require_client_cert = true
```

## Running the System

### 1. Start the Mock Server

```bash
cd mock-server
./target/release/mock-gpt-server
```

You should see:
```
[INFO] Starting Mock GPT-4o-mini API Server
[INFO] Configuration loaded successfully
[INFO] Starting mock GPT server on 127.0.0.1:8443
```

### 2. Start the mTLS Proxy

In a new terminal:
```bash
cd mtls-proxy
./target/release/mtls-proxy
```

You should see:
```
[INFO] Starting mTLS Proxy Server
[INFO] Configuration loaded successfully
[INFO] Starting proxy server on 127.0.0.1:8080
```

### 3. Test the Integration

#### Test Direct Connection to Mock Server
```bash
# Test health endpoint
curl -k https://localhost:8443/health

# Test models endpoint
curl -k https://localhost:8443/v1/models

# Test chat completion
curl -k -X POST https://localhost:8443/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Hello!"}]}'
```

#### Test Through the Proxy
```bash
# Test health endpoint through proxy
curl http://localhost:8080/health

# Test models endpoint through proxy
curl http://localhost:8080/v1/models

# Test chat completion through proxy
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Hello through proxy!"}]}'
```

## Testing Scenarios

### 1. Basic Functionality Test

Use the provided test scripts:

```bash
# Test the mock server directly
cd mock-server
python3 examples/test_mock_server.py

# Test the proxy
cd ../mtls-proxy
python3 examples/test_proxy.py
```

### 2. Performance Testing

#### Fast Response Testing
```bash
# Start mock server with fast responses
cd mock-server
MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS=10 ./target/release/mock-gpt-server

# In another terminal, test through proxy
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Test"}]}'
```

#### Slow Response Testing
```bash
# Start mock server with slow responses
cd mock-server
MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS=5000 ./target/release/mock-gpt-server

# Test timeout handling
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Test"}]}'
```

#### Error Testing
```bash
# Start mock server with error scenarios
cd mock-server
MOCK_GPT_RESPONSES_ERROR_RATE_PERCENT=10 ./target/release/mock-gpt-server

# Test error handling
for i in {1..10}; do
  curl -X POST http://localhost:8080/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Test"}]}'
  echo "Request $i completed"
done
```

### 3. Load Testing

#### Concurrent Request Testing
```bash
# Test with multiple concurrent requests
for i in {1..10}; do
  curl -X POST http://localhost:8080/v1/chat/completions \
    -H "Content-Type: application/json" \
    -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Test"}]}' &
done
wait
```

#### Streaming Response Testing
```bash
# Test streaming responses through proxy
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Write a story"}], "stream": true}'
```

## Monitoring and Debugging

### 1. Check Logs

#### Proxy Logs
```bash
# Check proxy logs
tail -f logs/proxy.log

# Check proxy database
sqlite3 logs/proxy_logs.db "SELECT * FROM requests ORDER BY timestamp DESC LIMIT 10;"
```

#### Mock Server Logs
```bash
# Check mock server logs
tail -f logs/mock-server.log
```

### 2. Monitor Resource Usage

```bash
# Monitor memory usage
ps aux | grep -E "(mtls-proxy|mock-gpt-server)"

# Monitor network connections
netstat -an | grep -E "(8080|8443)"
```

### 3. Health Checks

```bash
# Check proxy health
curl http://localhost:8080/health

# Check mock server health
curl -k https://localhost:8443/health
```

## Troubleshooting

### Common Issues

#### 1. Certificate Errors
```bash
# Verify certificate permissions
ls -la certs/
chmod 600 certs/*.key

# Verify certificate validity
openssl x509 -in certs/ca.crt -text -noout
```

#### 2. Connection Refused
```bash
# Check if services are running
ps aux | grep -E "(mtls-proxy|mock-gpt-server)"

# Check port availability
netstat -an | grep -E "(8080|8443)"
```

#### 3. mTLS Authentication Failures
```bash
# Verify certificate chain
openssl verify -CAfile certs/ca.crt certs/client.crt

# Check certificate subjects
openssl x509 -in certs/client.crt -subject -noout
openssl x509 -in certs/server.crt -subject -noout
```

#### 4. Timeout Issues
```bash
# Increase timeout in proxy config
echo 'timeout_secs = 120' >> config/local.toml

# Check mock server response time
time curl -k -X POST https://localhost:8443/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Test"}]}'
```

### Debug Mode

#### Enable Debug Logging
```bash
# Proxy debug mode
RUST_LOG=debug ./target/release/mock-gpt-server

# Mock server debug mode
RUST_LOG=debug ./target/release/mock-gpt-server
```

#### Verbose Testing
```bash
# Verbose curl output
curl -v -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Test"}]}'
```

## Production Considerations

### 1. Security Hardening

- Use proper CA-signed certificates
- Enable hostname verification
- Implement rate limiting
- Add authentication/authorization
- Use secure file permissions

### 2. Performance Optimization

- Tune connection limits
- Optimize buffer sizes
- Use connection pooling
- Implement caching
- Monitor resource usage

### 3. Monitoring and Alerting

- Set up log aggregation
- Implement health checks
- Add metrics collection
- Configure alerting
- Set up backup procedures

## Example Use Cases

### 1. Development Testing
```bash
# Quick development setup
./scripts/setup.sh
./target/release/mock-gpt-server &
./target/release/mtls-proxy &
```

### 2. CI/CD Integration
```bash
# Automated testing
cargo test
python3 examples/test_mock_server.py
python3 examples/test_proxy.py
```

### 3. Load Testing
```bash
# Performance testing
ab -n 1000 -c 10 -p test_data.json -T application/json http://localhost:8080/v1/chat/completions
```

## Next Steps

1. **Customize Responses**: Modify the mock server to generate more realistic responses
2. **Add More Endpoints**: Implement additional OpenAI API endpoints
3. **Enhance Logging**: Add more detailed request/response logging
4. **Performance Tuning**: Optimize for your specific use case
5. **Security Hardening**: Implement additional security measures
6. **Monitoring**: Set up comprehensive monitoring and alerting

This integration provides a complete testing environment for mTLS proxy development and validation.
