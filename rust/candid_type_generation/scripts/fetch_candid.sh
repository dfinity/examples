#!/bin/bash

FILE_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

set -e

echo "========================================="
echo "Fetching Candid interface from NNS Governance canister"
echo "========================================="

# NNS Governance canister ID on mainnet
NNS_GOVERNANCE_CANISTER_ID="rrkah-fqaaa-aaaaa-aaaaq-cai"

# Output file for the Candid interface
OUTPUT_FILE="${FILE_DIR}/../candid/nns_governance.did"
mkdir -p "${FILE_DIR}/../candid"

echo "Fetching Candid interface from canister: $NNS_GOVERNANCE_CANISTER_ID"
echo "Output file: $OUTPUT_FILE"

# Suppress security warnings since we're only reading public metadata
export DFX_WARNING=-mainnet_plaintext_identity

# Fetch the Candid interface metadata from the deployed canister
dfx canister --network ic metadata "$NNS_GOVERNANCE_CANISTER_ID" candid:service > "$OUTPUT_FILE"

# Check if the file was created successfully
if [ -f "$OUTPUT_FILE" ]; then
    echo "✅ Successfully fetched Candid interface!"
    echo "📄 Candid interface saved to: $OUTPUT_FILE"
    echo "📏 File size: $(wc -c < "$OUTPUT_FILE") bytes"
    echo "📊 Line count: $(wc -l < "$OUTPUT_FILE") lines"
else
    echo "❌ Failed to fetch Candid interface"
    exit 1
fi

echo "========================================="
echo "✅ Candid interface fetch complete!"
echo "========================================="
