.PHONY: all build release install uninstall clean test clippy fmt run help docker

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

# Build Docker image
docker:
	@echo "Building Docker image..."
	docker build -f Dockerfile.rust -t rtop-rust .
	@echo ""
	@echo "To run: docker run --rm -it --net=\"host\" --pid=\"host\" rtop-rust"

# Check everything (format, lint, test, build)
check: fmt clippy test build
	@echo "All checks passed!"

# Show help
help:
	@echo "Makefile for rtop (Rust)"
	@echo ""
	@echo "Available targets:"
	@echo "  make build         - Build in debug mode"
	@echo "  make release       - Build in release mode (optimized)"
	@echo "  make install       - Install to /usr/local/bin"
	@echo "  make uninstall     - Remove from /usr/local/bin"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make test          - Run tests"
	@echo "  make clippy        - Run linter"
	@echo "  make fmt           - Format code"
	@echo "  make run           - Build and run (debug)"
	@echo "  make run-release   - Build and run (release)"
	@echo "  make docker        - Build Docker image"
	@echo "  make check         - Run all checks (fmt, clippy, test, build)"
	@echo "  make help          - Show this help message"
