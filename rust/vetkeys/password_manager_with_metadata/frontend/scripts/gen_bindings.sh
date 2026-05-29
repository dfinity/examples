#!/bin/bash
# Bindings are always generated from the Rust backend since both backends
# expose the same Candid interface.

# Resolve the physical path of this script so that navigating up works
# correctly even when frontend/ is reached via a symlink (e.g. motoko/frontend).
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd -P)
if command -v candid-extractor >/dev/null 2>&1; then
    cd "$SCRIPT_DIR/../../rust/backend" && make extract-candid
fi

cd "$SCRIPT_DIR/../.."
rm -rf frontend/src/declarations/password_manager_with_metadata
mkdir -p frontend/src/declarations/password_manager_with_metadata
npx @icp-sdk/bindgen --did-file rust/backend/backend.did \
    --out-dir frontend/src/declarations/password_manager_with_metadata \
    --declarations-flat --force
