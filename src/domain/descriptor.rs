use crate::types::DomainConfig;

pub struct DomainDescriptor;

impl DomainDescriptor {
    pub fn print(domain: &DomainConfig) {
        let short_id: String = domain.id.chars().take(8).collect();
        
        println!("[DOMAIN DETAILS]");
        println!("{:<20} {}", "ID:", short_id);
        println!("{:<20} {}", "Name:", domain.name);
        println!("{:<20} {}", "Domain:", domain.domain);
        println!("{:<20} {}", "IP Address:", domain.ip);
        println!("{:<20} {}", "Created At:", domain.created_at.format("%Y-%m-%d %H:%M:%S"));
        
        println!("\n[CERTIFICATES]");
        println!("{:<20} {}", "Certificate Path:", domain.cert_path.display());
        println!("{:<20} {}", "Private Key Path:", domain.key_path.display());
        
        println!("\n[SERVER CONFIGURATION EXAMPLES]");
        println!("Use these absolute paths in your local web servers to enable HTTPS.");

        println!("\n-- Nginx --");
        println!("server {{");
        println!("    listen 443 ssl;");
        println!("    server_name {};", domain.domain);
        println!("    ssl_certificate {};", domain.cert_path.display());
        println!("    ssl_certificate_key {};", domain.key_path.display());
        println!("    ...");
        println!("}}");

        println!("\n-- Apache --");
        println!("<VirtualHost *:443>");
        println!("    ServerName {}", domain.domain);
        println!("    SSLEngine on");
        println!("    SSLCertificateFile \"{}\"", domain.cert_path.display());
        println!("    SSLCertificateKeyFile \"{}\"", domain.key_path.display());
        println!("    ...");
        println!("</VirtualHost>");

        println!("\n-- Node.js (Express / HTTPS) --");
        println!("const https = require('https');");
        println!("const fs = require('fs');");
        println!("const options = {{");
        println!("    key: fs.readFileSync('{}'),", domain.key_path.display());
        println!("    cert: fs.readFileSync('{}')", domain.cert_path.display());
        println!("}};");
        println!("https.createServer(options, app).listen(443);");

        println!("\n-- Python (FastAPI / Uvicorn) --");
        println!("uvicorn main:app --port 443 \\");
        println!("  --ssl-keyfile \"{}\" \\", domain.key_path.display());
        println!("  --ssl-certfile \"{}\"", domain.cert_path.display());
        
        println!("\n-- Go (net/http) --");
        println!("log.Fatal(http.ListenAndServeTLS(\":443\", \"{}\", \"{}\", handler))", 
            domain.cert_path.display(), 
            domain.key_path.display()
        );
    }
}