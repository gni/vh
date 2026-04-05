use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::types::domain_config::DomainConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub domains: Vec<DomainConfig>,
    pub root_ca_cert: PathBuf,
    pub root_ca_key: PathBuf,
}