use anyhow::Result;
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, IsCa, Issuer, KeyPair,
    SanType,
};
use std::fs;
use std::path::Path;

pub struct CaGenerator;

impl CaGenerator {
    pub fn create_root_ca(cert_path: &Path, key_path: &Path) -> Result<()> {
        let mut params = CertificateParams::default();
        params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);

        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, "VH Local Root CA");
        dn.push(DnType::OrganizationName, "VH Development Tools");
        params.distinguished_name = dn;

        let key_pair = KeyPair::generate()?;
        let cert = params.self_signed(&key_pair)?;

        fs::write(cert_path, cert.pem())?;
        fs::write(key_path, key_pair.serialize_pem())?;

        Ok(())
    }

    pub fn create_domain_cert(
        domain: &str,
        ca_cert_path: &Path,
        ca_key_path: &Path,
        cert_out: &Path,
        key_out: &Path,
    ) -> Result<()> {
        let ca_cert_pem = fs::read_to_string(ca_cert_path)?;
        let ca_key_pem = fs::read_to_string(ca_key_path)?;

        let ca_key_pair = KeyPair::from_pem(&ca_key_pem)?;
        
        let issuer = Issuer::from_ca_cert_pem(&ca_cert_pem, ca_key_pair)
            .map_err(|e| anyhow::anyhow!("Failed to parse CA: {}", e))?;

        let mut params = CertificateParams::new(vec![domain.to_string()])?;
        params.distinguished_name.push(DnType::CommonName, domain);
        
        let san_name = domain.to_string().try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert domain to IA5String"))?;
        params.subject_alt_names.push(SanType::DnsName(san_name));

        let cert_key_pair = KeyPair::generate()?;
        let cert = params.signed_by(&cert_key_pair, &issuer)?;

        fs::write(cert_out, cert.pem())?;
        fs::write(key_out, cert_key_pair.serialize_pem())?;

        Ok(())
    }
}