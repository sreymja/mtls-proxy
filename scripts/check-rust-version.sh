#!/bin/bash

# Check Rust version compatibility and fix lock file issues
# This script helps diagnose and fix Cargo lock file version problems

set -e

echo "=== Rust Version Compatibility Check ==="

# Check current Rust version
echo "Current Rust version:"
rustc --version
cargo --version

# Check if we need to update
echo ""
echo "Checking for updates..."
rustup update

# Check lock file version
if [ -f "Cargo.lock" ]; then
    echo ""
    echo "Checking Cargo.lock compatibility..."
    
    # Try to update dependencies
    if cargo update; then
        echo "✅ Cargo.lock is compatible"
    else
        echo "❌ Cargo.lock version issue detected"
        echo "Regenerating lock file..."
        rm -f Cargo.lock
        cargo generate-lockfile
        echo "✅ Lock file regenerated"
    fi
else
    echo "No Cargo.lock found, generating..."
    cargo generate-lockfile
fi

# Verify everything works
echo ""
echo "Verifying build..."
cargo check

echo ""
echo "✅ Rust environment is ready!"
