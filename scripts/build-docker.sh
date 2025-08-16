#!/bin/bash

# Build Docker image for mTLS proxy
# Usage: ./scripts/build-docker.sh [version] [tag]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
VERSION=${1:-0.1.0}
TAG=${2:-latest}
IMAGE_NAME="mtls-proxy"
REGISTRY=${DOCKER_REGISTRY:-"your-registry.com"}
FULL_IMAGE_NAME="${REGISTRY}/${IMAGE_NAME}:${TAG}"

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check dependencies
check_dependencies() {
    print_status "Checking Docker dependencies..."
    
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed or not in PATH"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        print_error "Docker daemon is not running or user is not in docker group"
        exit 1
    fi
    
    print_status "Docker dependencies satisfied"
}

# Function to build Docker image
build_image() {
    print_status "Building Docker image: ${FULL_IMAGE_NAME}"
    
    # Build the image
    docker build \
        --build-arg VERSION=${VERSION} \
        --tag ${FULL_IMAGE_NAME} \
        --file Dockerfile \
        .
    
    print_status "Docker image built successfully"
}

# Function to scan image for vulnerabilities
scan_image() {
    print_status "Scanning Docker image for vulnerabilities..."
    
    # Check if trivy is available
    if command -v trivy &> /dev/null; then
        print_status "Running Trivy vulnerability scan..."
        trivy image --severity HIGH,CRITICAL ${FULL_IMAGE_NAME} || {
            print_warning "Vulnerability scan found issues (see output above)"
        }
    else
        print_warning "Trivy not found, skipping vulnerability scan"
        print_status "Install Trivy: https://aquasecurity.github.io/trivy/latest/getting-started/installation/"
    fi
    
    # Check if hadolint is available for Dockerfile linting
    if command -v hadolint &> /dev/null; then
        print_status "Running Hadolint Dockerfile linting..."
        hadolint Dockerfile || {
            print_warning "Dockerfile linting found issues (see output above)"
        }
    else
        print_warning "Hadolint not found, skipping Dockerfile linting"
        print_status "Install Hadolint: https://github.com/hadolint/hadolint#install"
    fi
}

# Function to test Docker image
test_image() {
    print_status "Testing Docker image..."
    
    # Create a temporary container for testing
    local test_container="mtls-proxy-test-$$"
    
    # Start container in background
    docker run -d \
        --name ${test_container} \
        --publish 8080:8080 \
        --publish 8443:8443 \
        ${FULL_IMAGE_NAME}
    
    # Wait for container to start
    sleep 10
    
    # Test health endpoint
    if curl -f http://localhost:8080/health &> /dev/null; then
        print_status "Health check passed"
    else
        print_error "Health check failed"
        docker logs ${test_container}
        docker stop ${test_container} &> /dev/null
        docker rm ${test_container} &> /dev/null
        exit 1
    fi
    
    # Test web interface
    if curl -f http://localhost:8080/ui &> /dev/null; then
        print_status "Web interface test passed"
    else
        print_warning "Web interface test failed"
    fi
    
    # Clean up test container
    docker stop ${test_container} &> /dev/null
    docker rm ${test_container} &> /dev/null
    
    print_status "Docker image testing completed"
}

# Function to tag image with version
tag_version() {
    local version_tag="${REGISTRY}/${IMAGE_NAME}:v${VERSION}"
    
    print_status "Tagging image with version: ${version_tag}"
    docker tag ${FULL_IMAGE_NAME} ${version_tag}
    
    print_status "Available tags:"
    docker images ${REGISTRY}/${IMAGE_NAME} --format "table {{.Tag}}\t{{.Size}}\t{{.CreatedAt}}"
}

# Function to save image to tar file
save_image() {
    local tar_file="build/${IMAGE_NAME}-${VERSION}.tar"
    
    print_status "Saving Docker image to: ${tar_file}"
    
    mkdir -p build
    docker save ${FULL_IMAGE_NAME} -o ${tar_file}
    
    # Compress the tar file
    gzip ${tar_file}
    
    print_status "Image saved to: ${tar_file}.gz"
    print_status "Size: $(du -h ${tar_file}.gz | cut -f1)"
}

# Function to push image to registry
push_image() {
    if [ -z "$DOCKER_REGISTRY" ]; then
        print_warning "DOCKER_REGISTRY not set, skipping push"
        return
    fi
    
    print_status "Pushing Docker image to registry..."
    
    # Push both tags
    docker push ${FULL_IMAGE_NAME}
    docker push "${REGISTRY}/${IMAGE_NAME}:v${VERSION}"
    
    print_status "Docker image pushed successfully"
}

# Function to create deployment files
create_deployment_files() {
    print_status "Creating deployment files..."
    
    mkdir -p build
    
    # Create Kubernetes deployment
    cat > build/k8s-deployment.yaml << EOF
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mtls-proxy
  labels:
    app: mtls-proxy
spec:
  replicas: 1
  selector:
    matchLabels:
      app: mtls-proxy
  template:
    metadata:
      labels:
        app: mtls-proxy
    spec:
      containers:
      - name: mtls-proxy
        image: ${FULL_IMAGE_NAME}
        ports:
        - containerPort: 8443
          name: proxy
        - containerPort: 8080
          name: web
        env:
        - name: RUST_LOG
          value: "info"
        volumeMounts:
        - name: config
          mountPath: /etc/mtls-proxy
          readOnly: true
        - name: certs
          mountPath: /etc/mtls-proxy/certs
        - name: logs
          mountPath: /var/log/mtls-proxy
        - name: data
          mountPath: /var/lib/mtls-proxy
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
        resources:
          limits:
            memory: "512Mi"
            cpu: "1000m"
          requests:
            memory: "256Mi"
            cpu: "500m"
      volumes:
      - name: config
        configMap:
          name: mtls-proxy-config
      - name: certs
        secret:
          secretName: mtls-proxy-certs
      - name: logs
        emptyDir: {}
      - name: data
        persistentVolumeClaim:
          claimName: mtls-proxy-data
---
apiVersion: v1
kind: Service
metadata:
  name: mtls-proxy-service
spec:
  selector:
    app: mtls-proxy
  ports:
  - name: proxy
    port: 8443
    targetPort: 8443
  - name: web
    port: 8080
    targetPort: 8080
  type: ClusterIP
EOF
    
    # Create Docker Compose override for production
    cat > build/docker-compose.prod.yml << EOF
version: '3.8'

services:
  mtls-proxy:
    image: ${FULL_IMAGE_NAME}
    restart: unless-stopped
    ports:
      - "8443:8443"
      - "8080:8080"
    volumes:
      - ./config:/etc/mtls-proxy:ro
      - ./certs:/etc/mtls-proxy/certs
      - ./logs:/var/log/mtls-proxy
      - ./data:/var/lib/mtls-proxy
    environment:
      - RUST_LOG=info
    deploy:
      resources:
        limits:
          memory: 512M
          cpus: '1.0'
        reservations:
          memory: 256M
          cpus: '0.5'
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
EOF
    
    print_status "Deployment files created in build/ directory"
}

# Function to create build summary
create_build_summary() {
    local summary_file="build/DOCKER_BUILD_SUMMARY.md"
    
    cat > "$summary_file" << EOF
# mTLS Proxy Docker Build Summary

**Build Date:** $(date)
**Version:** ${VERSION}
**Tag:** ${TAG}
**Image:** ${FULL_IMAGE_NAME}

## Generated Files

### Docker Image
- **Image Name:** ${FULL_IMAGE_NAME}
- **Version Tag:** ${REGISTRY}/${IMAGE_NAME}:v${VERSION}
- **Size:** $(docker images ${FULL_IMAGE_NAME}:${TAG} --format "{{.Size}}")

### Archive Files
- **Tar Archive:** build/${IMAGE_NAME}-${VERSION}.tar.gz

### Deployment Files
- **Kubernetes:** build/k8s-deployment.yaml
- **Docker Compose:** build/docker-compose.prod.yml

## Usage

### Run with Docker
\`\`\`bash
# Run the container
docker run -d \\
  --name mtls-proxy \\
  --publish 8443:8443 \\
  --publish 8080:8080 \\
  --volume \$(pwd)/config:/etc/mtls-proxy:ro \\
  --volume \$(pwd)/certs:/etc/mtls-proxy/certs \\
  --volume \$(pwd)/logs:/var/log/mtls-proxy \\
  --volume \$(pwd)/data:/var/lib/mtls-proxy \\
  ${FULL_IMAGE_NAME}
\`\`\`

### Run with Docker Compose
\`\`\`bash
# Use production compose file
docker-compose -f build/docker-compose.prod.yml up -d
\`\`\`

### Deploy to Kubernetes
\`\`\`bash
# Apply Kubernetes deployment
kubectl apply -f build/k8s-deployment.yaml
\`\`\`

## Configuration

- **Config Directory:** /etc/mtls-proxy/
- **Certificate Directory:** /etc/mtls-proxy/certs/
- **Log Directory:** /var/log/mtls-proxy/
- **Data Directory:** /var/lib/mtls-proxy/

## Health Check

The container includes a health check that verifies the service is running:
\`\`\`bash
curl http://localhost:8080/health
\`\`\`

## Security

- Runs as non-root user (mtls-proxy)
- Uses multi-stage build to minimize attack surface
- Includes security scanning with Trivy
- Implements proper file permissions
EOF
    
    print_status "Build summary created: ${summary_file}"
}

# Main build process
main() {
    print_status "Starting Docker build for ${IMAGE_NAME} version ${VERSION}"
    
    # Check dependencies
    check_dependencies
    
    # Build image
    build_image
    
    # Scan image
    scan_image
    
    # Test image
    test_image
    
    # Tag image
    tag_version
    
    # Save image
    save_image
    
    # Create deployment files
    create_deployment_files
    
    # Create build summary
    create_build_summary
    
    # Push image (if registry is configured)
    push_image
    
    print_status "Docker build completed successfully!"
    print_status "Build artifacts are in: build/ directory"
    
    # Show final summary
    echo
    print_status "Build Summary:"
    echo "  Docker Image: ${FULL_IMAGE_NAME}"
    echo "  Version Tag: ${REGISTRY}/${IMAGE_NAME}:v${VERSION}"
    echo "  Archive: build/${IMAGE_NAME}-${VERSION}.tar.gz"
    echo "  Kubernetes: build/k8s-deployment.yaml"
    echo "  Docker Compose: build/docker-compose.prod.yml"
    echo "  Summary: build/DOCKER_BUILD_SUMMARY.md"
}

# Run main function
main "$@"
