use anyhow::{Context, Result};
use std::process::Command;
use std::io::Write;

pub struct HostsModifier;

impl HostsModifier {
    const HOSTS_PATH: &'static str = "/etc/hosts";

    pub fn add_entry(domain: &str, ip: &str) -> Result<()> {
        let entry = format!("{} {}", ip, domain);
        let current = std::fs::read_to_string(Self::HOSTS_PATH)?;

        let mut lines: Vec<&str> = current
            .lines()
            .filter(|line| !line.contains(domain))
            .collect();

        lines.push(&entry);
        let updated = format!("{}\n", lines.join("\n"));
        
        Self::apply_changes(updated)
    }

    pub fn remove_entry(domain: &str) -> Result<()> {
        let current = std::fs::read_to_string(Self::HOSTS_PATH)?;
        
        let filtered: Vec<&str> = current
            .lines()
            .filter(|line| !line.contains(domain))
            .collect();

        let updated = format!("{}\n", filtered.join("\n"));
        Self::apply_changes(updated)
    }

    fn apply_changes(content: String) -> Result<()> {
        let mut child = Command::new("sudo")
            .arg("tee")
            .arg(Self::HOSTS_PATH)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::null())
            .spawn()
            .context("Failed to elevate privileges for /etc/hosts update")?;

        let mut stdin = child.stdin.take().context("Failed to capture stdin")?;
        stdin.write_all(content.as_bytes())?;
        drop(stdin);

        let status = child.wait()?;
        if !status.success() {
            return Err(anyhow::anyhow!("Sudo operation failed with status {}", status));
        }
        Ok(())
    }
}