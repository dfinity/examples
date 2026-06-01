#!/bin/bash
set -eu

# Resolve the script's physical location so we work correctly even when the
# icp CLI has symlinked `frontend/` into a backend subdirectory for the build.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
FRONTEND_DIR="$(dirname "$SCRIPT_DIR")"
EXAMPLE_ROOT="$(dirname "$FRONTEND_DIR")"

# Bindings are always generated from the Rust backend since both backends
# expose the same Candid interface.

rm -rf "$FRONTEND_DIR/src/declarations/encrypted_notes"
mkdir -p "$FRONTEND_DIR/src/declarations/encrypted_notes"
npx --yes @icp-sdk/bindgen \
    --did-file "$EXAMPLE_ROOT/rust/backend/src/encrypted_notes_rust.did" \
    --out-dir "$FRONTEND_DIR/src/declarations/encrypted_notes" \
    --declarations-flat --force
