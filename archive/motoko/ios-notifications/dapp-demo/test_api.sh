#!/bin/bash

if echo $(dfx canister call ios_notifications_api whoami) | grep -q -e "$(dfx identity get-principal)"; then 
    echo "Test passed!"; 
else
    echo "Test failed :-("
    exit 1
fi
