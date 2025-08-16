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

# Check for dependency version conflicts
echo ""
echo "Checking for dependency version conflicts..."
if cargo check; then
    echo "✅ All dependencies are compatible"
else
    echo "❌ Dependency version conflict detected"
    echo "Attempting to resolve with compatible versions..."
    
    # Try aggressive update first
    cargo update --aggressive
    if cargo check; then
        echo "✅ Dependencies resolved successfully"
    else
        echo "Trying to pin problematic dependencies..."
        
        # Try pinning url to a compatible version
        cargo update url --precise 2.4.1 || true
        
        if cargo check; then
            echo "✅ Dependencies resolved by pinning url to 2.4.1"
        else
            echo "❌ Could not resolve dependency conflicts automatically"
            echo "You may need to manually update dependencies or Rust version"
            exit 1
        fi
    fi
fi

# Verify everything works
echo ""
echo "Verifying build..."
cargo check

echo ""
echo "✅ Rust environment is ready!"
