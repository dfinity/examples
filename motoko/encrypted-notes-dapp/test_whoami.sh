#!/usr/bin/sh

if [ $(dfx canister call encrypted_notes_motoko whoami | grep -e "blabla") ]; then 
    echo "Test passed!"; 
else 
    echo "Test failed :-("
    exit 1
fi