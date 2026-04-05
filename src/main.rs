mod types;
mod ca;
mod config;
mod hosts;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use chrono::Utc;
use directories::ProjectDirs;
use std::fs;

use crate::ca::CaGenerator;
use crate::config::ConfigPersistence;
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
    Create { name: String },
    List,
    Remove { name: String },
    Ca,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let proj_dirs = ProjectDirs::from("com", "vh", "vh-cli")
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
        Commands::Create { name } => {
            let domain = format!("{}.test", name);
            let cert_dir = data_dir.join("certs");
            fs::create_dir_all(&cert_dir)?;

            let cert_path = cert_dir.join(format!("{}.crt", domain));
            let key_path = cert_dir.join(format!("{}.key", domain));

            HostsModifier::add_entry(&domain)?;
            CaGenerator::create_domain_cert(
                &domain,
                &config.root_ca_cert,
                &config.root_ca_key,
                &cert_path,
                &key_path
            )?;

            config.domains.push(DomainConfig {
                name,
                domain,
                cert_path,
                key_path,
                created_at: Utc::now(),
            });
            config_store.save(&config)?;
            println!("Domain created successfully.");
        }
        Commands::List => {
            println!("{:<15} {:<20} {:<20}", "NAME", "DOMAIN", "CREATED");
            for d in &config.domains {
                println!("{:<15} {:<20} {:<20}", d.name, d.domain, d.created_at.to_rfc3339());
            }
        }
        Commands::Remove { name } => {
            if let Some(pos) = config.domains.iter().position(|d| d.name == name) {
                let domain = config.domains.remove(pos);
                HostsModifier::remove_entry(&domain.domain)?;
                config_store.save(&config)?;
                println!("Domain {} removed.", name);
            }
        }
        Commands::Ca => {
            println!("Root CA: {}", config.root_ca_cert.display());
            println!("Root Key: {}", config.root_ca_key.display());
        }
    }

    Ok(())
}