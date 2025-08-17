# mTLS Proxy Server - Installation Guide

## 📋 **System Requirements**

### **Minimum Requirements**
- **Operating System**: Fedora 35+ or RHEL 8+
- **Architecture**: x86_64
- **Memory**: 2GB RAM
- **Disk Space**: 1GB available space
- **Network**: Internet connectivity for installation
- **Privileges**: Root access for package installation

### **Recommended Requirements**
- **Operating System**: Fedora 38+ or RHEL 9+
- **Memory**: 4GB RAM
- **Disk Space**: 5GB available space
- **Network**: Stable internet connection

---

## 🚀 **Quick Installation**

### **1. Install Dependencies**
```bash
# Update system packages
sudo dnf update -y

# Install required dependencies
sudo dnf install -y openssl systemd
```

### **2. Install mTLS Proxy Package**
```bash
# Download and install the RPM package
sudo rpm -ivh mtls-proxy-0.1.0-1.x86_64.rpm
```

### **3. Configure Certificates**
```bash
# Create certificate directory (if not exists)
sudo mkdir -p /etc/mtls-proxy/certs

# Copy your certificates
sudo cp your-client.crt /etc/mtls-proxy/certs/client.crt
sudo cp your-client.key /etc/mtls-proxy/certs/client.key
sudo cp your-ca.crt /etc/mtls-proxy/certs/ca.crt

# Set proper permissions
sudo chown -R mtls-proxy:mtls-proxy /etc/mtls-proxy/certs
sudo chmod 600 /etc/mtls-proxy/certs/client.key
sudo chmod 644 /etc/mtls-proxy/certs/client.crt
sudo chmod 644 /etc/mtls-proxy/certs/ca.crt
```

### **4. Start the Service**
```bash
# Start the mTLS proxy service
sudo systemctl start mtls-proxy

# Enable service to start on boot
sudo systemctl enable mtls-proxy

# Check service status
sudo systemctl status mtls-proxy
```

### **5. Verify Installation**
```bash
# Test health endpoint
curl http://localhost:8440/ui/health

# Expected response:
# {"status":"healthy","service":"mtls-proxy"}
```

---

## 🔧 **Configuration via Web UI**

### **1. Access the Web Interface**
- Open your web browser
- Navigate to: `http://your-server:8440/ui`
- Login with default credentials:
  - **Username**: `admin`
  - **Password**: `admin123`

### **2. Configure Target URL**
1. Navigate to **Configuration** page
2. Update the **Target URL** field
3. Set appropriate **Timeout** and **Max Connections**
4. Click **Save Configuration**

### **3. Upload Certificates**
1. Navigate to **Certificate Management**
2. Upload your client certificate (.crt file)
3. Upload your client private key (.key file)
4. Upload CA certificate (.crt file) if required
5. Click **Upload Certificates**

### **4. Configure Authentication**
1. Navigate to **Authentication Settings**
2. Enable/disable authentication as needed
3. Update admin username and password
4. Click **Save Authentication Settings**

---

## 📁 **File Locations**

### **Configuration Files**
```
/etc/mtls-proxy/
├── config.toml              # Main configuration file
├── certs/                   # Certificate directory
│   ├── client.crt           # Client certificate
│   ├── client.key           # Client private key
│   └── ca.crt               # CA certificate
└── logs/                    # Log directory
```

### **Data Files**
```
/var/lib/mtls-proxy/
├── proxy_logs.db            # SQLite log database
└── uploads/                 # Uploaded files
```

### **Service Files**
```
/usr/lib/systemd/system/
└── mtls-proxy.service       # systemd service file
```

### **Documentation**
```
/usr/share/doc/mtls-proxy/
├── README.md                # Main documentation
├── INSTALL.md               # This installation guide
└── CONFIGURATION.md         # Configuration guide
```

---

## 🔒 **Security Configuration**

### **1. Firewall Configuration**
```bash
# Allow HTTP traffic to proxy (if needed)
sudo firewall-cmd --permanent --add-port=8440/tcp
sudo firewall-cmd --reload

# Or restrict to specific IP ranges
sudo firewall-cmd --permanent --add-rich-rule='rule family="ipv4" source address="192.168.1.0/24" port port="8440" protocol="tcp" accept'
sudo firewall-cmd --reload
```

### **2. SELinux Configuration**
```bash
# Check SELinux status
sestatus

# If SELinux is enabled, configure it
sudo setsebool -P httpd_can_network_connect 1
sudo setsebool -P httpd_can_network_relay 1
```

### **3. Certificate Security**
```bash
# Ensure proper certificate permissions
sudo chown mtls-proxy:mtls-proxy /etc/mtls-proxy/certs/*
sudo chmod 600 /etc/mtls-proxy/certs/client.key
sudo chmod 644 /etc/mtls-proxy/certs/*.crt
```

---

## 🛠 **Service Management**

### **Basic Service Commands**
```bash
# Start the service
sudo systemctl start mtls-proxy

# Stop the service
sudo systemctl stop mtls-proxy

# Restart the service
sudo systemctl restart mtls-proxy

# Check service status
sudo systemctl status mtls-proxy

# Enable service on boot
sudo systemctl enable mtls-proxy

# Disable service on boot
sudo systemctl disable mtls-proxy
```

### **View Logs**
```bash
# View systemd logs
sudo journalctl -u mtls-proxy -f

# View application logs
sudo tail -f /var/lib/mtls-proxy/logs/proxy.log

# View SQLite logs
sudo sqlite3 /var/lib/mtls-proxy/proxy_logs.db "SELECT * FROM requests ORDER BY timestamp DESC LIMIT 10;"
```

---

## 🔍 **Troubleshooting**

### **Common Issues**

#### **1. Service Won't Start**
```bash
# Check service status
sudo systemctl status mtls-proxy

# Check logs for errors
sudo journalctl -u mtls-proxy -n 50

# Verify configuration
sudo /usr/bin/mtls-proxy --show-config
```

#### **2. Certificate Issues**
```bash
# Check certificate permissions
ls -la /etc/mtls-proxy/certs/

# Verify certificate validity
openssl x509 -in /etc/mtls-proxy/certs/client.crt -text -noout

# Check certificate chain
openssl verify -CAfile /etc/mtls-proxy/certs/ca.crt /etc/mtls-proxy/certs/client.crt
```

#### **3. Network Connectivity Issues**
```bash
# Test target connectivity
curl -v https://your-target-server:443

# Check firewall rules
sudo firewall-cmd --list-all

# Test proxy endpoint
curl -v http://localhost:8080/health
```

#### **4. Permission Issues**
```bash
# Fix ownership
sudo chown -R mtls-proxy:mtls-proxy /etc/mtls-proxy/
sudo chown -R mtls-proxy:mtls-proxy /var/lib/mtls-proxy/

# Fix permissions
sudo chmod 755 /etc/mtls-proxy/
sudo chmod 600 /etc/mtls-proxy/certs/client.key
sudo chmod 644 /etc/mtls-proxy/certs/*.crt
```

### **Getting Help**
- Check the logs: `sudo journalctl -u mtls-proxy`
- Review configuration: `/etc/mtls-proxy/config.toml`
- Test connectivity: `curl http://localhost:8080/health`
- Check system resources: `top`, `df -h`, `free -h`

---

## 🗑️ **Uninstallation**

### **Remove the Package**
```bash
# Stop and remove the service
sudo systemctl stop mtls-proxy
sudo systemctl disable mtls-proxy

# Remove the package
sudo rpm -e mtls-proxy

# Clean up data (optional)
sudo rm -rf /var/lib/mtls-proxy
sudo rm -rf /etc/mtls-proxy
```

### **Preserve Data During Uninstall**
```bash
# Backup configuration and data
sudo cp -r /etc/mtls-proxy /tmp/mtls-proxy-config-backup
sudo cp -r /var/lib/mtls-proxy /tmp/mtls-proxy-data-backup

# Remove package
sudo rpm -e mtls-proxy

# Restore data (if reinstalling)
sudo cp -r /tmp/mtls-proxy-config-backup /etc/mtls-proxy
sudo cp -r /tmp/mtls-proxy-data-backup /var/lib/mtls-proxy
```

---

## 📞 **Support**

For additional support:
- Check the documentation in `/usr/share/doc/mtls-proxy/`
- Review the configuration guide: `CONFIGURATION.md`
- Check system logs: `sudo journalctl -u mtls-proxy`
- Test the health endpoint: `curl http://localhost:8080/health`

---

**Last Updated**: 2025-08-15
**Version**: 0.1.0
