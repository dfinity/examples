#!/usr/bin/env bash
set -e
wld="wasm-ld-8 --no-entry --export-dynamic --allow-undefined"
wcc="clang-8 --target=wasm32 -c -O3"
$wcc -DNDEBUG -I QR-Code-generator/c QR-Code-generator/c/qrcodegen.c qr.c
$wld qr.o qrcodegen.o -o qr.wasm
echo '{"canisters":{"qr":{"main":"src/qr"}}}' > dfx.json
install -D qr.wasm build/qr/qr.wasm
head -c 8 /dev/urandom > build/qr/_canister.id
