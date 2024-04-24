#!/bin/bash
pushd hosting/photo-storage
# verify frontend deps install and build
npm install
npm run build
# verify that frontend asset canister deploys
dfx start --background
dfx deploy
popd