use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainConfig {
    pub name: String,
    pub domain: String,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub created_at: DateTime<Utc>,
}