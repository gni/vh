use std::env::consts::OS;
use std::path::Path;

pub struct CaInstructions;

impl CaInstructions {
    pub fn print(cert_path: &Path, key_path: &Path) {
        println!("Root CA Certificate: {}", cert_path.display());
        println!("Root CA Private Key: {}", key_path.display());
        
        println!("\n[TRUST INSTRUCTIONS]");
        println!("To prevent browser SSL warnings, install the Root CA into your system's trust store:\n");
        
        match OS {
            "macos" => {
                println!("- macOS:");
                println!("  sudo security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain {}", cert_path.display());
            }
            "linux" => {
                if Path::new("/etc/debian_version").exists() {
                    println!("- Linux (Debian/Ubuntu):");
                    println!("  sudo cp {} /usr/local/share/ca-certificates/vh-local-ca.crt", cert_path.display());
                    println!("  sudo update-ca-certificates");
                } else if Path::new("/etc/arch-release").exists() || Path::new("/etc/fedora-release").exists() {
                    println!("- Linux (Arch/Fedora):");
                    println!("  sudo trust anchor {}", cert_path.display());
                } else {
                    println!("- Linux (Generic):");
                    println!("  Please consult your distribution's manual on how to add a trusted Root CA.");
                }
            }
            "windows" => {
                println!("- Windows:");
                println!("  Import the certificate via certlm.msc into 'Trusted Root Certification Authorities'.");
            }
            _ => {
                println!("- Unsupported OS:");
                println!("  Please consult your OS documentation on how to add a trusted Root CA.");
            }
        }
        
        println!("\n- Firefox (All platforms):");
        println!("  Settings > Privacy & Security > Certificates > View Certificates > Authorities > Import.");
    }
}