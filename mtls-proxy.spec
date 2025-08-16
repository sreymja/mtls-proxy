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

%changelog
* Thu Aug 15 2025 Your Name <your.email@example.com> - 0.1.0-1
- Initial release
- mTLS proxy server with web UI
- Certificate upload and management
- Configuration persistence
- systemd service integration
