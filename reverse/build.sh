#!/usr/bin/env bash

wld="wasm-ld-8 --no-entry --export-dynamic --allow-undefined"
wcc="clang --target=wasm32 -c -O3"
$wcc reverse.c
$wld reverse.o -o reverse.wasm
echo '{"canisters":{"reverse":{"main":"src/reverse"}}}' > dfx.json
install -D reverse.wasm build/reverse/reverse.wasm
head -c 8 /dev/urandom > build/reverse/_canister.id
