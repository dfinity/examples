#!/bin/bash
set -eu

# Resolve the script's physical location so we work correctly even when
# frontend/ is reached via a symlink.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
FRONTEND_DIR="$(dirname "$SCRIPT_DIR")"
EXAMPLE_ROOT="$(dirname "$FRONTEND_DIR")"

if command -v candid-extractor >/dev/null 2>&1; then
    (cd "$EXAMPLE_ROOT/backend" && make extract-candid)
fi

rm -rf "$FRONTEND_DIR/src/declarations/basic_timelock_ibe"
mkdir -p "$FRONTEND_DIR/src/declarations/basic_timelock_ibe"
npx --yes @icp-sdk/bindgen \
    --did-file "$EXAMPLE_ROOT/backend/backend.did" \
    --out-dir "$FRONTEND_DIR/src/declarations/basic_timelock_ibe" \
    --declarations-flat --force
