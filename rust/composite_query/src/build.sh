#!/bin/bash

set -euo pipefail

BASE="../target/wasm32-unknown-unknown/release"

echo "building data partition canister"

cargo build --target wasm32-unknown-unknown --release -p data_partition --locked
ic-cdk-optimizer ${BASE}/data_partition.wasm --output ${BASE}/data_partition.wasm

(
    echo "compressing data partition canister"
    cd ${BASE}
    gzip -c data_partition.wasm > data_partition.wasm.gz
)

echo "building kv frontend canister"
cargo build --target wasm32-unknown-unknown --release -p kv_frontend --locked
ic-cdk-optimizer ${BASE}/kv_frontend.wasm --output ./kv_frontend.wasm