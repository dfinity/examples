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

echo ""

# Step 4: Build with dfx
echo "4️⃣ Building with dfx..."
dfx build

echo ""
echo "✅ Build complete!"
echo "========================================="
echo "🎯 You can now deploy and test the canister:"
echo ""
echo "Deploy:"
echo "  dfx deploy"
echo ""
echo "Test methods:"
echo "  dfx canister call candid_type_generation health"
echo "  dfx canister call candid_type_generation get_info"  
echo "  dfx canister call candid_type_generation list_neurons_pretty"
echo ""
echo "🌐 Or deploy to mainnet to test with real NNS data:"
echo "  dfx deploy --network ic"
echo "========================================="
