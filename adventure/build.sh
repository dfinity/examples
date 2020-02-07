#!/usr/bin/env bash
set -e
wld="wasm-ld-8 --no-entry --export-dynamic --allow-undefined"
wcc="clang-8 --target=wasm32 -c -O3"
tar xfz Adventure2.5.tar.gz
patch < adv.patch
(echo "char*textFile(){static char s[]=" ; cat adventure.text | sed 's/"/\\"/g' | sed 's/.*/"&\\n"/'; echo ";return s;}") > adventure.text.c
# Suppress warnings. The original source is old.
$wcc -Wno-everything *.c
$wld *.o -o adventure.wasm
echo '{"canisters":{"adventure":{"main":"src/adventure"}}}' > dfx.json
install -D adventure.wasm build/adventure/adventure.wasm
head -c 8 /dev/urandom > build/adventure/_canister.id
