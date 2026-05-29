#!/bin/bash

# Bindings are always generated from the Rust backend since both backends
# expose the same Candid interface.
if command -v candid-extractor >/dev/null 2>&1; then
    cd ../../rust/backend && make extract-candid
    cd ../..
else
    cd ../..
fi

rm -rf frontend/src/declarations/basic_ibe

mkdir -p frontend/src/declarations/basic_ibe
npx @icp-sdk/bindgen --did-file rust/backend/backend.did --out-dir frontend/src/declarations/basic_ibe --declarations-flat --force
