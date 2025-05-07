#!/bin/bash

TARGET=${WASM_TARGET:-wasm32}  # Use WASM_TARGET env var, default to wasm32

if [ "$TARGET" = "wasm64" ]; then
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
    cargo +nightly build -Z build-std=std,panic_abort --target wasm64-unknown-unknown --release -p backend
    candid-extractor target/wasm64-unknown-unknown/release/backend.wasm > backend/backend.did
    cp target/wasm64-unknown-unknown/release/backend.wasm target/backend.wasm
else
    echo "Building for wasm32..."
    cargo build --target wasm32-unknown-unknown --release -p backend
    candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
    cp target/wasm32-unknown-unknown/release/backend.wasm target/backend.wasm
fi 