#!/usr/bin/env bash
set -e
cargo build --release --package nft-wallet --target wasm32-unknown-unknown
PATH="$PATH:$PWD/target/bin"
if ! command -v ic-cdk-optimizer &> /dev/null; then
    echo 'ic-cdk-optimizer is not installed; installing it locally. Install it globally to skip this step'
    echo 'This may take a while'
    cargo install --root target ic-cdk-optimizer --version 0.3.4 2> /dev/null
fi
cd target/wasm32-unknown-unknown/release
ic-cdk-optimizer nft_wallet.wasm -o nft_wallet-opt.wasm
