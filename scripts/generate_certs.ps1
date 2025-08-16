# Generate test certificates for mTLS proxy development (Windows PowerShell version)
# This script creates a self-signed CA and client/server certificates

param(
    [string]$CertDir = "certs"
)

Write-Host "=== Generating Test Certificates (Windows) ===" -ForegroundColor Green

# Get the script directory
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$CertDir = Join-Path $ScriptDir ".." $CertDir

Write-Host "Certificate directory: $CertDir"

# Create certs directory if it doesn't exist
if (!(Test-Path -Path $CertDir)) {
    New-Item -ItemType Directory -Path $CertDir -Force | Out-Null
    Write-Host "Created certificate directory: $CertDir"
}

# Check if OpenSSL is available
try {
    $opensslVersion = openssl version 2>$null
    if ($LASTEXITCODE -eq 0) {
        Write-Host "OpenSSL found: $opensslVersion" -ForegroundColor Green
    } else {
        throw "OpenSSL not found"
    }
} catch {
    Write-Host "OpenSSL not available. Please install OpenSSL for Windows or use WSL." -ForegroundColor Yellow
    Write-Host "Creating placeholder certificate files for testing..." -ForegroundColor Yellow
    
    # Create placeholder files
    $placeholderContent = "# Placeholder certificate file - replace with actual certificate"
    New-Item -ItemType File -Path (Join-Path $CertDir "ca.crt") -Force | Out-Null
    Set-Content -Path (Join-Path $CertDir "ca.crt") -Value $placeholderContent
    New-Item -ItemType File -Path (Join-Path $CertDir "ca.key") -Force | Out-Null
    Set-Content -Path (Join-Path $CertDir "ca.key") -Value $placeholderContent
    New-Item -ItemType File -Path (Join-Path $CertDir "server.crt") -Force | Out-Null
    Set-Content -Path (Join-Path $CertDir "server.crt") -Value $placeholderContent
    New-Item -ItemType File -Path (Join-Path $CertDir "server.key") -Force | Out-Null
    Set-Content -Path (Join-Path $CertDir "server.key") -Value $placeholderContent
    New-Item -ItemType File -Path (Join-Path $CertDir "client.crt") -Force | Out-Null
    Set-Content -Path (Join-Path $CertDir "client.crt") -Value $placeholderContent
    New-Item -ItemType File -Path (Join-Path $CertDir "client.key") -Force | Out-Null
    Set-Content -Path (Join-Path $CertDir "client.key") -Value $placeholderContent
    
    Write-Host "Placeholder files created. Replace with actual certificates for production use." -ForegroundColor Yellow
    exit 0
}

# Generate CA private key and certificate
Write-Host "Generating CA private key and certificate..." -ForegroundColor Cyan
openssl genrsa -out (Join-Path $CertDir "ca.key") 2048
if ($LASTEXITCODE -ne 0) { throw "Failed to generate CA private key" }

openssl req -new -x509 -days 365 -key (Join-Path $CertDir "ca.key") -out (Join-Path $CertDir "ca.crt") -subj "/C=US/ST=CA/L=San Francisco/O=Test CA/CN=test-ca.local"
if ($LASTEXITCODE -ne 0) { throw "Failed to generate CA certificate" }

# Generate server private key and certificate signing request
Write-Host "Generating server private key and CSR..." -ForegroundColor Cyan
openssl genrsa -out (Join-Path $CertDir "server.key") 2048
if ($LASTEXITCODE -ne 0) { throw "Failed to generate server private key" }

openssl req -new -key (Join-Path $CertDir "server.key") -out (Join-Path $CertDir "server.csr") -subj "/C=US/ST=CA/L=San Francisco/O=Test Server/CN=localhost"
if ($LASTEXITCODE -ne 0) { throw "Failed to generate server CSR" }

# Sign server certificate with CA
Write-Host "Signing server certificate..." -ForegroundColor Cyan
openssl x509 -req -days 365 -in (Join-Path $CertDir "server.csr") -CA (Join-Path $CertDir "ca.crt") -CAkey (Join-Path $CertDir "ca.key") -CAcreateserial -out (Join-Path $CertDir "server.crt")
if ($LASTEXITCODE -ne 0) { throw "Failed to sign server certificate" }

# Generate client private key and certificate signing request
Write-Host "Generating client private key and CSR..." -ForegroundColor Cyan
openssl genrsa -out (Join-Path $CertDir "client.key") 2048
if ($LASTEXITCODE -ne 0) { throw "Failed to generate client private key" }

openssl req -new -key (Join-Path $CertDir "client.key") -out (Join-Path $CertDir "client.csr") -subj "/C=US/ST=CA/L=San Francisco/O=Test Client/CN=test-client.local"
if ($LASTEXITCODE -ne 0) { throw "Failed to generate client CSR" }

# Sign client certificate with CA
Write-Host "Signing client certificate..." -ForegroundColor Cyan
openssl x509 -req -days 365 -in (Join-Path $CertDir "client.csr") -CA (Join-Path $CertDir "ca.crt") -CAkey (Join-Path $CertDir "ca.key") -CAcreateserial -out (Join-Path $CertDir "client.crt")
if ($LASTEXITCODE -ne 0) { throw "Failed to sign client certificate" }

# Set proper permissions (Windows equivalent)
Write-Host "Setting file permissions..." -ForegroundColor Cyan
# On Windows, we can't use chmod, but we can set file attributes
# Private keys should be readable only by the owner
Get-ChildItem -Path $CertDir -Filter "*.key" | ForEach-Object {
    $_.Attributes = $_.Attributes -bor [System.IO.FileAttributes]::ReadOnly
}

# Clean up CSR files
Write-Host "Cleaning up temporary files..." -ForegroundColor Cyan
Get-ChildItem -Path $CertDir -Filter "*.csr" | Remove-Item -Force

Write-Host "=== Certificate Generation Complete ===" -ForegroundColor Green
Write-Host "Generated certificates in: $CertDir" -ForegroundColor Green
Write-Host ""
Write-Host "Files created:" -ForegroundColor Yellow
Write-Host "  - ca.crt: CA certificate" -ForegroundColor White
Write-Host "  - ca.key: CA private key" -ForegroundColor White
Write-Host "  - server.crt: Server certificate" -ForegroundColor White
Write-Host "  - server.key: Server private key" -ForegroundColor White
Write-Host "  - client.crt: Client certificate" -ForegroundColor White
Write-Host "  - client.key: Client private key" -ForegroundColor White
Write-Host ""
Write-Host "Note: These are test certificates for development only!" -ForegroundColor Red
Write-Host "Do not use in production environments." -ForegroundColor Red
