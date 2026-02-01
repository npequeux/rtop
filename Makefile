.PHONY: all build release install uninstall clean test clippy fmt run help docker config export

# Default target
all: build

# Build in debug mode
build:
	@echo "Building rtop (debug mode)..."
	cargo build

# Build in release mode (optimized)
release:
	@echo "Building rtop (release mode)..."
	cargo build --release
	@echo ""
	@echo "Build completed!"
	@echo "Binary: target/release/rtop"
	@echo "Size: $$(du -h target/release/rtop | cut -f1)"

# Install to system
install: release
	@echo "Installing rtop to /usr/local/bin..."
	sudo cp target/release/rtop /usr/local/bin/rtop
	@echo "Installation complete! Run 'rtop' to start."

# Install with config
install-with-config: install
	@echo "Generating default configuration..."
	rtop --generate-config
	@echo "Config created at: ~/.config/rtop/config.toml"

# Uninstall from system
uninstall:
	@echo "Uninstalling rtop..."
	sudo rm -f /usr/local/bin/rtop
	@echo "Uninstall complete."

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Clean complete."

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Run clippy (linter)
clippy:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

# Format code
fmt:
	@echo "Formatting code..."
	cargo fmt

# Run the application
run: build
	@echo "Running rtop..."
	cargo run

# Run in release mode
run-release: release
	@echo "Running rtop (release mode)..."
	cargo run --release

# Run with help
run-help: release
	@echo "Showing rtop help..."
	./target/release/rtop --help

# Generate config file
config: release
	@echo "Generating default configuration..."
	./target/release/rtop --generate-config

# Show current config
show-config: release
	@echo "Current rtop configuration:"
	./target/release/rtop show-config

# Export test metrics
export: release
	@echo "Exporting metrics to /tmp/rtop-test.json..."
	./target/release/rtop --export /tmp/rtop-test.json
	@echo "Metrics exported. View with: cat /tmp/rtop-test.json | jq ."

# Build Docker image
docker:
	@echo "Building Docker image..."
	docker build -f Dockerfile.rust -t rtop-rust .
	@echo ""
	@echo "To run: docker run --rm -it --net=\"host\" --pid=\"host\" rtop-rust"

# Check everything (format, lint, test, build)
check: fmt clippy test build
	@echo "All checks passed!"

# Performance benchmark
bench: release
	@echo "Running performance benchmark..."
	@bash bench_perf.sh

# Show help
help:
	@echo "Makefile for rtop v2.0 (Rust)"
	@echo ""
	@echo "Building:"
	@echo "  make build              - Build in debug mode"
	@echo "  make release            - Build in release mode (optimized)"
	@echo ""
	@echo "Installation:"
	@echo "  make install            - Install to /usr/local/bin"
	@echo "  make install-with-config - Install and create config"
	@echo "  make uninstall          - Remove from /usr/local/bin"
	@echo ""
	@echo "Development:"
	@echo "  make test               - Run tests"
	@echo "  make clippy             - Run linter"
	@echo "  make fmt                - Format code"
	@echo "  make check              - Run all checks"
	@echo ""
	@echo "Running:"
	@echo "  make run                - Run in debug mode"
	@echo "  make run-release        - Run in release mode"
	@echo "  make run-help           - Show help"
	@echo ""
	@echo "Configuration:"
	@echo "  make config             - Generate default config"
	@echo "  make show-config        - Display current config"
	@echo ""
	@echo "Utilities:"
	@echo "  make export             - Export test metrics"
	@echo "  make bench              - Run performance benchmark"
	@echo "  make docker             - Build Docker image"
	@echo "  make clean              - Clean build artifacts"
	@echo "  make run           - Build and run (debug)"
	@echo "  make run-release   - Build and run (release)"
	@echo "  make docker        - Build Docker image"
	@echo "  make check         - Run all checks (fmt, clippy, test, build)"
	@echo "  make help          - Show this help message"
