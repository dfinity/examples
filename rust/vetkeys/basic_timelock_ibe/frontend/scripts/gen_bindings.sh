#!/bin/bash

# Generate candid bindings from the backend.
if command -v candid-extractor >/dev/null 2>&1; then
    cd ../backend && make extract-candid
    cd ..
else
    cd ..
fi

rm -rf frontend/src/declarations/basic_timelock_ibe

mkdir -p frontend/src/declarations/basic_timelock_ibe
npx @icp-sdk/bindgen --did-file backend/backend.did --out-dir frontend/src/declarations/basic_timelock_ibe --declarations-flat --force
