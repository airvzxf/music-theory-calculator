# Makefile for Tonic Music Calculator

.PHONY: all build test clean wasm serve

# Default target
all: build

# Build CLI and Core
build:
	cargo build --workspace

# Build Release version of CLI
release:
	cargo build --release -p tonic-music-cli

# Run tests
test:
	cargo test --workspace

# Build Wasm module
wasm:
	cd crates/tonic-music-wasm && wasm-pack build --target web

# Serve Wasm Demo (requires python3)
serve: wasm
	@echo "Serving demo at http://localhost:8000"
	cd crates/tonic-music-wasm && python3 -m http.server 8000

# Generate FFI Bindings and sync to apps
bindings:
	./scripts/generate_bindings.sh

# Build Android Debug APK
android-build: bindings
	cd apps/android && ./gradlew assembleDebug

# Build Android Release Bundle (AAB)
android-release: bindings
	cd apps/android && ./gradlew bundleRelease

# Clean build artifacts
clean:
	cargo clean
	rm -rf crates/tonic-music-wasm/pkg
