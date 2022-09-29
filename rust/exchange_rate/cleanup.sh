#!/bin/bash

# Call the script with cleanup.sh {network}
if [[ $# -lt 1 ]]; then
    echo "Number of arguments supplied not correct. Call this script: \
    ./deploy.sh {env} \
    env should be one of the networks configured in dfx.json."
    exit 1
fi

ENV=$1

# Clean up directory to start new replica instance clean
if [[ $ENV == "local" ]]; then
    # Stop local replica
    dfx stop

    # Remove .dfx folder
    rm -rf .dfx
fi
