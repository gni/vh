.PHONY: build test lint clean install release

BINARY_NAME=vh
INSTALL_DIR=/usr/local/bin

build:
	cargo build

release:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy -- -D warnings
	cargo fmt -- --check

clean:
	cargo clean

install: release
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	sudo cp target/release/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Installation complete."

completions: release
	@mkdir -p completions
	target/release/$(BINARY_NAME) completions bash > completions/vh.bash
	target/release/$(BINARY_NAME) completions zsh > completions/_vh
	target/release/$(BINARY_NAME) completions fish > completions/vh.fish
	@echo "Completions generated in ./completions/"