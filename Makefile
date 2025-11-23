# Makefile for Tonic Music Calculator

.PHONY: help all build release test wasm serve bindings bindings-debug android-build android-release clean

help: ## List all available commands
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

all: build ## Default target

build: ## Build CLI and Core
	cargo build --workspace

release: ## Build Release version of CLI
	cargo build --release -p tonic-music-cli

test: ## Run tests
	cargo test --workspace

wasm: ## Build Wasm module
	cd crates/tonic-music-wasm && wasm-pack build --target web

serve: wasm ## Serve Wasm Demo (requires python3)
	@echo "Serving demo at http://localhost:8000"
	cd crates/tonic-music-wasm && python3 -m http.server 8000

bindings: ## Generate FFI Bindings (Release) and sync to apps
	./scripts/generate_bindings.sh

bindings-debug: ## Generate FFI Bindings (Debug)
	./scripts/generate_bindings.sh --debug

android-build: bindings-debug ## Build Android Debug APK (uses Debug bindings)
	cd apps/android && ./gradlew assembleDebug

android-release: bindings ## Build Android Release Bundle (AAB) (uses Release bindings)
	cd apps/android && ./gradlew bundleRelease
	@echo "App Bundle created at: apps/android/app/build/outputs/bundle/release/app-release.aab"

clean: ## Clean build artifacts
	cargo clean
	rm -rf crates/tonic-music-wasm/pkg
