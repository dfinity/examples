#!/usr/bin/env bash
set -e
dfx canister create reverse
cp reverse.wasm .dfx/local/canisters/
dfx canister install --all
dfx canister call reverse go '("repaid")'
