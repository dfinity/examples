#!/usr/bin/env bash
set -e

BUILD_DIR=.dfx/local/canisters/reverse/

if [ ! -d "$BUILD_DIR" ]; then
  dfx canister create reverse
  mkdir -p $BUILD_DIR
  mv reverse.wasm $BUILD_DIR
  rm reverse.o
  dfx canister install --all
fi

dfx canister call reverse go '("repaid")'