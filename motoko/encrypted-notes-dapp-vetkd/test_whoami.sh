#!/bin/bash

if [ $(dfx canister call encrypted_notes_${BUILD_ENV} whoami | grep -e "$(dfx identity get-principal)") ]; then 
    echo "Test passed!"; 
else
    echo "Test failed :-("
    exit 1
fi
