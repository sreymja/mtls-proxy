#!/bin/bash

# Build RPM package for mTLS proxy
# Usage: ./scripts/build-rpm.sh [version] [release]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
VERSION=${1:-0.1.0}
RELEASE=${2:-1}
PACKAGE_NAME="mtls-proxy"
BUILD_DIR="build"
RPM_BUILD_DIR="${BUILD_DIR}/rpmbuild"

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
    print_status "Checking build dependencies..."
    
    local missing_deps=()
    
    # Check for required tools
    for tool in rpmbuild rpm rpmbuild-spec; do
        if ! command -v $tool &> /dev/null; then
            missing_deps+=($tool)
        fi
    done
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        missing_deps+=("rust")
    fi
    
    # Check for pkg-config
    if ! command -v pkg-config &> /dev/null; then
        missing_deps+=("pkg-config")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing dependencies: ${missing_deps[*]}"
        print_status "Please install the missing dependencies:"
        echo "  sudo yum install rpm-build rpmdevtools rust pkg-config openssl-devel"
        exit 1
    fi
    
    print_status "All dependencies satisfied"
}

# Function to clean build directory
clean_build() {
    print_status "Cleaning build directory..."
    rm -rf "${BUILD_DIR}"
    mkdir -p "${BUILD_DIR}"
}

# Function to prepare source tarball
prepare_source() {
    print_status "Preparing source tarball..."
    
    local tarball_name="${PACKAGE_NAME}-${VERSION}.tar.gz"
    local tarball_path="${BUILD_DIR}/${tarball_name}"
    
    # Create source tarball
    tar --exclude='.git' \
        --exclude='target' \
        --exclude='logs' \
        --exclude='*.db' \
        --exclude='*.db-*' \
        --exclude='build' \
        --exclude='node_modules' \
        --exclude='*.log' \
        -czf "${tarball_path}" .
    
    print_status "Source tarball created: ${tarball_path}"
}

# Function to setup RPM build environment
setup_rpm_build() {
    print_status "Setting up RPM build environment..."
    
    # Create RPM build directory structure
    mkdir -p "${RPM_BUILD_DIR}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    
    # Copy source tarball to SOURCES
    cp "${BUILD_DIR}/${PACKAGE_NAME}-${VERSION}.tar.gz" "${RPM_BUILD_DIR}/SOURCES/"
    
    # Copy spec file to SPECS
    cp "packaging/${PACKAGE_NAME}.spec" "${RPM_BUILD_DIR}/SPECS/"
    
    # Update spec file with correct version and release
    sed -i "s/^Version:.*/Version: ${VERSION}/" "${RPM_BUILD_DIR}/SPECS/${PACKAGE_NAME}.spec"
    sed -i "s/^Release:.*/Release: ${RELEASE}%{?dist}/" "${RPM_BUILD_DIR}/SPECS/${PACKAGE_NAME}.spec"
    
    print_status "RPM build environment setup complete"
}

# Function to build RPM package
build_rpm() {
    print_status "Building RPM package..."
    
    # Build the Rust binary first
    print_status "Building Rust binary..."
    cargo build --release
    
    # Build RPM package
    rpmbuild --define "_topdir ${PWD}/${RPM_BUILD_DIR}" \
             --define "_builddir ${PWD}/${RPM_BUILD_DIR}/BUILD" \
             --define "_rpmdir ${PWD}/${RPM_BUILD_DIR}/RPMS" \
             --define "_sourcedir ${PWD}/${RPM_BUILD_DIR}/SOURCES" \
             --define "_specdir ${PWD}/${RPM_BUILD_DIR}/SPECS" \
             --define "_srcrpmdir ${PWD}/${RPM_BUILD_DIR}/SRPMS" \
             -ba "${RPM_BUILD_DIR}/SPECS/${PACKAGE_NAME}.spec"
    
    print_status "RPM package build complete"
}

# Function to verify RPM package
verify_rpm() {
    print_status "Verifying RPM package..."
    
    local rpm_file=$(find "${RPM_BUILD_DIR}/RPMS" -name "*.rpm" | head -1)
    
    if [ -z "$rpm_file" ]; then
        print_error "No RPM package found"
        exit 1
    fi
    
    print_status "RPM package created: ${rpm_file}"
    
    # Verify RPM package
    rpm -K "$rpm_file"
    
    # Show package info
    print_status "Package information:"
    rpm -qip "$rpm_file"
    
    # Show package contents
    print_status "Package contents:"
    rpm -qlp "$rpm_file" | head -20
    
    print_status "RPM package verification complete"
}

# Function to create installation script
create_install_script() {
    print_status "Creating installation script..."
    
    local install_script="${BUILD_DIR}/install.sh"
    local rpm_file=$(find "${RPM_BUILD_DIR}/RPMS" -name "*.rpm" | head -1)
    
    cat > "$install_script" << EOF
#!/bin/bash

# mTLS Proxy Installation Script
# Generated on $(date)

set -e

echo "Installing mTLS Proxy..."

# Install RPM package
sudo rpm -ivh "$(basename "$rpm_file")"

# Create certificate directory if it doesn't exist
sudo mkdir -p /etc/mtls-proxy/certs

# Set proper permissions
sudo chown -R mtls-proxy:mtls-proxy /var/log/mtls-proxy
sudo chown -R mtls-proxy:mtls-proxy /var/lib/mtls-proxy
sudo chown -R mtls-proxy:mtls-proxy /etc/mtls-proxy/certs

# Enable and start service
sudo systemctl daemon-reload
sudo systemctl enable mtls-proxy
sudo systemctl start mtls-proxy

echo "Installation complete!"
echo "Service status: sudo systemctl status mtls-proxy"
echo "Web interface: http://127.0.0.1:8080/ui"
echo "Logs: sudo journalctl -u mtls-proxy"
EOF
    
    chmod +x "$install_script"
    print_status "Installation script created: ${install_script}"
}

# Function to create build summary
create_build_summary() {
    print_status "Creating build summary..."
    
    local summary_file="${BUILD_DIR}/BUILD_SUMMARY.md"
    local rpm_file=$(find "${RPM_BUILD_DIR}/RPMS" -name "*.rpm" | head -1)
    local srpm_file=$(find "${RPM_BUILD_DIR}/SRPMS" -name "*.rpm" | head -1)
    
    cat > "$summary_file" << EOF
# mTLS Proxy Build Summary

**Build Date:** $(date)
**Version:** ${VERSION}
**Release:** ${RELEASE}

## Generated Files

### RPM Package
- **File:** $(basename "$rpm_file")
- **Size:** $(du -h "$rpm_file" | cut -f1)
- **Architecture:** $(rpm -qip "$rpm_file" | grep Architecture | awk '{print $2}')

### Source RPM
- **File:** $(basename "$srpm_file")
- **Size:** $(du -h "$srpm_file" | cut -f1)

### Installation Script
- **File:** install.sh
- **Usage:** ./install.sh

## Installation

\`\`\`bash
# Install the RPM package
sudo rpm -ivh $(basename "$rpm_file")

# Or use the installation script
./install.sh
\`\`\`

## Service Management

\`\`\`bash
# Start the service
sudo systemctl start mtls-proxy

# Enable at boot
sudo systemctl enable mtls-proxy

# Check status
sudo systemctl status mtls-proxy

# View logs
sudo journalctl -u mtls-proxy
\`\`\`

## Configuration

- **Config Directory:** /etc/mtls-proxy/
- **Log Directory:** /var/log/mtls-proxy/
- **Data Directory:** /var/lib/mtls-proxy/
- **Certificate Directory:** /etc/mtls-proxy/certs/

## Web Interface

Access the web interface at: http://127.0.0.1:8080/ui

## Documentation

- **Man Page:** man mtls-proxy
- **Documentation:** /usr/share/doc/mtls-proxy/
EOF
    
    print_status "Build summary created: ${summary_file}"
}

# Main build process
main() {
    print_status "Starting RPM build for ${PACKAGE_NAME} version ${VERSION}-${RELEASE}"
    
    # Check dependencies
    check_dependencies
    
    # Clean and prepare
    clean_build
    prepare_source
    setup_rpm_build
    
    # Build package
    build_rpm
    
    # Verify and create artifacts
    verify_rpm
    create_install_script
    create_build_summary
    
    print_status "RPM build completed successfully!"
    print_status "Build artifacts are in: ${BUILD_DIR}/"
    
    # Show final summary
    echo
    print_status "Build Summary:"
    echo "  RPM Package: $(find "${RPM_BUILD_DIR}/RPMS" -name "*.rpm" | head -1)"
    echo "  Source RPM: $(find "${RPM_BUILD_DIR}/SRPMS" -name "*.rpm" | head -1)"
    echo "  Installation Script: ${BUILD_DIR}/install.sh"
    echo "  Build Summary: ${BUILD_DIR}/BUILD_SUMMARY.md"
}

# Run main function
main "$@"
