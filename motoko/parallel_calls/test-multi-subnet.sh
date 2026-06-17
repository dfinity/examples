#!/usr/bin/env bash
set -e

echo "=== Building Motoko WASMs ==="
icp build

echo "=== Running multi-subnet PocketIC test ==="
CALLER_WASM=.mops/.build/caller.wasm \
CALLEE_WASM=.mops/.build/callee.wasm \
  cargo run --manifest-path multi_subnet/Cargo.toml
