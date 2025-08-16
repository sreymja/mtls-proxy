use anyhow::Result;
use rustls::{Certificate, PrivateKey, RootCertStore};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tokio_rustls::TlsConnector;

pub struct TlsClient {
    pub(crate) connector: TlsConnector,
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
    use rustls::client::{ServerCertVerifier, ServerCertVerified};
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
}
