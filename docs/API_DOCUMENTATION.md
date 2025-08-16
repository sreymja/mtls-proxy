# mTLS Proxy API Documentation

## Overview

The mTLS Proxy provides a RESTful API for managing proxy configuration, certificates, and monitoring. The API is designed to be secure, performant, and easy to use.

## Base URL

- **Development**: `http://127.0.0.1:8080`
- **Production**: Configure via `config.toml`

## Authentication

Currently, the API does not require authentication for development purposes. For production deployments, consider implementing proper authentication mechanisms.

## API Endpoints

### Health Check

#### GET /health

Returns the health status of the proxy server.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "version": "0.1.0"
}
```

**Status Codes:**
- `200 OK`: Server is healthy
- `503 Service Unavailable`: Server is unhealthy

---

### Configuration Management

#### GET /ui/api/config/validate

Validates the current configuration.

**Response:**
```json
{
  "status": "success",
  "message": "Configuration is valid",
  "data": {
    "server": {
      "host": "127.0.0.1",
      "port": 8443,
      "max_connections": 1000
    },
    "tls": {
      "client_cert_path": "certs/client.crt",
      "client_key_path": "certs/client.key",
      "verify_hostname": true
    },
    "target": {
      "base_url": "https://example.com",
      "timeout_secs": 60
    }
  }
}
```

**Status Codes:**
- `200 OK`: Configuration is valid
- `400 Bad Request`: Configuration validation failed

#### POST /ui/api/config/update

Updates the proxy configuration.

**Request Body:**
```json
{
  "server": {
    "host": "127.0.0.1",
    "port": 8443,
    "max_connections": 1000,
    "connection_timeout_secs": 30,
    "connection_pool_size": 10,
    "max_request_size_mb": 10,
    "max_concurrent_requests": 100,
    "rate_limit_requests_per_second": 100,
    "rate_limit_burst_size": 200
  },
  "tls": {
    "client_cert_path": "certs/client.crt",
    "client_key_path": "certs/client.key",
    "ca_cert_path": "certs/ca.crt",
    "verify_hostname": true
  },
  "target": {
    "base_url": "https://example.com",
    "timeout_secs": 60
  },
  "logging": {
    "log_dir": "logs",
    "max_log_size_mb": 100,
    "retention_days": 30,
    "compression_enabled": true,
    "sqlite_db_path": "logs/proxy_logs.db"
  }
}
```

**Response:**
```json
{
  "status": "success",
  "message": "Configuration updated successfully",
  "data": {
    "updated_at": "2024-01-15T10:30:00Z"
  }
}
```

**Status Codes:**
- `200 OK`: Configuration updated successfully
- `400 Bad Request`: Invalid configuration data
- `500 Internal Server Error`: Failed to update configuration

---

### Certificate Management

#### GET /ui/api/certificates/list

Lists all available certificates.

**Response:**
```json
{
  "status": "success",
  "data": {
    "certificates": [
      {
        "name": "client.crt",
        "size": 2048,
        "modified": "2024-01-15T10:30:00Z"
      },
      {
        "name": "ca.crt",
        "size": 1024,
        "modified": "2024-01-15T10:30:00Z"
      }
    ]
  }
}
```

**Status Codes:**
- `200 OK`: Certificates listed successfully
- `500 Internal Server Error`: Failed to list certificates

#### POST /ui/api/certificates/upload

Uploads a new certificate file using multipart form data.

**Request:**
- **Content-Type**: `multipart/form-data`
- **Body**: Form data with file field named `certificate`

**Response:**
```json
{
  "status": "success",
  "message": "Certificate uploaded successfully",
  "data": {
    "filename": "new_cert.crt",
    "size": 2048,
    "uploaded_at": "2024-01-15T10:30:00Z"
  }
}
```

**Status Codes:**
- `200 OK`: Certificate uploaded successfully
- `400 Bad Request`: Invalid certificate data
- `413 Payload Too Large`: Certificate file too large
- `500 Internal Server Error`: Failed to upload certificate

#### DELETE /ui/api/certificates/delete

Deletes a certificate file.

**Request Body:**
```json
{
  "filename": "certificate.crt"
}
```

**Response:**
```json
{
  "status": "success",
  "message": "Certificate deleted successfully",
  "data": {
    "deleted_file": "certificate.crt",
    "deleted_at": "2024-01-15T10:30:00Z"
  }
}
```

**Status Codes:**
- `200 OK`: Certificate deleted successfully
- `400 Bad Request`: Invalid filename
- `404 Not Found`: Certificate file not found
- `500 Internal Server Error`: Failed to delete certificate

---

### Audit Logging

#### GET /ui/api/audit/logs

Retrieves audit logs with optional pagination.

**Query Parameters:**
- `limit` (optional): Number of logs to return (default: 100, max: 1000)
- `offset` (optional): Number of logs to skip (default: 0)

**Response:**
```json
{
  "status": "success",
  "data": {
    "logs": [
      {
        "id": 1,
        "timestamp": "2024-01-15T10:30:00Z",
        "event_type": "CONFIG_UPDATE",
        "user": "admin",
        "ip_address": "127.0.0.1",
        "details": "Updated server configuration",
        "metadata": {
          "old_port": 8080,
          "new_port": 8443
        }
      },
      {
        "id": 2,
        "timestamp": "2024-01-15T10:25:00Z",
        "event_type": "CERTIFICATE_UPLOAD",
        "user": "admin",
        "ip_address": "127.0.0.1",
        "details": "Uploaded new certificate",
        "metadata": {
          "filename": "new_cert.crt",
          "size": 2048
        }
      }
    ],
    "total": 150,
    "limit": 100,
    "offset": 0
  }
}
```

**Status Codes:**
- `200 OK`: Audit logs retrieved successfully
- `400 Bad Request`: Invalid query parameters
- `500 Internal Server Error`: Failed to retrieve audit logs

#### GET /ui/api/audit/stats

Retrieves audit log statistics.

**Response:**
```json
{
  "status": "success",
  "data": {
    "total_events": 150,
    "events_today": 25,
    "events_by_type": {
      "CONFIG_UPDATE": 50,
      "CERTIFICATE_UPLOAD": 30,
      "CERTIFICATE_DELETE": 20,
      "LOGIN": 50
    },
    "last_event": "2024-01-15T10:30:00Z"
  }
}
```

**Status Codes:**
- `200 OK`: Audit stats retrieved successfully
- `500 Internal Server Error`: Failed to retrieve audit stats

---

### Metrics

#### GET /metrics

Returns Prometheus-formatted metrics.

**Response:**
```
# HELP mtls_proxy_requests_total Total number of requests
# TYPE mtls_proxy_requests_total counter
mtls_proxy_requests_total{method="GET",status="200"} 150
mtls_proxy_requests_total{method="POST",status="200"} 75

# HELP mtls_proxy_request_duration_seconds Request duration in seconds
# TYPE mtls_proxy_request_duration_seconds histogram
mtls_proxy_request_duration_seconds_bucket{le="0.1"} 100
mtls_proxy_request_duration_seconds_bucket{le="0.5"} 200
mtls_proxy_request_duration_seconds_bucket{le="1.0"} 225
mtls_proxy_request_duration_seconds_bucket{le="+Inf"} 225

# HELP mtls_proxy_active_connections Current number of active connections
# TYPE mtls_proxy_active_connections gauge
mtls_proxy_active_connections 5
```

**Status Codes:**
- `200 OK`: Metrics retrieved successfully

---

### Web UI

#### GET /ui

Returns the main dashboard HTML page.

#### GET /ui/config

Returns the configuration management HTML page.

#### GET /ui/logs

Returns the logs viewer HTML page.

#### GET /ui/audit

Returns the audit log viewer HTML page.

---

## Error Codes

The API uses standardized error codes and responses. All error responses follow this format:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Human-readable error message",
    "details": "Additional error details",
    "path": "/api/endpoint",
    "request_id": "uuid-request-id"
  }
}
```

### Standard Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `VALIDATION_ERROR` | 400 | Input validation failed |
| `CONFIG_VALIDATION_FAILED` | 400 | Configuration validation failed |
| `CERTIFICATE_NOT_FOUND` | 404 | Certificate file not found |
| `FILESYSTEM_ERROR` | 500 | File system operation failed |
| `DATABASE_ERROR` | 500 | Database operation failed |
| `INTERNAL_ERROR` | 500 | Internal server error |
| `NOT_FOUND` | 404 | Resource not found |
| `METHOD_NOT_ALLOWED` | 405 | HTTP method not allowed |
| `PAYLOAD_TOO_LARGE` | 413 | Request payload too large |
| `RATE_LIMIT_EXCEEDED` | 429 | Rate limit exceeded |

### Field-Level Errors

For validation errors, the response includes field-specific errors:

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Validation failed",
    "details": "One or more fields failed validation",
    "field_errors": [
      {
        "field": "server.port",
        "message": "Port must be between 1 and 65535"
      },
      {
        "field": "target.base_url",
        "message": "Base URL must start with 'https://'"
      }
    ],
    "path": "/ui/api/config/update",
    "request_id": "uuid-request-id"
  }
}
```

---

## Rate Limiting

The API implements rate limiting to prevent abuse:

- **Default Rate Limit**: 100 requests per second
- **Burst Size**: 200 requests
- **Headers**: Rate limit information is included in response headers:
  - `X-RateLimit-Limit`: Maximum requests per window
  - `X-RateLimit-Remaining`: Remaining requests in current window
  - `X-RateLimit-Reset`: Time when the rate limit resets

When rate limited, the API returns:
- **Status Code**: `429 Too Many Requests`
- **Error Code**: `RATE_LIMIT_EXCEEDED`

---

## Request/Response Headers

### Standard Headers

| Header | Description |
|--------|-------------|
| `Content-Type` | Request/response content type (application/json) |
| `Accept` | Expected response format |
| `User-Agent` | Client identifier |
| `X-Request-ID` | Request correlation ID |

### Custom Headers

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Rate limit maximum |
| `X-RateLimit-Remaining` | Rate limit remaining |
| `X-RateLimit-Reset` | Rate limit reset time |

---

## Examples

### cURL Examples

#### Health Check
```bash
curl -X GET http://127.0.0.1:8080/health
```

#### Update Configuration
```bash
curl -X POST http://127.0.0.1:8080/ui/api/config/update \
  -H "Content-Type: application/json" \
  -d '{
    "server": {
      "port": 8443,
      "max_connections": 1000
    }
  }'
```

#### Upload Certificate
```bash
curl -X POST http://127.0.0.1:8080/ui/api/certificates/upload \
  -F "certificate=@/path/to/certificate.crt"
```

#### Get Audit Logs
```bash
curl -X GET "http://127.0.0.1:8080/ui/api/audit/logs?limit=10&offset=0"
```

### JavaScript Examples

#### Health Check
```javascript
const response = await fetch('http://127.0.0.1:8080/health');
const health = await response.json();
console.log(health.status);
```

#### Update Configuration
```javascript
const config = {
  server: {
    port: 8443,
    max_connections: 1000
  }
};

const response = await fetch('http://127.0.0.1:8080/ui/api/config/update', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json'
  },
  body: JSON.stringify(config)
});

const result = await response.json();
console.log(result.status);
```

#### Upload Certificate
```javascript
const formData = new FormData();
formData.append('certificate', fileInput.files[0]);

const response = await fetch('http://127.0.0.1:8080/ui/api/certificates/upload', {
  method: 'POST',
  body: formData
});

const result = await response.json();
console.log(result.data.filename);
```

---

## Versioning

The API version is included in the health check response. Future versions will maintain backward compatibility where possible.

## Support

For API support and questions:
- Check the troubleshooting guide
- Review the error codes and messages
- Examine the audit logs for detailed error information
