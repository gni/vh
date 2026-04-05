use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use crate::types::AppConfig;

pub struct ConfigPersistence {
    pub config_path: PathBuf,
}

impl ConfigPersistence {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            config_path: data_dir.join("config.json"),
        }
    }

    pub fn load(&self, data_dir: &PathBuf) -> Result<AppConfig> {
        if !self.config_path.exists() {
            return Ok(AppConfig {
                domains: Vec::new(),
                allowed_extensions: Vec::new(),
                root_ca_cert: data_dir.join("root_ca.crt"),
                root_ca_key: data_dir.join("root_ca.key"),
            });
        }
        let content = fs::read_to_string(&self.config_path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, config: &AppConfig) -> Result<()> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }
}