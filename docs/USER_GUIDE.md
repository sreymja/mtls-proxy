# mTLS Proxy User Guide

## Table of Contents

1. [Installation](#installation)
2. [Configuration](#configuration)
3. [Certificate Management](#certificate-management)
4. [Web Interface](#web-interface)
5. [Monitoring](#monitoring)
6. [Troubleshooting](#troubleshooting)
7. [Security Considerations](#security-considerations)

---

## Installation

### Prerequisites

- **Operating System**: Linux, macOS, or Windows
- **Rust**: Version 1.70 or higher
- **OpenSSL**: For certificate operations
- **System Requirements**:
  - Minimum 512MB RAM
  - 100MB disk space
  - Network access to target services

### Installation Methods

#### Method 1: From Source (Recommended)

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-org/mtls-proxy.git
   cd mtls-proxy
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Install the binary**:
   ```bash
   sudo cp target/release/mtls-proxy /usr/local/bin/
   sudo chmod +x /usr/local/bin/mtls-proxy
   ```

#### Method 2: Using Cargo Install

```bash
cargo install --git https://github.com/your-org/mtls-proxy.git
```

#### Method 3: Docker (Optional)

```bash
docker pull your-org/mtls-proxy:latest
docker run -p 8080:8080 -v $(pwd)/config:/app/config -v $(pwd)/certs:/app/certs your-org/mtls-proxy:latest
```

### Post-Installation Setup

1. **Create configuration directory**:
   ```bash
   sudo mkdir -p /etc/mtls-proxy
   sudo mkdir -p /var/log/mtls-proxy
   sudo mkdir -p /etc/mtls-proxy/certs
   ```

2. **Set up certificates**:
   ```bash
   # Copy your certificates to the certs directory
   sudo cp client.crt /etc/mtls-proxy/certs/
   sudo cp client.key /etc/mtls-proxy/certs/
   sudo cp ca.crt /etc/mtls-proxy/certs/
   
   # Set appropriate permissions
   sudo chmod 600 /etc/mtls-proxy/certs/*
   sudo chown root:root /etc/mtls-proxy/certs/*
   ```

3. **Create systemd service** (Linux):
   ```bash
   sudo tee /etc/systemd/system/mtls-proxy.service > /dev/null <<EOF
   [Unit]
   Description=mTLS Proxy
   After=network.target
   
   [Service]
   Type=simple
   User=mtls-proxy
   Group=mtls-proxy
   ExecStart=/usr/local/bin/mtls-proxy
   WorkingDirectory=/etc/mtls-proxy
   Restart=always
   RestartSec=5
   
   [Install]
   WantedBy=multi-user.target
   EOF
   
   sudo systemctl daemon-reload
   sudo systemctl enable mtls-proxy
   ```

---

## Configuration

### Configuration File Structure

The mTLS proxy uses TOML configuration files. The main configuration file is located at `config/default.toml`.

#### Default Configuration

```toml
[server]
host = "127.0.0.1"
port = 8443
max_connections = 1000
connection_timeout_secs = 30
connection_pool_size = 10
max_request_size_mb = 10
max_concurrent_requests = 100
rate_limit_requests_per_second = 100
rate_limit_burst_size = 200

[tls]
client_cert_path = "certs/client.crt"
client_key_path = "certs/client.key"
ca_cert_path = "certs/ca.crt"
verify_hostname = true

[target]
base_url = "https://gpt-4o-mini.internal:443"
timeout_secs = 60

[logging]
log_dir = "logs"
max_log_size_mb = 100
retention_days = 30
compression_enabled = true
sqlite_db_path = "logs/proxy_logs.db"

[ui]
enabled = true
port = 8080
```

### Configuration Sections

#### Server Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `host` | `127.0.0.1` | Server bind address |
| `port` | `8443` | Server port |
| `max_connections` | `1000` | Maximum concurrent connections |
| `connection_timeout_secs` | `30` | Connection timeout in seconds |
| `connection_pool_size` | `10` | Connection pool size |
| `max_request_size_mb` | `10` | Maximum request size in MB |
| `max_concurrent_requests` | `100` | Maximum concurrent requests |
| `rate_limit_requests_per_second` | `100` | Rate limit requests per second |
| `rate_limit_burst_size` | `200` | Rate limit burst size |

#### TLS Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `client_cert_path` | `certs/client.crt` | Client certificate path |
| `client_key_path` | `certs/client.key` | Client private key path |
| `ca_cert_path` | `certs/ca.crt` | CA certificate path |
| `verify_hostname` | `true` | Enable hostname verification |

#### Target Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `base_url` | `https://gpt-4o-mini.internal:443` | Target service URL |
| `timeout_secs` | `60` | Request timeout in seconds |

#### Logging Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `log_dir` | `logs` | Log directory |
| `max_log_size_mb` | `100` | Maximum log file size |
| `retention_days` | `30` | Log retention period |
| `compression_enabled` | `true` | Enable log compression |
| `sqlite_db_path` | `logs/proxy_logs.db` | SQLite database path |

### Environment Variables

You can override configuration values using environment variables:

```bash
export MTLS_PROXY_SERVER_PORT=8443
export MTLS_PROXY_TARGET_BASE_URL="https://api.example.com"
export MTLS_PROXY_LOGGING_LOG_DIR="/var/log/mtls-proxy"
```

### Configuration Validation

Validate your configuration:

```bash
# Using the web interface
curl http://127.0.0.1:8080/ui/api/config/validate

# Or check the configuration file syntax
mtls-proxy --validate-config
```

---

## Certificate Management

### Certificate Requirements

The mTLS proxy requires the following certificates:

1. **Client Certificate** (`client.crt`): Your client certificate in PEM format
2. **Client Private Key** (`client.key`): Your private key in PEM format
3. **CA Certificate** (`ca.crt`): Certificate Authority certificate in PEM format

### Certificate Formats

All certificates must be in PEM format:

```
-----BEGIN CERTIFICATE-----
MIIDiDCCAnCgAwIBAgIUZtVzwAULNmpRMhGZoCZ93kGnvewwDQYJKoZIhvcNAQEL
BQAwXDELMAkGA1UEBhMCVVMxCzAJBgNVBAgMAkNBMRYwFAYDVQQHDA1TYW4gRnJh
...
-----END CERTIFICATE-----
```

### Certificate Upload via Web Interface

1. **Access the web interface**: Navigate to `http://127.0.0.1:8080/ui`
2. **Go to Configuration**: Click on "Configuration" in the navigation
3. **Upload Certificate**: Use the file upload form to upload your certificate
4. **Verify Upload**: Check the certificate list to confirm the upload

### Certificate Upload via API

```bash
# Upload a certificate
curl -X POST http://127.0.0.1:8080/ui/api/certificates/upload \
  -F "certificate=@/path/to/your/certificate.crt"

# List certificates
curl http://127.0.0.1:8080/ui/api/certificates/list

# Delete a certificate
curl -X DELETE http://127.0.0.1:8080/ui/api/certificates/delete \
  -H "Content-Type: application/json" \
  -d '{"filename": "certificate.crt"}'
```

### Certificate Permissions

Ensure proper file permissions:

```bash
# Set restrictive permissions
chmod 600 certs/client.crt
chmod 600 certs/client.key
chmod 644 certs/ca.crt

# Set ownership (if running as a service)
chown mtls-proxy:mtls-proxy certs/*
```

### Certificate Renewal

When certificates expire:

1. **Obtain new certificates** from your CA
2. **Upload new certificates** via web interface or API
3. **Restart the proxy** to load new certificates:
   ```bash
   sudo systemctl restart mtls-proxy
   ```

---

## Web Interface

### Accessing the Web Interface

The web interface is available at `http://127.0.0.1:8080/ui` by default.

### Dashboard

The dashboard provides an overview of:
- **Server Status**: Health and uptime
- **Connection Statistics**: Active connections and request counts
- **Certificate Status**: Certificate validity and expiration
- **Recent Activity**: Latest audit log entries

### Configuration Management

The configuration page allows you to:
- **View Current Configuration**: See all current settings
- **Update Configuration**: Modify server, TLS, target, and logging settings
- **Validate Configuration**: Check configuration validity
- **Reset to Defaults**: Restore default configuration

### Certificate Management

The certificate management page provides:
- **Certificate List**: View all uploaded certificates
- **Certificate Upload**: Upload new certificates via file selection
- **Certificate Details**: View certificate information and validity
- **Certificate Deletion**: Remove certificates

### Logs Viewer

The logs viewer displays:
- **Request/Response Logs**: HTTP request and response details
- **Error Logs**: Error messages and stack traces
- **Audit Logs**: Configuration changes and certificate operations
- **Filtering**: Filter logs by date, type, and severity

### Audit Logs

The audit page shows:
- **Configuration Changes**: All configuration updates
- **Certificate Operations**: Uploads, deletions, and modifications
- **User Activity**: User actions and IP addresses
- **Statistics**: Event counts and trends

---

## Monitoring

### Health Checks

Monitor the proxy health:

```bash
# Basic health check
curl http://127.0.0.1:8080/health

# Expected response
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "0.1.0"
}
```

### Metrics

Access Prometheus metrics:

```bash
curl http://127.0.0.1:8080/metrics
```

Key metrics include:
- `mtls_proxy_requests_total`: Total request count
- `mtls_proxy_request_duration_seconds`: Request duration
- `mtls_proxy_active_connections`: Active connections
- `mtls_proxy_errors_total`: Error count

### Log Monitoring

Monitor logs for issues:

```bash
# Follow logs in real-time
tail -f /var/log/mtls-proxy/proxy.log

# Search for errors
grep ERROR /var/log/mtls-proxy/proxy.log

# Check audit logs
sqlite3 /var/log/mtls-proxy/audit_logs.db "SELECT * FROM audit_logs ORDER BY timestamp DESC LIMIT 10;"
```

### Performance Monitoring

Monitor performance metrics:

```bash
# Check memory usage
ps aux | grep mtls-proxy

# Monitor network connections
netstat -an | grep :8443

# Check disk usage
du -sh /var/log/mtls-proxy/
```

---

## Troubleshooting

### Common Issues

#### 1. Server Won't Start

**Symptoms**: `Error: Invalid byte 32, offset 4`

**Cause**: Corrupted certificate files

**Solution**:
```bash
# Check certificate files
file certs/client.crt
file certs/client.key
file certs/ca.crt

# Verify PEM format
openssl x509 -in certs/client.crt -text -noout

# Replace corrupted certificates
cp backup/client.crt certs/client.crt
```

#### 2. Certificate Validation Errors

**Symptoms**: `Certificate validation failed`

**Cause**: Invalid or expired certificates

**Solution**:
```bash
# Check certificate expiration
openssl x509 -in certs/client.crt -noout -dates

# Verify certificate chain
openssl verify -CAfile certs/ca.crt certs/client.crt

# Renew expired certificates
```

#### 3. Connection Timeouts

**Symptoms**: `Connection timeout`

**Cause**: Network issues or target service unavailable

**Solution**:
```bash
# Test network connectivity
ping gpt-4o-mini.internal

# Check target service
curl -v https://gpt-4o-mini.internal:443

# Verify firewall rules
sudo iptables -L
```

#### 4. Rate Limiting Issues

**Symptoms**: `429 Too Many Requests`

**Cause**: Exceeding rate limits

**Solution**:
```bash
# Check current rate limit settings
curl http://127.0.0.1:8080/ui/api/config/validate

# Adjust rate limits if needed
curl -X POST http://127.0.0.1:8080/ui/api/config/update \
  -H "Content-Type: application/json" \
  -d '{"server": {"rate_limit_requests_per_second": 200}}'
```

#### 5. Permission Denied

**Symptoms**: `Permission denied` errors

**Cause**: Incorrect file permissions

**Solution**:
```bash
# Fix certificate permissions
chmod 600 certs/client.crt
chmod 600 certs/client.key
chmod 644 certs/ca.crt

# Fix log directory permissions
chmod 755 logs/
chown mtls-proxy:mtls-proxy logs/
```

### Debug Mode

Enable debug logging:

```bash
# Set debug environment variable
export RUST_LOG=debug

# Start proxy with debug logging
mtls-proxy

# Or modify configuration
curl -X POST http://127.0.0.1:8080/ui/api/config/update \
  -H "Content-Type: application/json" \
  -d '{"logging": {"log_level": "debug"}}'
```

### Log Analysis

Analyze logs for issues:

```bash
# Search for errors
grep -i error /var/log/mtls-proxy/proxy.log

# Check recent activity
tail -n 100 /var/log/mtls-proxy/proxy.log

# Analyze audit logs
sqlite3 /var/log/mtls-proxy/audit_logs.db <<EOF
SELECT timestamp, event_type, details 
FROM audit_logs 
WHERE timestamp > datetime('now', '-1 hour')
ORDER BY timestamp DESC;
EOF
```

### Performance Issues

Troubleshoot performance problems:

```bash
# Check resource usage
top -p $(pgrep mtls-proxy)

# Monitor network activity
iftop -i eth0

# Check connection pool
netstat -an | grep :8443 | wc -l

# Analyze slow queries
grep "duration" /var/log/mtls-proxy/proxy.log | sort -k2 -n
```

---

## Security Considerations

### Certificate Security

1. **Secure Storage**: Store certificates in a secure location
2. **File Permissions**: Use restrictive permissions (600 for private keys)
3. **Regular Rotation**: Rotate certificates regularly
4. **Backup**: Keep secure backups of certificates

### Network Security

1. **Firewall**: Configure firewall rules to restrict access
2. **TLS**: Always use TLS for sensitive communications
3. **Rate Limiting**: Enable rate limiting to prevent abuse
4. **Access Control**: Implement proper authentication for production

### Log Security

1. **Log Rotation**: Implement log rotation to prevent disk space issues
2. **Log Encryption**: Consider encrypting sensitive log data
3. **Access Control**: Restrict access to log files
4. **Audit Trail**: Maintain audit logs for security events

### Production Deployment

1. **Service Account**: Run as a dedicated service account
2. **Resource Limits**: Set appropriate resource limits
3. **Monitoring**: Implement comprehensive monitoring
4. **Backup**: Regular backups of configuration and certificates
5. **Updates**: Keep the proxy updated with security patches

### Security Checklist

- [ ] Certificates have proper permissions (600 for keys, 644 for certs)
- [ ] Firewall rules restrict access to necessary ports only
- [ ] Rate limiting is enabled and configured appropriately
- [ ] Log files are rotated and have proper permissions
- [ ] Service runs as a dedicated user account
- [ ] Regular security updates are applied
- [ ] Audit logging is enabled and monitored
- [ ] Backup procedures are in place
- [ ] Monitoring and alerting are configured
- [ ] Incident response procedures are documented

---

## Support

### Getting Help

1. **Documentation**: Check this user guide and API documentation
2. **Logs**: Examine logs for error messages and debugging information
3. **Community**: Check GitHub issues and discussions
4. **Support**: Contact support team for enterprise issues

### Reporting Issues

When reporting issues, include:
- **Version**: mTLS proxy version
- **Platform**: Operating system and architecture
- **Configuration**: Relevant configuration settings
- **Logs**: Error logs and stack traces
- **Steps**: Steps to reproduce the issue
- **Expected vs Actual**: Expected behavior vs actual behavior

### Contributing

Contributions are welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

---

## Appendix

### Configuration Examples

#### Development Configuration

```toml
[server]
host = "127.0.0.1"
port = 8443
max_connections = 100

[tls]
client_cert_path = "certs/client.crt"
client_key_path = "certs/client.key"
verify_hostname = false

[target]
base_url = "https://localhost:9000"
timeout_secs = 30
```

#### Production Configuration

```toml
[server]
host = "0.0.0.0"
port = 8443
max_connections = 1000
rate_limit_requests_per_second = 50

[tls]
client_cert_path = "/etc/mtls-proxy/certs/client.crt"
client_key_path = "/etc/mtls-proxy/certs/client.key"
ca_cert_path = "/etc/mtls-proxy/certs/ca.crt"
verify_hostname = true

[target]
base_url = "https://api.production.com"
timeout_secs = 60

[logging]
log_dir = "/var/log/mtls-proxy"
max_log_size_mb = 100
retention_days = 90
```

### Useful Commands

```bash
# Start the proxy
mtls-proxy

# Start with custom config
mtls-proxy --config /path/to/config.toml

# Validate configuration
mtls-proxy --validate-config

# Show version
mtls-proxy --version

# Show help
mtls-proxy --help
```

### File Locations

| File | Default Location | Description |
|------|------------------|-------------|
| Binary | `/usr/local/bin/mtls-proxy` | Executable |
| Config | `config/default.toml` | Configuration file |
| Certificates | `certs/` | Certificate directory |
| Logs | `logs/` | Log directory |
| Database | `logs/proxy_logs.db` | SQLite database |
| Audit Logs | `logs/audit_logs.db` | Audit database |
