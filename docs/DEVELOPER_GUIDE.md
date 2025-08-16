# mTLS Proxy Developer Guide

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Project Structure](#project-structure)
3. [Module Documentation](#module-documentation)
4. [Development Setup](#development-setup)
5. [Testing](#testing)
6. [Contributing](#contributing)
7. [Code Style](#code-style)
8. [Performance Considerations](#performance-considerations)

---

## Architecture Overview

### High-Level Architecture

The mTLS proxy is built using Rust with the following key components:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Client        │    │   mTLS Proxy    │    │   Target        │
│   Application   │───▶│   Server        │───▶│   Service       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌─────────────────┐
                       │   Web UI        │
                       │   & API         │
                       └─────────────────┘
```

### Core Components

1. **Proxy Server**: Main proxy logic handling mTLS connections
2. **Web Interface**: HTML-based UI for configuration and monitoring
3. **REST API**: JSON-based API for programmatic access
4. **Configuration Manager**: TOML-based configuration system
5. **Certificate Manager**: Certificate upload and management
6. **Audit Logger**: SQLite-based audit logging
7. **Error Handler**: Standardized error handling and responses
8. **Metrics**: Prometheus-compatible metrics collection

### Technology Stack

- **Language**: Rust 1.70+
- **Web Framework**: Warp (async HTTP server)
- **TLS**: Rustls (TLS implementation)
- **Configuration**: config-rs (TOML configuration)
- **Database**: SQLite (logging and audit)
- **Metrics**: Prometheus (monitoring)
- **Testing**: tokio-test, tempfile (testing utilities)

---

## Project Structure

```
mtls-proxy/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Configuration structures
│   ├── proxy.rs             # Main proxy server logic
│   ├── tls.rs               # TLS client configuration
│   ├── config_manager.rs    # Configuration management
│   ├── audit.rs             # Audit logging
│   ├── errors.rs            # Error types and codes
│   ├── error_handler.rs     # Error handling middleware
│   └── tests.rs             # Integration tests
├── tests/
│   ├── performance_test.rs  # Performance tests
│   └── security_test.rs     # Security tests
├── docs/                    # Documentation
├── config/                  # Configuration files
├── certs/                   # Certificate files
├── logs/                    # Log files
├── Cargo.toml              # Dependencies
└── README.md               # Project overview
```

---

## Module Documentation

### Main Application (`main.rs`)

**Purpose**: Application entry point and server startup

**Key Functions**:
- `main()`: Application entry point
- Server initialization and configuration loading
- Signal handling and graceful shutdown

**Dependencies**:
- `lib.rs` for core functionality
- `config.rs` for configuration loading

### Library Exports (`lib.rs`)

**Purpose**: Public API exports and module organization

**Exports**:
```rust
pub mod config;
pub mod proxy;
pub mod tls;
pub mod config_manager;
pub mod audit;
pub mod errors;
pub mod error_handler;
```

### Configuration (`config.rs`)

**Purpose**: Configuration structures and validation

**Key Structures**:
```rust
pub struct Config {
    pub server: ServerConfig,
    pub tls: TlsConfig,
    pub target: TargetConfig,
    pub logging: LoggingConfig,
    pub ui: UiConfig,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    // ... other fields
}
```

**Key Functions**:
- `Config::load()`: Load configuration from files and environment
- `Config::validate()`: Validate configuration values
- `Config::default()`: Create default configuration

### Proxy Server (`proxy.rs`)

**Purpose**: Main proxy server implementation

**Key Components**:
- `ProxyServer`: Main server struct
- `AppState`: Application state shared across handlers
- Route definitions and handlers
- Web UI templates

**Key Functions**:
- `ProxyServer::new()`: Create new proxy server instance
- `ProxyServer::start()`: Start the server
- Route handlers for API endpoints
- Web UI handlers

**Route Structure**:
```rust
// Health check
GET /health

// Configuration API
GET /ui/api/config/validate
POST /ui/api/config/update

// Certificate API
GET /ui/api/certificates/list
POST /ui/api/certificates/upload
DELETE /ui/api/certificates/delete

// Audit API
GET /ui/api/audit/logs
GET /ui/api/audit/stats

// Metrics
GET /metrics

// Web UI
GET /ui
GET /ui/config
GET /ui/logs
GET /ui/audit
```

### TLS Client (`tls.rs`)

**Purpose**: TLS client configuration and management

**Key Structures**:
```rust
pub struct TlsClient {
    client: reqwest::Client,
    config: TlsConfig,
}
```

**Key Functions**:
- `TlsClient::new()`: Create new TLS client
- `TlsClient::request()`: Make TLS request to target
- Certificate loading and validation

### Configuration Manager (`config_manager.rs`)

**Purpose**: Configuration file operations and validation

**Key Functions**:
- `update_config()`: Update configuration file
- `upload_certificate()`: Upload and validate certificates
- `list_certificates()`: List available certificates
- `delete_certificate()`: Delete certificate files

### Audit Logger (`audit.rs`)

**Purpose**: SQLite-based audit logging

**Key Structures**:
```rust
pub struct AuditLogger {
    db_path: PathBuf,
}

pub struct AuditEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub user: Option<String>,
    pub ip_address: Option<String>,
    pub details: String,
    pub metadata: Option<serde_json::Value>,
}
```

**Key Functions**:
- `AuditLogger::new()`: Create new audit logger
- `AuditLogger::log_event()`: Log audit event
- `AuditLogger::get_logs()`: Retrieve audit logs
- `AuditLogger::get_stats()`: Get audit statistics

### Error Handling (`errors.rs`)

**Purpose**: Standardized error types and codes

**Key Structures**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    ValidationError,
    ConfigValidationFailed,
    CertificateNotFound,
    FilesystemError,
    DatabaseError,
    InternalError,
    NotFound,
    MethodNotAllowed,
    PayloadTooLarge,
    RateLimitExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: ErrorCode,
    pub message: String,
    pub details: Option<String>,
    pub path: String,
    pub request_id: String,
    pub field_errors: Option<Vec<FieldError>>,
}
```

**Key Functions**:
- Error creation helpers (`validation_error`, `config_error`, etc.)
- Error code to HTTP status mapping
- Field-level error handling

### Error Handler (`error_handler.rs`)

**Purpose**: Centralized error handling middleware

**Key Functions**:
- `handle_rejection()`: Main error handler for Warp rejections
- `create_error_response()`: Create standardized error responses
- `create_success_response()`: Create success responses
- Logging functions for different error types

---

## Development Setup

### Prerequisites

1. **Rust Toolchain**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   rustup default stable
   ```

2. **Development Dependencies**:
   ```bash
   # macOS
   brew install openssl pkg-config
   
   # Ubuntu/Debian
   sudo apt-get install pkg-config libssl-dev
   
   # CentOS/RHEL
   sudo yum install pkg-config openssl-devel
   ```

### Building

```bash
# Clone repository
git clone https://github.com/your-org/mtls-proxy.git
cd mtls-proxy

# Build in debug mode
cargo build

# Build in release mode
cargo build --release

# Run in development mode
RUST_ENV=development cargo run
```

### Development Configuration

Create `config/local.toml` for development:

```toml
[server]
host = "127.0.0.1"
port = 8443

[tls]
client_cert_path = "certs/client.crt"
client_key_path = "certs/client.key"
verify_hostname = false

[target]
base_url = "https://localhost:9000"
timeout_secs = 30

[logging]
log_dir = "logs"
sqlite_db_path = "logs/proxy_logs.db"
```

### IDE Setup

#### VS Code

Install recommended extensions:
- `rust-lang.rust-analyzer`
- `serayuzgur.crates`
- `vadimcn.vscode-lldb`

#### IntelliJ IDEA / CLion

Install Rust plugin and configure:
- Rust toolchain
- Cargo project settings
- Debug configurations

---

## Testing

### Test Structure

```
tests/
├── performance_test.rs  # Performance and load tests
└── security_test.rs     # Security validation tests

src/
└── tests.rs             # Integration tests
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test performance_test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_mtls_certificate_validation

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Test Categories

#### Unit Tests

Located in each module with `#[cfg(test)] mod tests`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_validation() {
        // Test implementation
    }
}
```

#### Integration Tests

Located in `src/tests.rs`:

```rust
#[tokio::test]
async fn test_configuration_api_endpoints() {
    // Test API endpoints
}
```

#### Performance Tests

Located in `tests/performance_test.rs`:

```rust
#[tokio::test]
async fn test_concurrent_request_handling() {
    // Test concurrent request handling
}
```

#### Security Tests

Located in `tests/security_test.rs`:

```rust
#[tokio::test]
async fn test_input_validation() {
    // Test security validations
}
```

### Test Utilities

#### Temporary Files

```rust
use tempfile::TempDir;

#[test]
fn test_with_temp_files() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    // Test implementation
}
```

#### Test Configuration

```rust
fn create_test_config() -> Config {
    let mut config = Config::default();
    config.logging.sqlite_db_path = PathBuf::from("test_logs.db");
    config.logging.log_dir = PathBuf::from("test_logs");
    config
}
```

---

## Contributing

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```

3. **Make your changes**:
   - Follow the code style guidelines
   - Add tests for new functionality
   - Update documentation

4. **Run tests**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt --check
   ```

5. **Commit your changes**:
   ```bash
   git add .
   git commit -m "feat: add new feature description"
   ```

6. **Push and create pull request**

### Commit Message Format

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build/tooling changes

Examples:
```
feat(proxy): add rate limiting support
fix(config): resolve certificate validation issue
docs(api): update endpoint documentation
test(security): add input validation tests
```

### Pull Request Guidelines

1. **Title**: Clear, descriptive title
2. **Description**: Detailed description of changes
3. **Tests**: Include tests for new functionality
4. **Documentation**: Update relevant documentation
5. **Breaking Changes**: Clearly mark breaking changes

### Code Review Process

1. **Automated Checks**: CI/CD pipeline runs tests
2. **Code Review**: At least one maintainer review required
3. **Approval**: Changes approved before merge
4. **Merge**: Squash and merge to main branch

---

## Code Style

### Rust Style Guidelines

Follow the official Rust style guide and use `rustfmt`:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Naming Conventions

- **Functions**: `snake_case`
- **Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Types**: `PascalCase`
- **Modules**: `snake_case`

### Documentation

#### Function Documentation

```rust
/// Updates the proxy configuration with new settings.
///
/// # Arguments
///
/// * `config` - The new configuration to apply
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the configuration
/// is invalid or cannot be applied.
///
/// # Examples
///
/// ```
/// use mtls_proxy::config_manager;
///
/// let config = Config::default();
/// config_manager::update_config(config)?;
/// ```
pub fn update_config(config: Config) -> Result<(), AppError> {
    // Implementation
}
```

#### Module Documentation

```rust
//! Configuration management module.
//!
//! This module provides functionality for managing proxy configuration,
//! including loading from files, validation, and updates.
```

### Error Handling

Use the standardized error types:

```rust
use crate::errors::{AppError, validation_error, config_error};

fn validate_config(config: &Config) -> Result<(), AppError> {
    if config.server.port == 0 {
        return Err(validation_error("Server port cannot be 0"));
    }
    
    if !config.tls.client_cert_path.exists() {
        return Err(config_error("Client certificate not found"));
    }
    
    Ok(())
}
```

### Logging

Use structured logging with appropriate levels:

```rust
use tracing::{info, warn, error, debug};

fn process_request(request: &Request) -> Result<Response, AppError> {
    debug!("Processing request: {:?}", request);
    
    match handle_request(request) {
        Ok(response) => {
            info!("Request processed successfully");
            Ok(response)
        }
        Err(e) => {
            error!("Request processing failed: {}", e);
            Err(e)
        }
    }
}
```

---

## Performance Considerations

### Memory Management

1. **Connection Pooling**: Reuse TLS connections
2. **Buffer Management**: Use appropriate buffer sizes
3. **Resource Cleanup**: Ensure proper cleanup of resources

### Async Programming

1. **Non-blocking Operations**: Use async/await for I/O operations
2. **Concurrent Processing**: Handle multiple requests concurrently
3. **Backpressure**: Implement backpressure for high load

### Database Operations

1. **Connection Pooling**: Use connection pools for SQLite
2. **Batch Operations**: Batch database operations when possible
3. **Indexing**: Ensure proper indexing for audit logs

### Monitoring

1. **Metrics Collection**: Collect performance metrics
2. **Resource Monitoring**: Monitor memory and CPU usage
3. **Alerting**: Set up alerts for performance issues

### Optimization Techniques

1. **Zero-copy**: Minimize data copying
2. **Lazy Loading**: Load resources on demand
3. **Caching**: Cache frequently accessed data
4. **Compression**: Use compression for large responses

---

## Debugging

### Debug Logging

Enable debug logging:

```bash
export RUST_LOG=debug
cargo run
```

### Profiling

Use profiling tools:

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph

# Use perf (Linux)
perf record --call-graph=dwarf target/release/mtls-proxy
perf report
```

### Memory Analysis

Use memory analysis tools:

```bash
# Install cargo-valgrind
cargo install cargo-valgrind

# Run with valgrind
cargo valgrind run
```

---

## Deployment

### Release Process

1. **Version Bump**: Update version in `Cargo.toml`
2. **Changelog**: Update `CHANGELOG.md`
3. **Tag Release**: Create git tag
4. **Build Artifacts**: Build release binaries
5. **Publish**: Publish to package repositories

### Release Checklist

- [ ] All tests passing
- [ ] Documentation updated
- [ ] Changelog updated
- [ ] Version bumped
- [ ] Release notes written
- [ ] Binaries built and tested
- [ ] Release tagged and published

---

## Support

### Getting Help

1. **Documentation**: Check this developer guide
2. **Issues**: Search existing GitHub issues
3. **Discussions**: Use GitHub discussions
4. **Community**: Join community channels

### Reporting Bugs

When reporting bugs, include:
- **Version**: mTLS proxy version
- **Platform**: OS and architecture
- **Steps**: Steps to reproduce
- **Expected vs Actual**: Expected vs actual behavior
- **Logs**: Relevant log output
- **Environment**: Development environment details

### Feature Requests

When requesting features:
- **Use Case**: Describe the use case
- **Benefits**: Explain the benefits
- **Implementation**: Suggest implementation approach
- **Priority**: Indicate priority level
