#!/usr/bin/env bash
set -e
wld="wasm-ld --no-entry --export-dynamic --allow-undefined"
wcc="clang --target=wasm32 -c -O3"
$wcc reverse.c
$wld reverse.o -o reverse.wasm
echo '{"canisters":{"reverse":{"main":"src/reverse"}}}' > dfx.json
