# mTLS Proxy Server - Fedora Linux Deployment Plan

## 📋 **Overview**

This document outlines the deployment strategy for the mTLS Proxy Server on Fedora Linux, including:
- Package creation and distribution
- UI-based configuration management
- Certificate and key file management
- Installation and deployment procedures

---

## 🎯 **Deployment Requirements**

### **Target Environment**
- **OS**: Fedora Linux (latest LTS)
- **Architecture**: x86_64
- **Package Format**: RPM package
- **Service Management**: systemd
- **Web Server**: Built-in Warp server (no external dependencies)

### **UI Configuration Requirements**
- **Target URL Management**: Change proxy target URL
- **Certificate Upload**: Upload client certificate (.crt)
- **Key File Upload**: Upload client private key (.key)
- **CA Certificate Upload**: Upload CA certificate (optional)
- **Configuration Persistence**: Save settings to disk
- **Real-time Validation**: Validate configuration changes

---

## 📦 **Package Creation Strategy**

### **1. RPM Package Structure**
```
mtls-proxy-0.1.0-1.x86_64.rpm
├── /usr/bin/mtls-proxy                    # Binary executable
├── /usr/lib/systemd/system/mtls-proxy.service  # systemd service
├── /etc/mtls-proxy/                       # Configuration directory
│   ├── config.toml                        # Main configuration
│   ├── certs/                             # Certificate directory
│   │   ├── client.crt                     # Client certificate
│   │   ├── client.key                     # Client private key
│   │   └── ca.crt                         # CA certificate
│   └── logs/                              # Log directory
├── /var/lib/mtls-proxy/                   # Data directory
│   ├── proxy_logs.db                      # SQLite logs
│   └── uploads/                           # Uploaded files
├── /usr/share/mtls-proxy/                 # Static assets
│   ├── ui/                                # UI templates
│   └── static/                            # Static files
└── /usr/share/doc/mtls-proxy/             # Documentation
    ├── README.md
    ├── INSTALL.md
    └── CONFIGURATION.md
```

### **2. Package Dependencies**
```toml
# Cargo.toml dependencies for packaging
[dependencies]
# ... existing dependencies ...
rpm = "0.3"                    # RPM package creation
systemd = "0.10"               # systemd integration
multipart = "0.18"             # File upload handling
tempfile = "3.8"               # Temporary file handling
```

---

## 🔧 **UI Configuration Management**

### **1. Enhanced Configuration API**

#### **New API Endpoints**
```rust
// Configuration management endpoints
POST /ui/api/config/update          # Update configuration
GET  /ui/api/config/current         # Get current configuration
POST /ui/api/certificates/upload    # Upload certificate files
GET  /ui/api/certificates/list      # List uploaded certificates
DELETE /ui/api/certificates/{name}  # Delete certificate
POST /ui/api/config/validate        # Validate configuration
POST /ui/api/config/restart         # Restart service (if enabled)
```

#### **Configuration Update Handler**
```rust
#[derive(Deserialize)]
pub struct ConfigUpdateRequest {
    pub target_url: String,
    pub timeout_secs: u64,
    pub max_connections: usize,
    pub auth_enabled: bool,
    pub admin_username: String,
    pub admin_password: Option<String>, // Optional for updates
}

async fn update_config_handler(
    user: User,
    state: AppState,
    config_update: ConfigUpdateRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate configuration
    // Update configuration file
    // Reload configuration
    // Return success/error response
}
```

### **2. File Upload Management**

#### **Certificate Upload Handler**
```rust
#[derive(Deserialize)]
pub struct CertificateUpload {
    pub cert_type: CertificateType, // "client", "key", "ca"
    pub file_data: Vec<u8>,
    pub filename: String,
}

async fn upload_certificate_handler(
    user: User,
    state: AppState,
    upload: CertificateUpload,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Validate file format
    // Save to secure location
    // Update configuration
    // Return success/error response
}
```

### **3. Enhanced UI Components**

#### **Configuration Management Page**
```html
<!-- /ui/config.html -->
<div class="config-section">
    <h2>Target Configuration</h2>
    <form id="targetConfigForm">
        <label>Target URL:</label>
        <input type="url" id="targetUrl" required>
        
        <label>Timeout (seconds):</label>
        <input type="number" id="timeout" min="1" max="300">
        
        <label>Max Connections:</label>
        <input type="number" id="maxConnections" min="1" max="10000">
    </form>
</div>

<div class="config-section">
    <h2>Certificate Management</h2>
    <form id="certUploadForm" enctype="multipart/form-data">
        <label>Client Certificate:</label>
        <input type="file" id="clientCert" accept=".crt,.pem">
        
        <label>Client Private Key:</label>
        <input type="file" id="clientKey" accept=".key,.pem">
        
        <label>CA Certificate (optional):</label>
        <input type="file" id="caCert" accept=".crt,.pem">
        
        <button type="submit">Upload Certificates</button>
    </form>
</div>

<div class="config-section">
    <h2>Authentication</h2>
    <form id="authConfigForm">
        <label>Enable Authentication:</label>
        <input type="checkbox" id="authEnabled">
        
        <label>Admin Username:</label>
        <input type="text" id="adminUsername">
        
        <label>Admin Password:</label>
        <input type="password" id="adminPassword">
    </form>
</div>
```

---

## 🛠 **Implementation Plan**

### **Phase 1: Core Configuration Management**

#### **1.1 Enhanced Configuration System**
- [ ] **Add configuration persistence**
  - [ ] Implement configuration file writing
  - [ ] Add configuration validation
  - [ ] Add configuration reload capability
- [ ] **Add configuration API endpoints**
  - [ ] GET /ui/api/config/current
  - [ ] POST /ui/api/config/update
  - [ ] POST /ui/api/config/validate
- [ ] **Add configuration UI**
  - [ ] Create configuration management page
  - [ ] Add form validation
  - [ ] Add real-time feedback

#### **1.2 File Upload System**
- [ ] **Implement file upload handling**
  - [ ] Add multipart form handling
  - [ ] Add file validation (certificate/key formats)
  - [ ] Add secure file storage
- [ ] **Add certificate management API**
  - [ ] POST /ui/api/certificates/upload
  - [ ] GET /ui/api/certificates/list
  - [ ] DELETE /ui/api/certificates/{name}
- [ ] **Add certificate management UI**
  - [ ] Create file upload forms
  - [ ] Add file validation feedback
  - [ ] Add certificate status display

### **Phase 2: Package Creation**

#### **2.1 RPM Package Build System**
- [ ] **Create RPM spec file**
  - [ ] Define package structure
  - [ ] Define dependencies
  - [ ] Define installation scripts
- [ ] **Add build automation**
  - [ ] Create build script
  - [ ] Add CI/CD integration
  - [ ] Add version management
- [ ] **Add systemd integration**
  - [ ] Create systemd service file
  - [ ] Add service management commands
  - [ ] Add log rotation configuration

#### **2.2 Installation Scripts**
- [ ] **Create installation script**
  - [ ] Add dependency checking
  - [ ] Add user/group creation
  - [ ] Add directory setup
  - [ ] Add SELinux configuration
- [ ] **Create uninstallation script**
  - [ ] Add cleanup procedures
  - [ ] Add data preservation options
  - [ ] Add service removal

### **Phase 3: Security & Validation**

#### **3.1 Security Enhancements**
- [ ] **Add file upload security**
  - [ ] Validate file types
  - [ ] Scan for malicious content
  - [ ] Secure file permissions
- [ ] **Add configuration validation**
  - [ ] Validate URLs
  - [ ] Validate certificate chains
  - [ ] Validate file permissions
- [ ] **Add audit logging**
  - [ ] Log configuration changes
  - [ ] Log file uploads
  - [ ] Log authentication attempts

#### **3.2 Service Management**
- [ ] **Add service control**
  - [ ] Add restart capability
  - [ ] Add configuration reload
  - [ ] Add status monitoring
- [ ] **Add backup/restore**
  - [ ] Add configuration backup
  - [ ] Add certificate backup
  - [ ] Add restore procedures

---

## 📦 **Package Creation Process**

### **1. RPM Spec File**
```spec
# mtls-proxy.spec
Name:           mtls-proxy
Version:        0.1.0
Release:        1%{?dist}
Summary:        mTLS Proxy Server with Web UI

License:        MIT
URL:            https://github.com/your-org/mtls-proxy
Source0:        %{name}-%{version}.tar.gz

BuildArch:      x86_64
BuildRequires:  cargo
BuildRequires:  rust

Requires:       systemd
Requires:       openssl

%description
mTLS Proxy Server with web-based configuration management.
Supports certificate upload and real-time configuration updates.

%prep
%autosetup

%build
cargo build --release

%install
# Create directories
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/lib/systemd/system
mkdir -p %{buildroot}/etc/mtls-proxy/certs
mkdir -p %{buildroot}/etc/mtls-proxy/logs
mkdir -p %{buildroot}/var/lib/mtls-proxy
mkdir -p %{buildroot}/usr/share/mtls-proxy
mkdir -p %{buildroot}/usr/share/doc/mtls-proxy

# Install binary
install -m 755 target/release/mtls-proxy %{buildroot}/usr/bin/

# Install systemd service
install -m 644 scripts/mtls-proxy.service %{buildroot}/usr/lib/systemd/system/

# Install configuration
install -m 644 config/default.toml %{buildroot}/etc/mtls-proxy/config.toml

# Install documentation
install -m 644 README.md %{buildroot}/usr/share/doc/mtls-proxy/
install -m 644 INSTALL.md %{buildroot}/usr/share/doc/mtls-proxy/
install -m 644 CONFIGURATION.md %{buildroot}/usr/share/doc/mtls-proxy/

%files
%license LICENSE
%doc %{_docdir}/%{name}
/usr/bin/mtls-proxy
/usr/lib/systemd/system/mtls-proxy.service
/etc/mtls-proxy/config.toml
%dir /etc/mtls-proxy/certs
%dir /etc/mtls-proxy/logs
%dir /var/lib/mtls-proxy

%pre
# Create mtls-proxy user and group
getent group mtls-proxy >/dev/null || groupadd -r mtls-proxy
getent passwd mtls-proxy >/dev/null || useradd -r -g mtls-proxy -d /var/lib/mtls-proxy -s /sbin/nologin mtls-proxy

%post
# Enable and start service
systemctl daemon-reload
systemctl enable mtls-proxy.service

%preun
# Stop service before removal
if [ $1 -eq 0 ]; then
    systemctl stop mtls-proxy.service
fi

%postun
# Clean up if package is being removed
if [ $1 -eq 0 ]; then
    systemctl disable mtls-proxy.service
fi
```

### **2. Build Script**
```bash
#!/bin/bash
# build-package.sh

set -e

VERSION="0.1.0"
PACKAGE_NAME="mtls-proxy"

echo "Building $PACKAGE_NAME version $VERSION"

# Build the application
echo "Building application..."
cargo build --release

# Create package directory
echo "Creating package structure..."
mkdir -p package/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create source tarball
echo "Creating source tarball..."
tar -czf package/SOURCES/${PACKAGE_NAME}-${VERSION}.tar.gz \
    --exclude=target \
    --exclude=.git \
    --exclude=package \
    .

# Build RPM package
echo "Building RPM package..."
rpmbuild --define "_topdir $(pwd)/package" \
         --define "version $VERSION" \
         -ba mtls-proxy.spec

echo "Package built successfully!"
echo "RPM file: package/RPMS/x86_64/${PACKAGE_NAME}-${VERSION}-1.x86_64.rpm"
```

---

## 🚀 **Installation Procedures**

### **1. System Requirements**
```bash
# Minimum system requirements
- Fedora 35+ or RHEL 8+
- 2GB RAM
- 1GB disk space
- Network connectivity
- systemd support
```

### **2. Installation Steps**
```bash
# 1. Install dependencies
sudo dnf install -y openssl systemd

# 2. Install the package
sudo rpm -ivh mtls-proxy-0.1.0-1.x86_64.rpm

# 3. Configure certificates (via UI or manual)
sudo mkdir -p /etc/mtls-proxy/certs
sudo cp your-client.crt /etc/mtls-proxy/certs/client.crt
sudo cp your-client.key /etc/mtls-proxy/certs/client.key
sudo cp your-ca.crt /etc/mtls-proxy/certs/ca.crt

# 4. Set proper permissions
sudo chown -R mtls-proxy:mtls-proxy /etc/mtls-proxy/certs
sudo chmod 600 /etc/mtls-proxy/certs/client.key
sudo chmod 644 /etc/mtls-proxy/certs/client.crt
sudo chmod 644 /etc/mtls-proxy/certs/ca.crt

# 5. Start the service
sudo systemctl start mtls-proxy
sudo systemctl enable mtls-proxy

# 6. Verify installation
sudo systemctl status mtls-proxy
curl http://localhost:8080/health
```

### **3. Configuration via UI**
```bash
# 1. Access the web UI
# Open browser to: http://your-server:8080/ui

# 2. Login with default credentials
# Username: admin
# Password: admin123

# 3. Navigate to Configuration page
# Update target URL, upload certificates, configure authentication

# 4. Save configuration
# Configuration is automatically validated and applied
```

---

## 📚 **Documentation Requirements**

### **1. Installation Guide**
- [ ] **INSTALL.md**: Step-by-step installation instructions
- [ ] **System requirements**: Hardware and software requirements
- [ ] **Dependency installation**: Required packages
- [ ] **Certificate setup**: Certificate preparation and installation
- [ ] **Service configuration**: systemd service management
- [ ] **Troubleshooting**: Common installation issues

### **2. Configuration Guide**
- [ ] **CONFIGURATION.md**: Configuration options and examples
- [ ] **UI configuration**: Web-based configuration management
- [ ] **Certificate management**: Uploading and managing certificates
- [ ] **Authentication setup**: Configuring admin access
- [ ] **Security considerations**: Security best practices
- [ ] **Performance tuning**: Optimization recommendations

### **3. Administration Guide**
- [ ] **Service management**: Starting, stopping, restarting
- [ ] **Log management**: Log files and rotation
- [ ] **Monitoring**: Health checks and metrics
- [ ] **Backup procedures**: Configuration and certificate backup
- [ ] **Upgrade procedures**: Updating the proxy server
- [ ] **Troubleshooting**: Common issues and solutions

---

## 🔒 **Security Considerations**

### **1. File Upload Security**
- [ ] **File type validation**: Only allow .crt, .key, .pem files
- [ ] **Content validation**: Validate certificate/key content
- [ ] **Virus scanning**: Scan uploaded files for malware
- [ ] **Secure storage**: Store files with proper permissions
- [ ] **Access control**: Restrict file access to service user

### **2. Configuration Security**
- [ ] **Input validation**: Validate all configuration inputs
- [ ] **Path traversal protection**: Prevent directory traversal attacks
- [ ] **Authentication**: Require admin authentication for changes
- [ ] **Audit logging**: Log all configuration changes
- [ ] **Backup encryption**: Encrypt sensitive configuration backups

### **3. Service Security**
- [ ] **User isolation**: Run as dedicated service user
- [ ] **File permissions**: Restrict file access permissions
- [ ] **Network security**: Bind to localhost by default
- [ ] **SELinux integration**: Configure SELinux policies
- [ ] **Firewall configuration**: Configure firewall rules

---

## 🧪 **Testing Strategy**

### **1. Package Testing**
- [ ] **Installation testing**: Test package installation on clean system
- [ ] **Dependency testing**: Verify all dependencies are satisfied
- [ ] **Service testing**: Test systemd service management
- [ ] **Uninstallation testing**: Test clean package removal

### **2. Configuration Testing**
- [ ] **UI testing**: Test web-based configuration
- [ ] **File upload testing**: Test certificate upload functionality
- [ ] **Validation testing**: Test configuration validation
- [ ] **Persistence testing**: Test configuration persistence

### **3. Integration Testing**
- [ ] **End-to-end testing**: Test complete deployment workflow
- [ ] **Security testing**: Test security features and vulnerabilities
- [ ] **Performance testing**: Test under load conditions
- [ ] **Recovery testing**: Test failure and recovery scenarios

---

## 📋 **Implementation Checklist**

### **Phase 1: Core Features (Week 1-2)**
- [ ] **Enhanced configuration system**
  - [ ] Add configuration persistence
  - [ ] Add configuration validation
  - [ ] Add configuration API endpoints
- [ ] **File upload system**
  - [ ] Add multipart form handling
  - [ ] Add file validation
  - [ ] Add secure file storage
- [ ] **Enhanced UI**
  - [ ] Create configuration management page
  - [ ] Add file upload forms
  - [ ] Add validation feedback

### **Phase 2: Packaging (Week 3-4)**
- [ ] **RPM package creation**
  - [ ] Create RPM spec file
  - [ ] Add build automation
  - [ ] Add systemd integration
- [ ] **Installation scripts**
  - [ ] Create installation script
  - [ ] Create uninstallation script
  - [ ] Add dependency checking

### **Phase 3: Security & Testing (Week 5-6)**
- [ ] **Security enhancements**
  - [ ] Add file upload security
  - [ ] Add configuration validation
  - [ ] Add audit logging
- [ ] **Testing**
  - [ ] Package testing
  - [ ] Configuration testing
  - [ ] Integration testing

### **Phase 4: Documentation (Week 7-8)**
- [ ] **Documentation creation**
  - [ ] Installation guide
  - [ ] Configuration guide
  - [ ] Administration guide
- [ ] **Final testing**
  - [ ] End-to-end testing
  - [ ] Security testing
  - [ ] Performance testing

---

**Last Updated**: 2025-08-15
**Target Completion**: 8 weeks
**Priority**: High (Production deployment requirement)
