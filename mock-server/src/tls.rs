use anyhow::Result;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio_rustls::TlsAcceptor;

pub struct TlsServer {
    acceptor: TlsAcceptor,
}

impl TlsServer {
    pub fn new(
        cert_path: &Path,
        key_path: &Path,
        _ca_cert_path: Option<&Path>,
        _require_client_cert: bool,
    ) -> Result<Self> {
        // Load server certificate
        let server_cert = load_certificate(cert_path)?;
        
        // Load server private key
        let server_key = load_private_key(key_path)?;
        
        // Create server config with no client auth for now
        let mut server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(vec![server_cert], server_key)?;

        // Enable HTTP/2
        server_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        let acceptor = TlsAcceptor::from(Arc::new(server_config));
        
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
