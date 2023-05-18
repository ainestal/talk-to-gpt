.PHONY: run release install

# Run a conversation, exit with Ctrl-C
run:
	@echo "Running..."
	@cargo run

# Build a release binary
release:
	@echo "Building release binary..."
	@cargo build --release

# Install the release binary
install: release
	@sudo cp target/release/talk-to-gpt /usr/local/bin
	@echo "Installed to /usr/local/bin/talk-to-gpt"
	@echo "Run with 'talk-to-gpt' in any terminal"