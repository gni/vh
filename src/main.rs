mod types;
mod ca;
mod config;
mod hosts;
mod domain;
mod logger;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Shell};
use chrono::Utc;
use colored::Colorize;
use directories::ProjectDirs;
use std::fs;
use std::io;
use uuid::Uuid;

use crate::ca::{CaGenerator, CaInstructions};
use crate::config::ConfigPersistence;
use crate::domain::DomainDescriptor;
use crate::hosts::HostsModifier;
use crate::logger::Logger;
use crate::types::DomainConfig;

#[derive(Parser)]
#[command(name = "vh", version = "0.1.0", about = "Production-grade Local VHost Manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose logging output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create or update a local development domain
    Create { 
        /// Domain name (e.g., 'myapp' creates 'myapp.test', 'api.local' creates 'api.local')
        domain: String,
        /// Target IP address for the local domain resolution
        #[arg(short, long, default_value = "127.0.0.1", help = "IP address to point the domain to")]
        ip: String,
    },
    /// List all managed domains
    List,
    /// View detailed information and integration paths for a domain
    Describe {
        /// ID, name, or domain to describe
        identifier: String,
    },
    /// Remove a managed domain
    Remove { 
        /// ID, name, or domain to remove
        identifier: String 
    },
    /// Manage allowed custom domain extensions (TLDs)
    #[command(alias = "ext")]
    Extension {
        #[command(subcommand)]
        command: ExtensionCommands,
    },
    /// Generate shell completion scripts
    Completions {
        /// The shell to generate the completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Show path to the Root CA certificate and installation instructions
    Ca,
}

#[derive(Subcommand)]
enum ExtensionCommands {
    /// Allow a custom domain extension (TLD)
    Allow {
        /// The extension to allow (e.g., 'dev')
        name: String,
    },
    /// Remove a custom domain extension (TLD) from the allowed list
    Remove {
        /// The extension to remove
        name: String,
    },
    /// List all custom allowed extensions
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    Logger::init(cli.verbose);

    let proj_dirs = ProjectDirs::from("com", "vh", "vh")
        .context("Failed to resolve project directories")?;
    let data_dir = proj_dirs.data_dir().to_path_buf();

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    let config_store = ConfigPersistence::new(data_dir.clone());
    let mut config = config_store.load(&data_dir)?;

    if !matches!(cli.command, Commands::Completions { .. }) && !config.root_ca_cert.exists() {
        tracing::debug!("Initializing new Root CA at {:?}", config.root_ca_cert);
        CaGenerator::create_root_ca(&config.root_ca_cert, &config.root_ca_key)?;
    }

    match cli.command {
        Commands::Create { domain: input, ip } => {
            tracing::debug!("Processing create request for domain: {}", input);
            let (target_domain, tld) = if let Some(idx) = input.rfind('.') {
                (input.clone(), input[idx + 1..].to_string())
            } else {
                (format!("{}.test", input), "test".to_string())
            };

            let safe_extensions = ["test", "localhost", "invalid", "example", "local", "loc"];
            let is_safe = safe_extensions.contains(&tld.as_str()) || config.allowed_extensions.contains(&tld);

            if !is_safe {
                anyhow::bail!(
                    "{} Unsafe Extension Blocked: '.{}'\n\
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
                    vh extension allow {}",
                    "[ERROR]".red().bold(),
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
                tracing::debug!("Found existing domain, removing from hosts before update");
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
            println!("{} Domain {} ({}) {} successfully.", "[SUCCESS]".green().bold(), target_domain.bold(), short_id, action);
        }
        Commands::List => {
            println!("{:<10} {:<25} {:<15} {:<20}", "ID".bold(), "DOMAIN".bold(), "IP".bold(), "CREATED".bold());
            for d in &config.domains {
                let short_id: String = d.id.chars().take(8).collect();
                println!("{:<10} {:<25} {:<15} {:<20}", short_id, d.domain, d.ip, d.created_at.format("%Y-%m-%d %H:%M:%S").to_string());
            }
        }
        Commands::Describe { identifier } => {
            if let Some(domain_cfg) = config.domains.iter().find(|d| d.id.starts_with(&identifier) || d.domain == identifier || d.name == identifier) {
                DomainDescriptor::print(domain_cfg);
            } else {
                println!("{} Domain or ID '{}' not found.", "[ERROR]".red().bold(), identifier);
            }
        }
        Commands::Remove { identifier } => {
            if let Some(pos) = config.domains.iter().position(|d| d.id.starts_with(&identifier) || d.domain == identifier || d.name == identifier) {
                let removed = config.domains.remove(pos);
                HostsModifier::remove_entry(&removed.domain)?;
                config_store.save(&config)?;
                
                let short_id: String = removed.id.chars().take(8).collect();
                println!("{} Domain {} ({}) removed.", "[SUCCESS]".green().bold(), removed.domain.bold(), short_id);
            } else {
                println!("{} Domain or ID '{}' not found.", "[ERROR]".red().bold(), identifier);
            }
        }
        Commands::Extension { command } => match command {
            ExtensionCommands::Allow { name } => {
                let clean_ext = name.trim_start_matches('.').to_string();
                if !config.allowed_extensions.contains(&clean_ext) {
                    config.allowed_extensions.push(clean_ext.clone());
                    config_store.save(&config)?;
                    println!("{} Added '.{}' to allowed extensions.", "[SUCCESS]".green().bold(), clean_ext);
                } else {
                    println!("{} Extension '.{}' is already allowed.", "[INFO]".cyan().bold(), clean_ext);
                }
            }
            ExtensionCommands::Remove { name } => {
                let clean_ext = name.trim_start_matches('.');
                if let Some(pos) = config.allowed_extensions.iter().position(|e| e == clean_ext) {
                    config.allowed_extensions.remove(pos);
                    config_store.save(&config)?;
                    println!("{} Removed '.{}' from allowed extensions.", "[SUCCESS]".green().bold(), clean_ext);
                } else {
                    println!("{} Extension '.{}' not found in custom allowed list.", "[WARNING]".yellow().bold(), clean_ext);
                }
            }
            ExtensionCommands::List => {
                if config.allowed_extensions.is_empty() {
                    println!("No custom extensions allowed yet.");
                } else {
                    println!("{}", "Custom Allowed Extensions:".bold());
                    for ext in &config.allowed_extensions {
                        println!("  .{}", ext);
                    }
                }
            }
        },
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "vh", &mut io::stdout());
        }
        Commands::Ca => {
            CaInstructions::print(&config.root_ca_cert, &config.root_ca_key);
        }
    }

    Ok(())
}