#!/usr/bin/env bash
set -e

echo "=== Building Rust WASMs ==="
cargo build --package caller --target wasm32-unknown-unknown --release
cargo build --package callee --target wasm32-unknown-unknown --release

echo "=== Running multi-subnet PocketIC test ==="
CALLER_WASM=target/wasm32-unknown-unknown/release/caller.wasm \
CALLEE_WASM=target/wasm32-unknown-unknown/release/callee.wasm \
  cargo run --manifest-path multi_subnet/Cargo.toml
