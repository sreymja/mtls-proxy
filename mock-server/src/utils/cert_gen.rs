use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct CertificateGenerator;

impl CertificateGenerator {
    pub fn generate_test_certificates(cert_dir: &Path) -> Result<()> {
        // Create certificate directory if it doesn't exist
        fs::create_dir_all(cert_dir)?;

        // Generate CA certificate and key
        Self::generate_ca_certificate(cert_dir)?;
        
        // Generate server certificate and key
        Self::generate_server_certificate(cert_dir)?;
        
        // Generate client certificate and key
        Self::generate_client_certificate(cert_dir)?;

        println!("Test certificates generated in: {}", cert_dir.display());
        Ok(())
    }

    fn generate_ca_certificate(cert_dir: &Path) -> Result<()> {
        let ca_key_path = cert_dir.join("ca.key");
        let ca_cert_path = cert_dir.join("ca.crt");

        // Generate CA private key
        let rsa = openssl::rsa::Rsa::generate(2048)?;
        let ca_key = openssl::pkey::PKey::from_rsa(rsa)?;
        fs::write(&ca_key_path, ca_key.private_key_to_pem_pkcs8()?)?;

        // Generate CA certificate
        let mut ca_cert = openssl::x509::X509::builder()?;
        ca_cert.set_version(2)?;
        let serial = openssl::bn::BigNum::from_u32(1)?;
        let serial_int = openssl::asn1::Asn1Integer::from_bn(&serial)?;
        ca_cert.set_serial_number(&serial_int)?;
        ca_cert.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0)?.as_ref())?;
        ca_cert.set_not_after(&openssl::asn1::Asn1Time::days_from_now(365)?.as_ref())?;
        
        let mut subject = openssl::x509::X509Name::builder()?;
        subject.append_entry_by_text("C", "US")?;
        subject.append_entry_by_text("ST", "CA")?;
        subject.append_entry_by_text("L", "San Francisco")?;
        subject.append_entry_by_text("O", "Test CA")?;
        subject.append_entry_by_text("CN", "Test CA")?;
        let subject_name = subject.build();
        ca_cert.set_subject_name(&subject_name)?;
        ca_cert.set_issuer_name(&subject_name)?;
        
        ca_cert.set_pubkey(&ca_key)?;
        ca_cert.sign(&ca_key, openssl::hash::MessageDigest::sha256())?;
        
        fs::write(&ca_cert_path, ca_cert.build().to_pem()?)?;
        
        Ok(())
    }

    fn generate_server_certificate(cert_dir: &Path) -> Result<()> {
        let ca_key_path = cert_dir.join("ca.key");
        let ca_cert_path = cert_dir.join("ca.crt");
        let server_key_path = cert_dir.join("server.key");
        let server_cert_path = cert_dir.join("server.crt");

        // Load CA key and certificate
        let ca_key = openssl::pkey::PKey::private_key_from_pem(&fs::read(&ca_key_path)?)?;
        let ca_cert = openssl::x509::X509::from_pem(&fs::read(&ca_cert_path)?)?;

        // Generate server private key
        let rsa = openssl::rsa::Rsa::generate(2048)?;
        let server_key = openssl::pkey::PKey::from_rsa(rsa)?;
        fs::write(&server_key_path, server_key.private_key_to_pem_pkcs8()?)?;

        // Generate server certificate
        let mut server_cert = openssl::x509::X509::builder()?;
        server_cert.set_version(2)?;
        let serial = openssl::bn::BigNum::from_u32(2)?;
        let serial_int = openssl::asn1::Asn1Integer::from_bn(&serial)?;
        server_cert.set_serial_number(&serial_int)?;
        server_cert.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0)?.as_ref())?;
        server_cert.set_not_after(&openssl::asn1::Asn1Time::days_from_now(365)?.as_ref())?;
        
        let mut subject = openssl::x509::X509Name::builder()?;
        subject.append_entry_by_text("C", "US")?;
        subject.append_entry_by_text("ST", "CA")?;
        subject.append_entry_by_text("L", "San Francisco")?;
        subject.append_entry_by_text("O", "Test Server")?;
        subject.append_entry_by_text("CN", "localhost")?;
        server_cert.set_subject_name(&subject.build())?;
        server_cert.set_issuer_name(ca_cert.subject_name())?;
        
        server_cert.set_pubkey(&server_key)?;
        server_cert.sign(&ca_key, openssl::hash::MessageDigest::sha256())?;
        
        fs::write(&server_cert_path, server_cert.build().to_pem()?)?;
        
        Ok(())
    }

    fn generate_client_certificate(cert_dir: &Path) -> Result<()> {
        let ca_key_path = cert_dir.join("ca.key");
        let ca_cert_path = cert_dir.join("ca.crt");
        let client_key_path = cert_dir.join("client.key");
        let client_cert_path = cert_dir.join("client.crt");

        // Load CA key and certificate
        let ca_key = openssl::pkey::PKey::private_key_from_pem(&fs::read(&ca_key_path)?)?;
        let ca_cert = openssl::x509::X509::from_pem(&fs::read(&ca_cert_path)?)?;

        // Generate client private key
        let rsa = openssl::rsa::Rsa::generate(2048)?;
        let client_key = openssl::pkey::PKey::from_rsa(rsa)?;
        fs::write(&client_key_path, client_key.private_key_to_pem_pkcs8()?)?;

        // Generate client certificate
        let mut client_cert = openssl::x509::X509::builder()?;
        client_cert.set_version(2)?;
        let serial = openssl::bn::BigNum::from_u32(3)?;
        let serial_int = openssl::asn1::Asn1Integer::from_bn(&serial)?;
        client_cert.set_serial_number(&serial_int)?;
        client_cert.set_not_before(&openssl::asn1::Asn1Time::days_from_now(0)?.as_ref())?;
        client_cert.set_not_after(&openssl::asn1::Asn1Time::days_from_now(365)?.as_ref())?;
        
        let mut subject = openssl::x509::X509Name::builder()?;
        subject.append_entry_by_text("C", "US")?;
        subject.append_entry_by_text("ST", "CA")?;
        subject.append_entry_by_text("L", "San Francisco")?;
        subject.append_entry_by_text("O", "Test Client")?;
        subject.append_entry_by_text("CN", "test-client")?;
        client_cert.set_subject_name(&subject.build())?;
        client_cert.set_issuer_name(ca_cert.subject_name())?;
        
        client_cert.set_pubkey(&client_key)?;
        client_cert.sign(&ca_key, openssl::hash::MessageDigest::sha256())?;
        
        fs::write(&client_cert_path, client_cert.build().to_pem()?)?;
        
        Ok(())
    }
}
