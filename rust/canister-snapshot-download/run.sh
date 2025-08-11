#!/usr/bin/bash
set -eo pipefail

rm -rf ./snapshots
mkdir ./snapshots

dfx stop
dfx start --clean --background
dfx deploy

dfx canister call uxrrr-q7777-77774-qaaaq-cai "setup"
dfx canister call uxrrr-q7777-77774-qaaaq-cai "print"

dfx canister stop uxrrr-q7777-77774-qaaaq-cai
dfx canister snapshot create uxrrr-q7777-77774-qaaaq-cai
# snapshot id 0000000000000000ffffffffff9000010101

dfx canister snapshot download --dir ./snapshots uxrrr-q7777-77774-qaaaq-cai 0000000000000000ffffffffff9000010101

# manipulate file
sed -i -e 's/Colour/Color/g' ./snapshots/stable_memory.bin

dfx canister snapshot upload --dir ./snapshots uxrrr-q7777-77774-qaaaq-cai

dfx canister snapshot load uxrrr-q7777-77774-qaaaq-cai 0000000000000001ffffffffff9000010101
dfx canister start uxrrr-q7777-77774-qaaaq-cai
dfx canister call uxrrr-q7777-77774-qaaaq-cai "print"


