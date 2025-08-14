#!/usr/bin/bash
set -eo pipefail

rm -rf ./snapshots
mkdir ./snapshots

dfx stop
dfx start --clean --background
dfx deploy

dfx canister call quotes "setup"
dfx canister call quotes "print"

dfx canister stop quotes
dfx canister snapshot create quotes
# snapshot id 0000000000000000ffffffffff9000010101

dfx canister snapshot download --dir ./snapshots quotes 0000000000000000ffffffffff9000010101

# manipulate file
sed -i -e 's/Colour/Color/g' ./snapshots/stable_memory.bin

dfx canister snapshot upload --dir ./snapshots quotes

dfx canister snapshot load quotes 0000000000000001ffffffffff9000010101
dfx canister start quotes
dfx canister call quotes "print"


