# Mock GPT-4o-mini API Server

A lightweight, resource-efficient mock server that simulates the OpenAI GPT-4o-mini API for local testing. Built in Rust for minimal resource usage and maximum performance.

## Features

- **OpenAI API Compatibility**: Implements the same endpoints and response format as OpenAI's API
- **mTLS Support**: Requires client certificate authentication for security
- **Realistic Responses**: Generates plausible AI responses for testing
- **Configurable Behavior**: Customizable response times, error rates, and scenarios
- **Streaming Support**: Full support for streaming chat completions
- **Request Logging**: Comprehensive logging for debugging
- **Web UI**: Real-time dashboard for monitoring requests, responses, and server health
- **Resource Efficient**: Minimal CPU and memory footprint (~5-10MB)

## Quick Start

### Prerequisites
- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- OpenSSL (for certificate generation)

### Installation

1. **Setup the mock server:**
```bash
cd mock-server
./scripts/setup.sh
```

2. **Run the server:**
```bash
./target/release/mock-gpt-server
```

3. **Test the server:**
```bash
python3 examples/test_mock_server.py
```

4. **Test the UI:**
```bash
python3 examples/test_ui.py
```

## Configuration

### Default Configuration
The server loads configuration from multiple sources in order:
1. `config/default.toml` (required)
2. `config/test_scenarios.toml` (optional)
3. `config/local.toml` (optional, for local overrides)
4. Environment variables (prefixed with `MOCK_GPT_`)

### Example Configuration
```toml
[server]
host = "127.0.0.1"
port = 8443
max_connections = 1000

[tls]
cert_path = "certs/server.crt"
key_path = "certs/server.key"
ca_cert_path = "certs/ca.crt"
require_client_cert = true

[responses]
default_delay_ms = 100
error_rate_percent = 0
streaming_enabled = true
max_tokens = 1000
temperature = 0.7

[models]
available = ["gpt-4o-mini", "gpt-4o", "gpt-3.5-turbo"]
```

### Environment Variables
```bash
export MOCK_GPT_SERVER_HOST="0.0.0.0"
export MOCK_GPT_SERVER_PORT="8443"
export MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS="50"
export MOCK_GPT_RESPONSES_ERROR_RATE_PERCENT="5"
```

## Web UI

The mock server includes a web-based dashboard for monitoring and debugging:

```bash
# Access the dashboard
open https://localhost:8443/ui/dashboard

# View request logs
open https://localhost:8443/ui/requests

# Check health status
open https://localhost:8443/ui/health
```

The UI provides:
- **Real-time statistics**: Request counts, success rates, response times
- **Request details**: Complete view of all requests and responses with full headers and bodies
- **Health monitoring**: Server status and configuration information
- **Filtering and search**: Find specific requests by method, status code, etc.
- **Auto-refresh**: Automatic updates every 30 seconds
- **Interactive charts**: Visual representation of HTTP methods and status codes

### API Endpoints

### Health Check
```bash
curl -k https://localhost:8443/health
```
Response:
```json
{"status":"healthy","service":"mock-gpt-server"}
```

### List Models
```bash
curl -k https://localhost:8443/v1/models
```
Response:
```json
{
  "object": "list",
  "data": [
    {
      "id": "gpt-4o-mini",
      "object": "model",
      "created": 1698940800,
      "owned_by": "openai"
    }
  ]
}
```

### Chat Completions
```bash
curl -k -X POST https://localhost:8443/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [{"role": "user", "content": "Hello!"}],
    "max_tokens": 100
  }'
```

### Streaming Chat Completions
```bash
curl -k -X POST https://localhost:8443/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "gpt-4o-mini",
    "messages": [{"role": "user", "content": "Hello!"}],
    "stream": true
  }'
```

## Test Scenarios

The mock server supports different test scenarios for various testing needs:

### Fast Responses
```bash
MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS=10 ./target/release/mock-gpt-server
```

### Slow Responses
```bash
MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS=5000 ./target/release/mock-gpt-server
```

### Error Scenarios
```bash
MOCK_GPT_RESPONSES_ERROR_RATE_PERCENT=10 ./target/release/mock-gpt-server
```

### Timeout Scenarios
```bash
MOCK_GPT_RESPONSES_DEFAULT_DELAY_MS=30000 ./target/release/mock-gpt-server
```

## Integration with mTLS Proxy

### 1. Configure the Proxy
Update the proxy configuration to point to the mock server:

```toml
[target]
base_url = "https://localhost:8443"
timeout_secs = 60
```

### 2. Use the Same Certificates
The mock server and proxy should use the same CA certificate for mTLS authentication.

### 3. Test the Integration
```bash
# Start the mock server
./target/release/mock-gpt-server

# Start the proxy (in another terminal)
cd ../mtls-proxy
./target/release/mtls-proxy

# Test through the proxy
curl -X POST http://localhost:8080/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"model": "gpt-4o-mini", "messages": [{"role": "user", "content": "Hello!"}]}'
```

## Response Generation

### Realistic AI Responses
The mock server generates contextually appropriate responses based on user input:

- **Greetings**: Responds to "hello", "hi" with friendly greetings
- **Help requests**: Provides helpful responses to "help" requests
- **Code questions**: Acknowledges programming-related queries
- **General questions**: Provides informative responses with variations

### Response Customization
Responses can be customized through configuration:

```toml
[responses]
default_delay_ms = 100        # Response delay in milliseconds
error_rate_percent = 0        # Percentage of requests that return errors
streaming_enabled = true      # Enable/disable streaming responses
max_tokens = 1000            # Maximum tokens in responses
temperature = 0.7            # Response creativity (0.0-1.0)
```

## Error Handling

### Built-in Error Scenarios
- **Invalid model**: Returns 400 error for unsupported models
- **Invalid JSON**: Returns 400 error for malformed requests
- **Missing fields**: Returns 400 error for required field validation
- **Random errors**: Configurable percentage of random errors
- **Timeouts**: Configurable response delays

### Error Response Format
```json
{
  "error": {
    "message": "Model not found",
    "type": "invalid_request_error",
    "code": "model_not_found"
  }
}
```

## Performance Characteristics

### Resource Usage
- **Memory**: ~5-10MB baseline
- **CPU**: Minimal usage, configurable delays
- **Startup time**: <1 second
- **Concurrent connections**: 1000+ (configurable)

### Response Times
- **Fast mode**: 10-50ms
- **Normal mode**: 100-500ms
- **Slow mode**: 5-30 seconds
- **Timeout mode**: 30+ seconds

## Development

### Building from Source
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Development Mode
```bash
cargo run
```

### Certificate Generation
```bash
# Generate test certificates
./generate_certs.sh

# Or use the built-in generator
cargo run --bin generate-certs
```

## Docker Support

### Building the Image
```bash
docker build -t mock-gpt-server .
```

### Running with Docker
```bash
docker run -p 8443:8443 -v $(pwd)/certs:/app/certs mock-gpt-server
```

### Docker Compose
```yaml
version: '3.8'
services:
  mock-gpt-server:
    build: .
    ports:
      - "8443:8443"
    volumes:
      - ./certs:/app/certs
      - ./config:/app/config
    environment:
      - MOCK_GPT_SERVER_HOST=0.0.0.0
```

## Monitoring and Debugging

### Logging
The server uses structured logging with different levels:
```bash
# Set log level
RUST_LOG=debug ./target/release/mock-gpt-server

# Available levels: error, warn, info, debug, trace
```

### Health Monitoring
```bash
# Check server health
curl -k https://localhost:8443/health

# Monitor logs
tail -f logs/mock-server.log
```

### Performance Monitoring
```bash
# Monitor resource usage
ps aux | grep mock-gpt-server

# Monitor network connections
netstat -an | grep 8443
```

## Troubleshooting

### Common Issues

1. **Certificate Errors**
   - Ensure certificates are in the correct format (PEM)
   - Check file permissions (600 for private keys)
   - Verify certificate paths in configuration

2. **Connection Refused**
   - Check if server is running on the correct port
   - Verify firewall settings
   - Check for port conflicts

3. **mTLS Authentication Failures**
   - Ensure client certificate is signed by the CA
   - Verify certificate chain is complete
   - Check certificate expiration dates

4. **High Memory Usage**
   - Reduce `max_connections` in configuration
   - Monitor for memory leaks
   - Restart server if needed

### Debug Mode
```bash
# Enable debug logging
RUST_LOG=debug ./target/release/mock-gpt-server

# Enable trace logging for detailed debugging
RUST_LOG=trace ./target/release/mock-gpt-server
```

## Security Considerations

- **Test Certificates**: Generated certificates are for testing only
- **Network Security**: Use firewalls to restrict access
- **Certificate Management**: Regularly rotate certificates in production
- **Access Control**: Limit access to trusted clients only

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

[Add your license information here]
