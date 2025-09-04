#!/bin/bash

# Build the hello canister for WebAssembly target

set -e

echo "🔨 Building Hello Canister for WebAssembly..."

# Build the canister
cargo build --release --target wasm32-unknown-unknown

echo "✅ Build complete!"
echo "📦 WebAssembly module location:"
echo "   target/wasm32-unknown-unknown/release/hello_canister.wasm"
echo ""
echo "🧪 To run tests:"
echo "   cargo test"
