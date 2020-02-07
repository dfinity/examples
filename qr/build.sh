#!/usr/bin/env bash
set -e

if ! [ -a QR-Code-generator]; then
  git clone https://github.com/nayuki/QR-Code-generator
  cd QR-Code-generator
  git checkout fd6917601d0a77b6b6df81599118212d8cdb9a27
  cd ..
fi
wld="wasm-ld-8 --no-entry --export-dynamic --allow-undefined"
wcc="clang-8 --target=wasm32 -c -O3"
$wcc -DNDEBUG -I QR-Code-generator/c QR-Code-generator/c/qrcodegen.c qr.c
$wld qr.o qrcodegen.o -o qr.wasm
echo '{"canisters":{"qr":{"main":"src/qr"}}}' > dfx.json
install -D qr.wasm build/qr/qr.wasm
head -c 8 /dev/urandom > build/qr/_canister.id
