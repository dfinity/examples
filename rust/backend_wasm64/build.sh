#!/bin/bash

echo "Building for wasm64..."

# Check for nightly toolchain
if ! rustup toolchain list | grep -q nightly; then
    echo "Installing nightly toolchain..."
    rustup toolchain install nightly
fi

# Check for rust-src component
if ! rustup component list --toolchain nightly | grep -q "rust-src.*installed"; then
    echo "Adding rust-src component..."
    rustup component add rust-src --toolchain nightly
fi

# Build for wasm64
cargo +nightly build -Z build-std=std,panic_abort --target wasm64-unknown-unknown --release -p backend

# Extract candid interface
candid-extractor target/wasm64-unknown-unknown/release/backend.wasm > backend/backend.did

# Copy wasm file to expected location
cp target/wasm64-unknown-unknown/release/backend.wasm target/backend.wasm 