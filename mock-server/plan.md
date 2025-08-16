# Mock GPT-4o-mini API Server Plan

## Overview

Create a mock GPT-4o-mini API server that simulates the OpenAI API responses for local testing of the mTLS proxy server. This will allow testing without requiring access to the actual private GPT-4o-mini instance.

## Requirements

### Functional Requirements
1. **OpenAI API Compatibility**: Implement the same endpoints and response format as OpenAI's API
2. **mTLS Support**: Require client certificate authentication
3. **Realistic Responses**: Generate plausible AI responses for testing
4. **Configurable Behavior**: Allow customization of response times, error rates, etc.
5. **Request Logging**: Log all incoming requests for debugging
6. **Streaming Support**: Support for streaming chat completions

### Non-Functional Requirements
1. **Low Resource Usage**: Minimal CPU and memory footprint
2. **Fast Startup**: Quick startup for development workflow
3. **Easy Configuration**: Simple configuration for different test scenarios
4. **Docker Support**: Containerized deployment for consistency

## Architecture

### Technology Stack
- **Language**: Rust (same as proxy server for consistency)
- **HTTP Server**: Hyper + Tower (same stack as proxy)
- **TLS**: Rustls with mTLS support
- **Response Generation**: Template-based with randomization
- **Configuration**: TOML files

### Component Structure
```
mock-server/
├── src/
│   ├── main.rs              # Server entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Configuration management
│   ├── tls.rs               # mTLS server setup
│   ├── handlers/            # API endpoint handlers
│   │   ├── mod.rs
│   │   ├── chat.rs          # Chat completions
│   │   ├── models.rs        # Models endpoint
│   │   └── health.rs        # Health checks
│   ├── responses/           # Response templates
│   │   ├── mod.rs
│   │   ├── chat.rs          # Chat completion responses
│   │   └── models.rs        # Models list responses
│   └── utils/               # Utility functions
│       ├── mod.rs
│       └── response_gen.rs  # Response generation logic
├── config/
│   ├── default.toml         # Default configuration
│   └── test_scenarios.toml  # Test scenario configurations
├── templates/               # Response templates
│   ├── chat_responses.json
│   └── model_responses.json
└── docker/                  # Docker configuration
    ├── Dockerfile
    └── docker-compose.yml
```

## Implementation Plan

### Phase 1: Basic Server Setup (Day 1)
1. **Project Structure**
   - Create Rust project with dependencies
   - Set up basic HTTP server with Hyper
   - Implement configuration management

2. **mTLS Server**
   - Configure TLS server with client certificate verification
   - Set up certificate generation for testing
   - Implement certificate validation

3. **Basic Endpoints**
   - Health check endpoint (`/health`)
   - Models endpoint (`/v1/models`)
   - Basic chat completions endpoint (`/v1/chat/completions`)

### Phase 2: Response Generation (Day 2)
1. **Response Templates**
   - Create JSON templates for different response types
   - Implement response randomization
   - Add configurable response delays

2. **Chat Completions**
   - Implement streaming and non-streaming responses
   - Add realistic AI response generation
   - Support different models (gpt-4o-mini, gpt-4o, etc.)

3. **Error Handling**
   - Implement realistic error responses
   - Add configurable error rates
   - Support different HTTP status codes

### Phase 3: Advanced Features (Day 3)
1. **Request Logging**
   - Log all incoming requests
   - Store request/response pairs
   - Add request validation

2. **Configuration Scenarios**
   - Fast responses for quick testing
   - Slow responses for timeout testing
   - Error scenarios for error handling
   - High load scenarios for performance testing

3. **Docker Support**
   - Create Dockerfile for containerized deployment
   - Add docker-compose for easy setup
   - Include certificate generation in container

### Phase 4: Testing and Integration (Day 4)
1. **Integration Testing**
   - Test with mTLS proxy server
   - Verify request/response flow
   - Test error scenarios

2. **Performance Testing**
   - Load testing with multiple concurrent requests
   - Memory usage monitoring
   - Response time validation

3. **Documentation**
   - API documentation
   - Setup instructions
   - Testing scenarios guide

## API Endpoints to Implement

### Core Endpoints
1. **GET /health** - Health check
2. **GET /v1/models** - List available models
3. **POST /v1/chat/completions** - Chat completions (main endpoint)
4. **POST /v1/completions** - Legacy completions (optional)

### Response Types
1. **Standard Response**: JSON response with completion
2. **Streaming Response**: Server-sent events format
3. **Error Response**: OpenAI-compatible error format

## Configuration Options

### Server Configuration
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

[models]
available = ["gpt-4o-mini", "gpt-4o", "gpt-3.5-turbo"]
```

### Test Scenarios
```toml
[scenarios.fast]
name = "Fast Responses"
default_delay_ms = 10
error_rate_percent = 0

[scenarios.slow]
name = "Slow Responses"
default_delay_ms = 5000
error_rate_percent = 0

[scenarios.errors]
name = "Error Scenarios"
default_delay_ms = 100
error_rate_percent = 10

[scenarios.timeout]
name = "Timeout Scenarios"
default_delay_ms = 30000
error_rate_percent = 0
```

## Response Templates

### Chat Completion Response
```json
{
  "id": "chatcmpl-{{random_id}}",
  "object": "chat.completion",
  "created": {{timestamp}},
  "model": "{{model}}",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "{{response_content}}"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": {{prompt_tokens}},
    "completion_tokens": {{completion_tokens}},
    "total_tokens": {{total_tokens}}
  }
}
```

### Models Response
```json
{
  "object": "list",
  "data": [
    {
      "id": "gpt-4o-mini",
      "object": "model",
      "created": 1698940800,
      "owned_by": "openai"
    },
    {
      "id": "gpt-4o",
      "object": "model",
      "created": 1698940800,
      "owned_by": "openai"
    }
  ]
}
```

## Testing Strategy

### Unit Testing
- Test individual response generators
- Test configuration loading
- Test TLS certificate handling

### Integration Testing
- Test with mTLS proxy server
- Test request/response flow
- Test error scenarios

### Load Testing
- Test with multiple concurrent requests
- Test memory usage under load
- Test response time consistency

## Deployment Options

### Local Development
- Run directly with `cargo run`
- Use local certificates
- Simple configuration

### Docker Deployment
- Containerized for consistency
- Include certificate generation
- Easy setup with docker-compose

### Production-like Testing
- Use same certificates as production
- Configure similar response times
- Test with production-like load

## Success Criteria

1. **Functionality**
   - All OpenAI API endpoints work correctly
   - mTLS authentication works
   - Streaming responses work
   - Error handling is realistic

2. **Performance**
   - Startup time < 1 second
   - Memory usage < 50MB
   - Response time configurable
   - Handles 100+ concurrent requests

3. **Usability**
   - Easy to configure and run
   - Clear documentation
   - Docker support
   - Integration with proxy testing

## Timeline

- **Day 1**: Basic server setup and mTLS configuration
- **Day 2**: Response generation and API endpoints
- **Day 3**: Advanced features and Docker support
- **Day 4**: Testing, integration, and documentation

## Next Steps

1. Create the project structure
2. Implement basic HTTP server with mTLS
3. Add OpenAI API endpoint handlers
4. Create response generation system
5. Add configuration management
6. Implement Docker support
7. Create comprehensive tests
8. Document setup and usage
