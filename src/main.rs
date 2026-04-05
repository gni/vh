mod types;
mod ca;
mod config;
mod hosts;
mod domain;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use chrono::Utc;
use directories::ProjectDirs;
use std::fs;
use uuid::Uuid;

use crate::ca::{CaGenerator, CaInstructions};
use crate::config::ConfigPersistence;
use crate::domain::DomainDescriptor;
use crate::hosts::HostsModifier;
use crate::types::DomainConfig;

#[derive(Parser)]
#[command(name = "vh", version = "0.1.0", about = "Production-grade Local VHost Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create { 
        domain: String,
        #[arg(short, long, default_value = "127.0.0.1", help = "IP address to point the domain to")]
        ip: String,
    },
    List,
    Describe {
        identifier: String,
    },
    Remove { 
        identifier: String 
    },
    AllowExt {
        ext: String,
    },
    RemoveExt {
        ext: String,
    },
    Ca,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let proj_dirs = ProjectDirs::from("com", "vh", "vh")
        .context("Failed to resolve project directories")?;
    let data_dir = proj_dirs.data_dir().to_path_buf();

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    let config_store = ConfigPersistence::new(data_dir.clone());
    let mut config = config_store.load(&data_dir)?;

    if !config.root_ca_cert.exists() {
        CaGenerator::create_root_ca(&config.root_ca_cert, &config.root_ca_key)?;
    }

    match cli.command {
        Commands::Create { domain: input, ip } => {
            let (target_domain, tld) = if let Some(idx) = input.rfind('.') {
                (input.clone(), input[idx + 1..].to_string())
            } else {
                (format!("{}.test", input), "test".to_string())
            };

            let safe_extensions = ["test", "localhost", "invalid", "example", "local", "loc"];
            let is_safe = safe_extensions.contains(&tld.as_str()) || config.allowed_extensions.contains(&tld);

            if !is_safe {
                anyhow::bail!(
                    "[ERROR] Unsafe Extension Blocked: '.{}'\n\
                    \n\
                    RISK EXPLANATION:\n\
                    Using a real TLD (like .com, .dev, .app) locally hijacks DNS queries. \
                    Traffic meant for the real internet will be forcefully routed to your \
                    localhost. This breaks external services, APIs, and web browsing.\n\
                    \n\
                    Allowed safe local extensions (RFC 2606 / mDNS / common dev): {:?}\n\
                    Custom allowed extensions: {:?}\n\
                    \n\
                    If you know what you are doing, allow it with:\n\
                    vh allow-ext {}",
                    tld,
                    safe_extensions,
                    config.allowed_extensions,
                    tld
                );
            }

            let mut new_id = Uuid::new_v4().to_string();
            let mut is_update = false;

            if let Some(idx) = config.domains.iter().position(|d| d.domain == target_domain || d.name == input) {
                let existing = config.domains.remove(idx);
                new_id = existing.id;
                is_update = true;
                HostsModifier::remove_entry(&existing.domain)?;
            }

            let cert_dir = data_dir.join("certs");
            fs::create_dir_all(&cert_dir)?;

            let cert_path = cert_dir.join(format!("{}.crt", target_domain));
            let key_path = cert_dir.join(format!("{}.key", target_domain));

            HostsModifier::add_entry(&target_domain, &ip)?;
            CaGenerator::create_domain_cert(
                &target_domain,
                &config.root_ca_cert,
                &config.root_ca_key,
                &cert_path,
                &key_path
            )?;

            config.domains.push(DomainConfig {
                id: new_id.clone(),
                name: input,
                domain: target_domain.clone(),
                ip,
                cert_path,
                key_path,
                created_at: Utc::now(),
            });
            
            config_store.save(&config)?;
            
            let short_id: String = new_id.chars().take(8).collect();
            let action = if is_update { "updated" } else { "created" };
            println!("[SUCCESS] Domain {} ({}) {} successfully.", target_domain, short_id, action);
        }
        Commands::List => {
            println!("{:<10} {:<25} {:<15} {:<20}", "ID", "DOMAIN", "IP", "CREATED");
            for d in &config.domains {
                let short_id: String = d.id.chars().take(8).collect();
                println!("{:<10} {:<25} {:<15} {:<20}", short_id, d.domain, d.ip, d.created_at.format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        Commands::Describe { identifier } => {
            if let Some(domain_cfg) = config.domains.iter().find(|d| d.id.starts_with(&identifier) || d.domain == identifier || d.name == identifier) {
                DomainDescriptor::print(domain_cfg);
            } else {
                println!("[ERROR] Domain or ID '{}' not found.", identifier);
            }
        }
        Commands::Remove { identifier } => {
            if let Some(pos) = config.domains.iter().position(|d| d.id.starts_with(&identifier) || d.domain == identifier || d.name == identifier) {
                let removed = config.domains.remove(pos);
                HostsModifier::remove_entry(&removed.domain)?;
                config_store.save(&config)?;
                
                let short_id: String = removed.id.chars().take(8).collect();
                println!("[SUCCESS] Domain {} ({}) removed.", removed.domain, short_id);
            } else {
                println!("[ERROR] Domain or ID '{}' not found.", identifier);
            }
        }
        Commands::AllowExt { ext } => {
            let clean_ext = ext.trim_start_matches('.').to_string();
            if !config.allowed_extensions.contains(&clean_ext) {
                config.allowed_extensions.push(clean_ext.clone());
                config_store.save(&config)?;
                println!("[SUCCESS] Added '.{}' to allowed extensions.", clean_ext);
            } else {
                println!("[INFO] Extension '.{}' is already allowed.", clean_ext);
            }
        }
        Commands::RemoveExt { ext } => {
            let clean_ext = ext.trim_start_matches('.');
            if let Some(pos) = config.allowed_extensions.iter().position(|e| e == clean_ext) {
                config.allowed_extensions.remove(pos);
                config_store.save(&config)?;
                println!("[SUCCESS] Removed '.{}' from allowed extensions.", clean_ext);
            } else {
                println!("[WARNING] Extension '.{}' not found in custom allowed list.", clean_ext);
            }
        }
        Commands::Ca => {
            CaInstructions::print(&config.root_ca_cert, &config.root_ca_key);
        }
    }

    Ok(())
}