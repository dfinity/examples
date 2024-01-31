#!/usr/bin/env bash
set -e
for param do
    if [ "$param" = "ic" ] || [ "$param" = "--network=ic" ] ; then
        replicaparam="--replica=https://ic0.app"
        networkparam="--network=ic"
        export NODE_ENV=production
        export DFX_NETWORK=ic
    fi
done
dfx deploy nftwallet "$@"
canister=$(dfx canister ${networkparam:+"$networkparam"} id nftwallet)
PATH="$PATH:$PWD/target/bin"
if ! command -v icx-asset &> /dev/null ; then
    echo 'icx-asset is not installed; installing it locally. Install it globally to skip this step'
    echo 'This may take a while'
    cargo install --root target icx-asset --version 0.20.0 2> /dev/null
fi
identity=$(dfx identity whoami)
pemfile="$HOME/.config/dfx/identity/${identity:-default}/identity.pem"
if [ -e "$pemfile" ] ; then
    pemparam="--pem=$pemfile"
fi
npm run install
npm run build
icx-asset ${pemparam:+"$pemparam"} ${replicaparam:+"$replicaparam"} --ttl 120s sync "$canister" ./frontend/public
