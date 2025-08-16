# mTLS Proxy

A high-performance, secure mTLS (mutual TLS) proxy server built in Rust with a modern web interface for configuration and monitoring.

## 🚀 Features

- **Secure mTLS Proxy**: Full mutual TLS support with certificate management
- **Web Interface**: Modern HTML-based UI for configuration and monitoring
- **REST API**: Comprehensive JSON API for programmatic access
- **Audit Logging**: SQLite-based audit trail for all operations
- **Rate Limiting**: Built-in rate limiting to prevent abuse
- **Metrics**: Prometheus-compatible metrics collection
- **Error Handling**: Standardized error responses and logging
- **Configuration Management**: TOML-based configuration with validation
- **Certificate Management**: Upload, validation, and management of certificates
- **Comprehensive Testing**: Unit, integration, performance, and security tests

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [API Documentation](#api-documentation)
- [Web Interface](#web-interface)
- [Testing](#testing)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

## 🛠 Installation

### Prerequisites

- **Rust**: Version 1.70 or higher
- **OpenSSL**: For certificate operations
- **System Requirements**: 512MB RAM, 100MB disk space

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/your-org/mtls-proxy.git
cd mtls-proxy

# Build the project
cargo build --release

# Install the binary
sudo cp target/release/mtls-proxy /usr/local/bin/
sudo chmod +x /usr/local/bin/mtls-proxy
```

### Using Cargo Install

```bash
cargo install --git https://github.com/your-org/mtls-proxy.git
```

### Docker (Optional)

```bash
docker pull your-org/mtls-proxy:latest
docker run -p 8080:8080 -v $(pwd)/config:/app/config -v $(pwd)/certs:/app/certs your-org/mtls-proxy:latest
```

## 🚀 Quick Start

### 1. Set Up Certificates

```bash
# Create certificate directory
mkdir -p certs

# Copy your certificates (replace with your actual certificates)
cp /path/to/your/client.crt certs/
cp /path/to/your/client.key certs/
cp /path/to/your/ca.crt certs/

# Set appropriate permissions
chmod 600 certs/client.key
chmod 644 certs/client.crt certs/ca.crt
```

### 2. Start the Proxy

```bash
# Start with default configuration
mtls-proxy

# Or start with custom configuration
mtls-proxy --config /path/to/config.toml
```

### 3. Access the Web Interface

Open your browser and navigate to:
- **Dashboard**: http://127.0.0.1:8080/ui
- **Configuration**: http://127.0.0.1:8080/ui/config
- **Logs**: http://127.0.0.1:8080/ui/logs
- **Audit**: http://127.0.0.1:8080/ui/audit

### 4. Test the API

```bash
# Health check
curl http://127.0.0.1:8080/health

# Validate configuration
curl http://127.0.0.1:8080/ui/api/config/validate

# List certificates
curl http://127.0.0.1:8080/ui/api/certificates/list
```

## ⚙️ Configuration

The mTLS proxy uses TOML configuration files. The default configuration is located at `config/default.toml`.

### Basic Configuration

```toml
[server]
host = "127.0.0.1"
port = 8443
max_connections = 1000
rate_limit_requests_per_second = 100

[tls]
client_cert_path = "certs/client.crt"
client_key_path = "certs/client.key"
ca_cert_path = "certs/ca.crt"
verify_hostname = true

[target]
base_url = "https://api.example.com"
timeout_secs = 60

[logging]
log_dir = "logs"
max_log_size_mb = 100
retention_days = 30
```

### Environment Variables

You can override configuration values using environment variables:

```bash
export MTLS_PROXY_SERVER_PORT=8443
export MTLS_PROXY_TARGET_BASE_URL="https://api.example.com"
export MTLS_PROXY_LOGGING_LOG_DIR="/var/log/mtls-proxy"
```

For detailed configuration options, see the [User Guide](docs/USER_GUIDE.md#configuration).

## 📚 API Documentation

The mTLS proxy provides a comprehensive REST API for programmatic access.

### Key Endpoints

- **Health Check**: `GET /health`
- **Configuration**: `GET /ui/api/config/validate`, `POST /ui/api/config/update`
- **Certificates**: `GET /ui/api/certificates/list`, `POST /ui/api/certificates/upload`, `DELETE /ui/api/certificates/delete`
- **Audit Logs**: `GET /ui/api/audit/logs`, `GET /ui/api/audit/stats`
- **Metrics**: `GET /metrics`

### Example API Usage

```bash
# Update configuration
curl -X POST http://127.0.0.1:8080/ui/api/config/update \
  -H "Content-Type: application/json" \
  -d '{
    "server": {
      "port": 8443,
      "max_connections": 1000
    }
  }'

# Upload certificate
curl -X POST http://127.0.0.1:8080/ui/api/certificates/upload \
  -F "certificate=@/path/to/certificate.crt"

# Get audit logs
curl "http://127.0.0.1:8080/ui/api/audit/logs?limit=10&offset=0"
```

For complete API documentation, see the [API Documentation](docs/API_DOCUMENTATION.md).

## 🌐 Web Interface

The mTLS proxy includes a modern web interface for easy configuration and monitoring.

### Features

- **Dashboard**: Overview of server status, connections, and recent activity
- **Configuration Management**: Visual configuration editor with validation
- **Certificate Management**: Upload, view, and manage certificates
- **Logs Viewer**: Real-time log viewing with filtering
- **Audit Trail**: Complete audit log with statistics
- **Metrics**: Performance metrics and monitoring

### Accessing the Interface

- **Main Dashboard**: http://127.0.0.1:8080/ui
- **Configuration**: http://127.0.0.1:8080/ui/config
- **Logs**: http://127.0.0.1:8080/ui/logs
- **Audit**: http://127.0.0.1:8080/ui/audit

## 🧪 Testing

The mTLS proxy includes comprehensive testing across multiple categories.

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test categories
cargo test --test performance_test    # Performance tests
cargo test --test security_test       # Security tests
cargo test --lib                      # Unit tests
```

### Test Coverage

- **Unit Tests**: 47 tests covering all modules
- **Integration Tests**: 5 tests covering API endpoints
- **Performance Tests**: 4 tests for load handling and benchmarks
- **Security Tests**: 6 tests for security validation

### Test Categories

1. **Unit Tests**: Individual module functionality
2. **Integration Tests**: API endpoint testing
3. **Performance Tests**: Load testing and benchmarks
4. **Security Tests**: Input validation, certificate security, file path security

For detailed testing information, see the [Developer Guide](docs/DEVELOPER_GUIDE.md#testing).

## 📖 Documentation

Comprehensive documentation is available in the `docs/` directory:

- **[API Documentation](docs/API_DOCUMENTATION.md)**: Complete API reference with examples
- **[User Guide](docs/USER_GUIDE.md)**: Installation, configuration, and troubleshooting
- **[Developer Guide](docs/DEVELOPER_GUIDE.md)**: Architecture, development setup, and contributing

### Quick Links

- [Installation Guide](docs/USER_GUIDE.md#installation)
- [Configuration Guide](docs/USER_GUIDE.md#configuration)
- [Certificate Management](docs/USER_GUIDE.md#certificate-management)
- [Troubleshooting](docs/USER_GUIDE.md#troubleshooting)
- [API Examples](docs/API_DOCUMENTATION.md#examples)

## 🔧 Development

### Prerequisites

- Rust 1.70+
- OpenSSL development libraries
- Git

### Building from Source

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

### Development Setup

1. **Install Rust**: Follow the [official Rust installation guide](https://rustup.rs/)
2. **Install Dependencies**: Install OpenSSL development libraries
3. **Clone Repository**: Clone the repository and navigate to the directory
4. **Build Project**: Run `cargo build` to build the project
5. **Run Tests**: Run `cargo test` to ensure everything works

For detailed development information, see the [Developer Guide](docs/DEVELOPER_GUIDE.md).

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](docs/DEVELOPER_GUIDE.md#contributing) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Add tests for new functionality
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m "feat: add new feature"`
7. Push and create a pull request

### Code Style

- Follow Rust style guidelines
- Use `cargo fmt` for code formatting
- Use `cargo clippy` for linting
- Add documentation for new functions and modules

## 📊 Project Status

### Completed Features

- ✅ **Phase 1**: Certificate upload, audit logging, error handling
- ✅ **Phase 2.1**: Comprehensive testing (47 unit + 5 integration tests)
- ✅ **Phase 2.2**: Performance testing (4 performance tests)
- ✅ **Phase 2.3**: Security testing (6 security tests)
- ✅ **Phase 3.1**: Documentation (API, User, Developer guides)

### Current Status

- **Total Tests**: 62 tests (all passing)
- **Documentation**: Complete API, User, and Developer guides
- **Security**: Comprehensive security validation
- **Performance**: Load testing and benchmarks implemented

### Next Steps

- **Phase 3.2**: Deployment preparation (RPM packages, Docker support)
- **Phase 4**: Dioxus UI framework migration
- **Phase 5**: Advanced features (monitoring, configuration templates)

## 🐛 Troubleshooting

### Common Issues

1. **Server Won't Start**: Check certificate files and permissions
2. **Certificate Errors**: Verify PEM format and file paths
3. **Permission Denied**: Set appropriate file permissions
4. **Connection Timeouts**: Check network connectivity and target service

### Getting Help

1. **Documentation**: Check the [User Guide](docs/USER_GUIDE.md#troubleshooting)
2. **Logs**: Examine logs for error messages
3. **Issues**: Search existing GitHub issues
4. **Community**: Join community discussions

For detailed troubleshooting, see the [Troubleshooting Guide](docs/USER_GUIDE.md#troubleshooting).

## 📈 Performance

The mTLS proxy is designed for high performance:

- **Concurrent Requests**: Handles 100+ concurrent requests
- **Response Time**: < 100ms for simple requests
- **Memory Usage**: < 100MB under normal load
- **Rate Limiting**: Configurable rate limiting (default: 100 req/sec)

### Performance Tests

All performance tests pass:
- ✅ Concurrent request handling
- ✅ Rate limiting effectiveness
- ✅ Memory usage under load
- ✅ Performance benchmarks

## 🔒 Security

The mTLS proxy implements comprehensive security measures:

- **Certificate Validation**: Full mTLS certificate validation
- **Input Validation**: SQL injection, XSS, path traversal protection
- **File Security**: Path traversal and symlink attack prevention
- **Rate Limiting**: Built-in rate limiting to prevent abuse
- **Audit Logging**: Complete audit trail for all operations

### Security Tests

All security tests pass:
- ✅ mTLS certificate validation
- ✅ Input validation
- ✅ File path security
- ✅ Configuration security
- ✅ Certificate security
- ✅ Authentication security

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Warp](https://github.com/seanmonstar/warp) - Async HTTP server framework
- [Rustls](https://github.com/rustls/rustls) - TLS implementation
- [SQLite](https://www.sqlite.org/) - Database engine
- [Prometheus](https://prometheus.io/) - Metrics format

## 📞 Support

- **Documentation**: [User Guide](docs/USER_GUIDE.md), [API Documentation](docs/API_DOCUMENTATION.md)
- **Issues**: [GitHub Issues](https://github.com/your-org/mtls-proxy/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/mtls-proxy/discussions)
- **Email**: support@your-org.com

---

**mTLS Proxy** - Secure, high-performance mTLS proxy with modern web interface
