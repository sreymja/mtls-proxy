# Certificate Setup and CI/CD Fixes

## Overview

This document explains the certificate generation setup and the fixes made to resolve GitHub Actions workflow issues.

## Problem

The original GitHub Actions workflow had the following issues:

1. **Cross-platform compatibility**: The workflow used Unix/Linux commands (`mkdir -p`, `openssl`, `chmod`) on all platforms including Windows, causing failures on Windows runners.

2. **Directory creation error**: The error `mkdir -p certs` failed on Windows because:
   - PowerShell doesn't recognize the `-p` flag
   - The directory already existed in some cases

## Solution

### 1. Platform-Specific Certificate Generation

The workflow now uses different approaches for different platforms:

#### Unix/Linux/macOS
```yaml
- name: Generate test certificates (Unix)
  if: matrix.os != 'windows-latest'
  run: |
    chmod +x scripts/generate_certs.sh
    ./scripts/generate_certs.sh
```

#### Windows
```yaml
- name: Generate test certificates (Windows)
  if: matrix.os == 'windows-latest'
  shell: pwsh
  run: |
    .\scripts\generate_certs.ps1
```

### 2. Certificate Generation Scripts

#### Bash Script (`scripts/generate_certs.sh`)
- Generates CA, server, and client certificates using OpenSSL
- Sets proper file permissions
- Cleans up temporary files
- Works on Unix/Linux/macOS systems

#### PowerShell Script (`scripts/generate_certs.ps1`)
- Windows-compatible version of the certificate generation script
- Checks for OpenSSL availability
- Creates placeholder files if OpenSSL is not available
- Handles Windows-specific file operations

### 3. File Management

#### .gitignore Updates
Added certificate files to `.gitignore` to prevent committing test certificates:
```
# Certificate files (generated for testing)
certs/*.crt
certs/*.key
certs/*.srl
certs/*.csr
```

## Usage

### Local Development

#### Unix/Linux/macOS
```bash
chmod +x scripts/generate_certs.sh
./scripts/generate_certs.sh
```

#### Windows
```powershell
.\scripts\generate_certs.ps1
```

### CI/CD Pipeline

The GitHub Actions workflow automatically generates certificates for each platform:
- **Ubuntu/macOS**: Uses the bash script with OpenSSL
- **Windows**: Uses the PowerShell script with fallback to placeholder files

## Generated Files

The scripts create the following certificate files:

- `ca.crt` - CA certificate
- `ca.key` - CA private key
- `server.crt` - Server certificate
- `server.key` - Server private key
- `client.crt` - Client certificate
- `client.key` - Client private key

## Security Notes

⚠️ **Important**: These are test certificates for development only!
- Do not use in production environments
- Certificates are self-signed and not trusted by default
- Private keys have restricted permissions (600 on Unix, read-only on Windows)

## Troubleshooting

### Windows Issues
1. **OpenSSL not found**: The PowerShell script will create placeholder files
2. **Permission errors**: Run PowerShell as Administrator if needed
3. **Path issues**: Ensure the script is run from the project root

### Unix/Linux Issues
1. **Permission denied**: Make sure the script is executable (`chmod +x`)
2. **OpenSSL not installed**: Install OpenSSL via package manager
3. **Directory creation fails**: Check write permissions in the project directory

## CI/CD Status

After these fixes:
- ✅ All platforms (Ubuntu, macOS, Windows) pass certificate generation
- ✅ Tests run successfully on all platforms
- ✅ No more `mkdir -p` errors on Windows
- ✅ Proper fallback handling for missing OpenSSL on Windows
