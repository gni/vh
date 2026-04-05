# vh (Virtual Host Manager)

A simple, production-grade CLI tool for managing local development domains with trusted SSL certificates.

## Installation

Ensure you have Rust and Cargo installed, then run:

From crates:
```bash
cargo install vh
````

Local:

```bash
cargo build --release # Check the Makefile
```

## Quick Start

**1. Create a local domain**
This creates `myapp.test`, adds it to `/etc/hosts`, and generates SSL certificates.

```bash
vh create myapp
```

*Want a specific IP or TLD?*

```bash
vh create api.loc -i 127.0.0.5
```

**2. Trust the Local CA**
Get the green padlock in your browser by trusting the automatically generated Root CA.

```bash
vh ca
```

*(Follow the printed OS-specific instructions to add the CA to your system's trust store).*

**3. Configure your server**
Get the exact certificate paths and copy-paste configuration for Nginx, Apache, Node.js, Python, or Go.

```bash
vh describe myapp
```

## Usage

```bash
vh create <domain>        # Create a new domain (default extension: .test)
vh list                   # List all managed domains
vh describe <identifier>  # Show details and web server config snippets
vh remove <identifier>    # Remove a domain and clean up /etc/hosts
vh ca                     # Show Root CA paths and trust instructions
```

### Managing Extensions

By default, `vh` only allows safe local extensions (like `.test`, `.localhost`, `.local`) to prevent hijacking real internet traffic. If you know what you are doing, you can allow custom extensions:

```bash
vh extension allow loc    # Allow .loc domains
vh extension list         # View custom allowed extensions
vh extension remove loc   # Remove .loc from allowed list
```

### Shell Completions

Generate autocomplete scripts for your shell:

```bash
vh completions bash > ~/.local/share/bash-completion/completions/vh
```

## How it works

1.  **Hosts Routing:** Safely edits `/etc/hosts` to point your custom domain to localhost (or a specified IP).
2.  **Local CA:** Automatically generates a dedicated Root Certificate Authority in `~/.local/share/vh/`.
3.  **SSL Certificates:** Issues domain-specific Subject Alternative Name (SAN) certificates signed by the local CA, ensuring modern browsers accept them without warnings once the root is trusted.

-----

**Author:** Lucian BLETAN  
**Repository:** [https://github.com/gni/vh](https://www.google.com/search?q=https://github.com/gni/vh)
