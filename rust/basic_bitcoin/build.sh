#!/usr/bin/env bash
set -euo pipefail

TARGET="wasm32-unknown-unknown"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

# Change to the script directory
cd "$SCRIPT_DIR"

# Build based on the platform. On MacOS, use LLVM's clang and llvm-ar
# to avoid issues with the default clang and ar. See more info in the README.
if [ "$(uname)" == "Darwin" ]; then
  LLVM_PATH=$(brew --prefix llvm)
  AR="${LLVM_PATH}/bin/llvm-ar" CC="${LLVM_PATH}/bin/clang" cargo build --target $TARGET --release
else
  cargo build --target $TARGET --release
fi