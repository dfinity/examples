#!/bin/bash

# Bindings are always generated from the Rust backend since both backends
# expose the same Candid interface.

cd ../..
rm -rf frontend/src/declarations/encrypted_notes
mkdir -p frontend/src/declarations/encrypted_notes
npx @icp-sdk/bindgen --did-file rust/backend/src/encrypted_notes_rust.did --out-dir frontend/src/declarations/encrypted_notes --declarations-flat --force
