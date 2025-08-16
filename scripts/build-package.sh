#!/bin/bash

set -e

# Configuration
VERSION="0.1.0"
PACKAGE_NAME="mtls-proxy"
BUILD_DIR="package"
SPEC_FILE="mtls-proxy.spec"

echo "Building $PACKAGE_NAME version $VERSION"

# Check dependencies
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo is required but not installed"
    exit 1
fi

if ! command -v rpmbuild &> /dev/null; then
    echo "Error: rpmbuild is required but not installed"
    echo "Install with: sudo dnf install rpm-build"
    exit 1
fi

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf $BUILD_DIR
rm -rf target/release

# Build the application
echo "Building application..."
cargo build --release

# Create package directory structure
echo "Creating package structure..."
mkdir -p $BUILD_DIR/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create source tarball
echo "Creating source tarball..."
tar -czf $BUILD_DIR/SOURCES/${PACKAGE_NAME}-${VERSION}.tar.gz \
    --exclude=target \
    --exclude=.git \
    --exclude=$BUILD_DIR \
    --exclude=*.rpm \
    .

# Copy spec file
cp $SPEC_FILE $BUILD_DIR/SPECS/

# Build RPM package
echo "Building RPM package..."
rpmbuild --define "_topdir $(pwd)/$BUILD_DIR" \
         --define "version $VERSION" \
         --define "release 1" \
         -ba $BUILD_DIR/SPECS/$SPEC_FILE

# Find the built RPM
RPM_FILE=$(find $BUILD_DIR/RPMS -name "*.rpm" | head -1)

if [ -n "$RPM_FILE" ]; then
    echo "Package built successfully!"
    echo "RPM file: $RPM_FILE"
    echo "Package size: $(du -h $RPM_FILE | cut -f1)"
    
    # Show package info
    echo ""
    echo "Package information:"
    rpm -qip $RPM_FILE
    
    # Copy to current directory for convenience
    cp $RPM_FILE .
    echo "Copied to: $(basename $RPM_FILE)"
else
    echo "Error: RPM package not found"
    exit 1
fi
