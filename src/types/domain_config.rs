use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use uuid::Uuid;

fn default_ip() -> String {
    "127.0.0.1".to_string()
}

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DomainConfig {
    #[serde(default = "generate_id")]
    pub id: String,
    pub name: String,
    pub domain: String,
    #[serde(default = "default_ip")]
    pub ip: String,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
    pub created_at: DateTime<Utc>,
}