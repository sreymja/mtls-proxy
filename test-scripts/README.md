# Test Scripts

This directory contains various test scripts for the mTLS proxy server.

## Available Test Scripts

### `test_proxy.sh`
Comprehensive test script that tests the basic functionality of the proxy server including:
- Health endpoint testing
- UI endpoint testing
- Logs endpoint testing
- API logs endpoint testing
- Proxy endpoint testing

### `test_simple.sh`
Simple test script that provides basic usage instructions and quick testing commands.

### `test_ui_refactor.sh`
Tests the UI refactoring changes, specifically testing that all UI pages work with the new embedded CSS approach.

### `test_config_ui.sh`
Tests the configuration UI functionality including:
- Configuration validation
- Configuration updates
- Certificate management
- Authentication (if enabled)

### `test_metrics.sh`
Tests the metrics and monitoring endpoints of the proxy server.

### `test_auth.sh`
Tests authentication and authorization functionality (if enabled).

## Usage

All scripts are executable and can be run directly:

```bash
# Run a specific test
./test-scripts/test_proxy.sh

# Run all tests (if you have a script to do so)
for script in test-scripts/*.sh; do
    echo "Running $script..."
    ./$script
done
```

## Port Configuration

All test scripts have been updated to use the new port strategy:
- **8440**: HTTP proxy server (default)
- **8443**: HTTPS proxy server (when TLS enabled)
- **8444**: Mock server for testing (HTTPS)

## Prerequisites

Before running the test scripts, ensure:
1. The proxy server is built (`cargo build --release`)
2. Certificates are available in the `certs/` directory
3. The mock server is running (if testing mTLS functionality)
4. The proxy server is running on the expected port

## Notes

- Some tests may require the proxy server to be running
- Some tests may require the mock server to be running
- Tests are designed for development and testing environments
- All scripts include proper error handling and status reporting
