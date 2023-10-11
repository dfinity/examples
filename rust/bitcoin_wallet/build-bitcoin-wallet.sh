#!/usr/bin/env bash
set -xeuo pipefail

TARGET="wasm32-unknown-unknown"

# NOTE: On macOS a specific version of llvm-ar and clang need to be set here.
# Otherwise the wasm compilation of rust-secp256k1 will fail.
if [ "$(uname)" == "Darwin" ]; then
  # On macs we need to use the brew versions
  AR="/usr/local/opt/llvm/bin/llvm-ar" CC="/usr/local/opt/llvm/bin/clang" cargo build -p bitcoin_wallet --target $TARGET --release
else
  cargo build -p bitcoin_wallet --target $TARGET --release
fi

cargo install ic-cdk-optimizer --version 0.3.1 --root ./target
STATUS=$?

if [ "$STATUS" -eq "0" ]; then
      ./target/bin/ic-cdk-optimizer \
      ./target/$TARGET/release/bitcoin_wallet.wasm \
      -o ./target/$TARGET/release/bitcoin-wallet.wasm
  true
else
  echo Could not install ic-cdk-optimizer.
  false
fi

