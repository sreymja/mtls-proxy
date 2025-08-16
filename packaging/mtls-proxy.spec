Name:           mtls-proxy
Version:        0.1.0
Release:        1%{?dist}
Summary:        High-performance mTLS proxy server with web interface

License:        MIT
URL:            https://github.com/your-org/mtls-proxy
Source0:        %{name}-%{version}.tar.gz
BuildArch:      x86_64

# Build dependencies
BuildRequires:  rust
BuildRequires:  cargo
BuildRequires:  pkgconfig
BuildRequires:  openssl-devel

# Runtime dependencies
Requires:       openssl
Requires:       systemd

# Optional dependencies
Suggests:       sqlite

%description
A high-performance, secure mTLS (mutual TLS) proxy server built in Rust with a modern web interface for configuration and monitoring.

Features:
- Secure mTLS proxy with certificate management
- Modern HTML-based web interface
- Comprehensive REST API
- SQLite-based audit logging
- Built-in rate limiting
- Prometheus-compatible metrics
- Standardized error handling

%package doc
Summary:        Documentation for %{name}
Requires:       %{name} = %{version}-%{release}

%description doc
Documentation for the mTLS proxy server, including:
- API documentation
- User guide
- Developer guide
- Configuration examples

%prep
%autosetup -p1

%build
# Build the release binary
%cargo_build --release

%install
# Create directories
install -d %{buildroot}%{_bindir}
install -d %{buildroot}%{_sysconfdir}/%{name}
install -d %{buildroot}%{_localstatedir}/log/%{name}
install -d %{buildroot}%{_localstatedir}/lib/%{name}
install -d %{buildroot}%{_unitdir}
install -d %{buildroot}%{_presetdir}
install -d %{buildroot}%{_mandir}/man8
install -d %{buildroot}%{_docdir}/%{name}

# Install binary
install -m 755 target/release/%{name} %{buildroot}%{_bindir}/%{name}

# Install configuration files
install -m 644 config/default.toml %{buildroot}%{_sysconfdir}/%{name}/
install -m 644 config/local.toml %{buildroot}%{_sysconfdir}/%{name}/ 2>/dev/null || true

# Install systemd service file
install -m 644 packaging/%{name}.service %{buildroot}%{_unitdir}/

# Install systemd preset
install -m 644 packaging/%{name}.preset %{buildroot}%{_presetdir}/

# Install man page
install -m 644 packaging/%{name}.8 %{buildroot}%{_mandir}/man8/

# Install documentation
install -m 644 README.md %{buildroot}%{_docdir}/%{name}/
install -m 644 docs/*.md %{buildroot}%{_docdir}/%{name}/ 2>/dev/null || true

# Create certificate directory
install -d %{buildroot}%{_sysconfdir}/%{name}/certs

# Set proper ownership for log and data directories
%{__chmod} 755 %{buildroot}%{_localstatedir}/log/%{name}
%{__chmod} 755 %{buildroot}%{_localstatedir}/lib/%{name}
%{__chmod} 700 %{buildroot}%{_sysconfdir}/%{name}/certs

%files
%license LICENSE
%{_bindir}/%{name}
%{_sysconfdir}/%{name}/default.toml
%config(noreplace) %{_sysconfdir}/%{name}/local.toml
%{_sysconfdir}/%{name}/certs
%{_unitdir}/%{name}.service
%{_presetdir}/%{name}.preset
%{_mandir}/man8/%{name}.8
%dir %{_localstatedir}/log/%{name}
%dir %{_localstatedir}/lib/%{name}

%files doc
%{_docdir}/%{name}/README.md
%{_docdir}/%{name}/*.md

%pre
# Create system user and group if they don't exist
getent group %{name} >/dev/null 2>&1 || groupadd -r %{name}
getent passwd %{name} >/dev/null 2>&1 || useradd -r -g %{name} -d /var/lib/%{name} -s /sbin/nologin %{name}

%post
# Enable systemd preset
%systemd_post %{name}.service

# Set proper permissions
chown -R %{name}:%{name} %{_localstatedir}/log/%{name} 2>/dev/null || true
chown -R %{name}:%{name} %{_localstatedir}/lib/%{name} 2>/dev/null || true
chown -R %{name}:%{name} %{_sysconfdir}/%{name}/certs 2>/dev/null || true

%preun
%systemd_preun %{name}.service

%postun
%systemd_postun_with_restart %{name}.service

%posttrans
# Reload systemd daemon
systemctl daemon-reload >/dev/null 2>&1 || true

%changelog
* %(date '+%a %b %d %Y') %{packager} - %{version}-%{release}
- Initial RPM package for mTLS proxy
- Includes systemd service configuration
- Includes documentation and man pages
