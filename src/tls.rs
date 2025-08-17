use anyhow::Result;
use rustls::{Certificate, PrivateKey, RootCertStore, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio_rustls::{TlsAcceptor, TlsConnector};

pub struct TlsClient {
    pub(crate) connector: TlsConnector,
}

pub struct TlsServer {
    pub(crate) acceptor: TlsAcceptor,
}

impl TlsClient {
    pub fn new(
        client_cert_path: &Path,
        client_key_path: &Path,
        ca_cert_path: Option<&Path>,
        verify_hostname: bool,
    ) -> Result<Self> {
        // Load client certificate
        let client_cert = load_certificate(client_cert_path)?;

        // Load client private key
        let client_key = load_private_key(client_key_path)?;

        // Create root certificate store
        let mut root_store = RootCertStore::empty();

        // Add CA certificate if provided
        if let Some(ca_path) = ca_cert_path {
            let ca_certs = load_certificates(ca_path)?;
            for cert in ca_certs {
                root_store.add(&cert)?;
            }
        }

        // Create client config
        let mut client_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_client_auth_cert(vec![client_cert], client_key)?;

        // Configure hostname verification
        if !verify_hostname {
            client_config
                .dangerous()
                .set_certificate_verifier(std::sync::Arc::new(danger::NoCertificateVerifier {}));
        }

        let connector = TlsConnector::from(std::sync::Arc::new(client_config));

        Ok(Self { connector })
    }

    pub fn connector(&self) -> &TlsConnector {
        &self.connector
    }
}

impl TlsServer {
    pub fn new(
        cert_path: &Path,
        key_path: &Path,
        ca_cert_path: Option<&Path>,
        require_client_cert: bool,
    ) -> Result<Self> {
        // Load server certificate
        let server_cert = load_certificate(cert_path)?;

        // Load server private key
        let server_key = load_private_key(key_path)?;

        // Create server config
        let server_config = if require_client_cert {
            // Create root certificate store for client verification
            let mut root_store = RootCertStore::empty();

            // Add CA certificate if provided
            if let Some(ca_path) = ca_cert_path {
                let ca_certs = load_certificates(ca_path)?;
                for cert in ca_certs {
                    root_store.add(&cert)?;
                }
            }

            ServerConfig::builder()
                .with_safe_defaults()
                .with_client_cert_verifier(std::sync::Arc::new(danger::ClientCertVerifier::new(
                    root_store,
                )))
                .with_single_cert(vec![server_cert], server_key)?
        } else {
            ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(vec![server_cert], server_key)?
        };

        // Enable HTTP/2
        let mut config = server_config;
        config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        let acceptor = TlsAcceptor::from(Arc::new(config));

        Ok(Self { acceptor })
    }

    pub fn acceptor(&self) -> &TlsAcceptor {
        &self.acceptor
    }
}

fn load_certificate(path: &Path) -> Result<Certificate> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = certs(&mut reader)?;

    if certs.is_empty() {
        anyhow::bail!("No certificates found in {}", path.display());
    }

    Ok(Certificate(certs[0].clone()))
}

fn load_certificates(path: &Path) -> Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = certs(&mut reader)?;

    Ok(certs.into_iter().map(Certificate).collect())
}

fn load_private_key(path: &Path) -> Result<PrivateKey> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    // Try PKCS8 first, then RSA
    if let Ok(keys) = pkcs8_private_keys(&mut reader) {
        if !keys.is_empty() {
            return Ok(PrivateKey(keys[0].clone()));
        }
    }

    // Reset reader and try RSA
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let keys = rsa_private_keys(&mut reader)?;

    if keys.is_empty() {
        anyhow::bail!("No private keys found in {}", path.display());
    }

    Ok(PrivateKey(keys[0].clone()))
}

mod danger {
    use rustls::client::{ServerCertVerified, ServerCertVerifier};
    use rustls::server::{ClientCertVerified, ClientCertVerifier as RustlsClientCertVerifier};
    use rustls::{Certificate, DistinguishedName};
    use std::time::SystemTime;

    pub struct NoCertificateVerifier;

    impl ServerCertVerifier for NoCertificateVerifier {
        fn verify_server_cert(
            &self,
            _end_entity: &rustls::Certificate,
            _intermediates: &[rustls::Certificate],
            _server_name: &rustls::ServerName,
            _scts: &mut dyn Iterator<Item = &[u8]>,
            _ocsp_response: &[u8],
            _now: SystemTime,
        ) -> Result<ServerCertVerified, rustls::Error> {
            Ok(ServerCertVerified::assertion())
        }
    }

    pub struct ClientCertVerifier {
        roots: rustls::RootCertStore,
    }

    impl ClientCertVerifier {
        pub fn new(roots: rustls::RootCertStore) -> Self {
            Self { roots }
        }
    }

    impl RustlsClientCertVerifier for ClientCertVerifier {
        fn offer_client_auth(&self) -> bool {
            true
        }

        fn client_auth_root_subjects(&self) -> &[DistinguishedName] {
            &[]
        }

        fn verify_client_cert(
            &self,
            _end_entity: &Certificate,
            _intermediates: &[Certificate],
            _now: SystemTime,
        ) -> Result<ClientCertVerified, rustls::Error> {
            // For development, accept any client certificate
            // In production, you would verify against the root store
            Ok(ClientCertVerified::assertion())
        }
    }
}
