#!/bin/bash
set -eu

# Resolve the script's physical location so we work correctly even when the
# icp CLI has symlinked `frontend/` into a backend subdirectory for the build.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
FRONTEND_DIR="$(dirname "$SCRIPT_DIR")"
EXAMPLE_ROOT="$(dirname "$FRONTEND_DIR")"

# Bindings are always generated from the Rust backend since both backends
# expose the same Candid interface.
if command -v candid-extractor >/dev/null 2>&1; then
    (cd "$EXAMPLE_ROOT/rust/backend" && make extract-candid)
fi

rm -rf "$FRONTEND_DIR/src/declarations/basic_ibe"
mkdir -p "$FRONTEND_DIR/src/declarations/basic_ibe"
npx @icp-sdk/bindgen \
    --did-file "$EXAMPLE_ROOT/rust/backend/backend.did" \
    --out-dir "$FRONTEND_DIR/src/declarations/basic_ibe" \
    --declarations-flat --force
