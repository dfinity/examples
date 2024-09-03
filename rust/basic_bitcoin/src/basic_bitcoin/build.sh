#!/usr/bin/env bash
set -euo pipefail

TARGET="wasm32-unknown-unknown"
CANISTER="basic_bitcoin"
SCRIPT_DIR="$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

pushd $SCRIPT_DIR

# NOTE: On macOS a specific version of llvm-ar and clang need to be set here.
# Otherwise the wasm compilation of rust-secp256k1 will fail.
if [ "$(uname)" == "Darwin" ]; then
  LLVM_PATH=$(brew --prefix llvm)
  # On macs we need to use the brew versions
  AR="${LLVM_PATH}/bin/llvm-ar" CC="${LLVM_PATH}/bin/clang" cargo build --target $TARGET --release
else
  cargo build --target $TARGET --release
fi

cargo install ic-wasm --version 0.2.0 --root ./

./bin/ic-wasm \
      "$SCRIPT_DIR/../../target/$TARGET/release/$CANISTER.wasm" \
      -o "$SCRIPT_DIR/../../target/$TARGET/release/$CANISTER.wasm" \
      metadata candid:service -f "$SCRIPT_DIR/basic_bitcoin.did" -v public

popd

