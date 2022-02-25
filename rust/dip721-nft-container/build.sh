#!/usr/bin/env sh
set -e
cargo build --package dip721-nft-container --release --target wasm32-unknown-unknown
PATH="$PATH:$PWD/target/bin"
if ! command -v ic-cdk-optimizer &> /dev/null; then
    echo 'ic-cdk-optimizer is not installed; installing it locally. Install it globally to skip this step'
    echo 'This may take a while'
    cargo install ic-cdk-optimizer --root target 2> /dev/null
fi
cd target/wasm32-unknown-unknown/release
ic-cdk-optimizer dip721_nft_container.wasm -o dip721_nft_container-opt.wasm
