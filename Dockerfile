# Multi-stage build for mTLS Proxy
# Stage 1: Build stage
FROM rust:1.89-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release

# Remove dummy main.rs and copy actual source code
RUN rm src/main.rs
COPY src/ ./src/
COPY config/ ./config/

# Build the application
RUN cargo build --release

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r mtls-proxy && useradd -r -g mtls-proxy mtls-proxy

# Create necessary directories
RUN mkdir -p /etc/mtls-proxy/certs \
    /var/log/mtls-proxy \
    /var/lib/mtls-proxy \
    && chown -R mtls-proxy:mtls-proxy /etc/mtls-proxy \
    /var/log/mtls-proxy \
    /var/lib/mtls-proxy

# Copy binary from builder stage
COPY --from=builder /app/target/release/mtls-proxy /usr/local/bin/mtls-proxy

# Copy configuration files
COPY --from=builder /app/config/default.toml /etc/mtls-proxy/default.toml

# Set proper permissions
RUN chmod +x /usr/local/bin/mtls-proxy \
    && chmod 644 /etc/mtls-proxy/default.toml \
    && chmod 700 /etc/mtls-proxy/certs

# Switch to non-root user
USER mtls-proxy

# Expose ports
EXPOSE 8443 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["/usr/local/bin/mtls-proxy", "--config", "/etc/mtls-proxy/default.toml"]

# Labels
LABEL maintainer="mTLS Proxy Team <support@your-org.com>"
LABEL description="High-performance mTLS proxy server with web interface"
LABEL version="0.1.0"
LABEL org.opencontainers.image.source="https://github.com/your-org/mtls-proxy"
