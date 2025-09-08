#!/bin/bash

set -e

echo "🚀 Building Candid Type Generation Example"
echo "========================================="

# Step 1: Fetch Candid interface from NNS Governance
echo "1️⃣ Fetching Candid interface..."
./scripts/fetch_candid.sh

echo ""

# Step 2: Generate Rust types from Candid file
echo "2️⃣ Generating Rust types..."
./scripts/generate_types.sh

echo ""

# Step 3: Build the canister
echo "3️⃣ Building the canister..."
echo "🔧 Running cargo build..."
cargo build --target wasm32-unknown-unknown --release
