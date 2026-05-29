#!/bin/bash

# Bindings are always generated from the Rust backend since both backends
# expose the same Candid interface.
if command -v candid-extractor >/dev/null 2>&1; then
    cd ../../rust/backend && make extract-candid
    cd ../..
else
    cd ../..
fi

rm -rf frontend/src/declarations/basic_bls_signing

mkdir -p frontend/src/declarations/basic_bls_signing
npx @icp-sdk/bindgen --did-file rust/backend/backend.did --out-dir frontend/src/declarations/basic_bls_signing --declarations-flat --force
