#!/bin/bash

set -ex

POCKET_IC_SERVER_VERSION=${POCKET_IC_SERVER_VERSION:=4.0.0}
POCKET_IC_SERVER_PATH=${POCKET_IC_SERVER_PATH:="${HOME}/bin/pocket-ic"}

echo "Downloading PocketIC."
mkdir -p "$(dirname "${POCKET_IC_SERVER_PATH}")"
curl -sSL "https://github.com/dfinity/pocketic/releases/download/${POCKET_IC_SERVER_VERSION}/pocket-ic-x86_64-darwin.gz" -o ${POCKET_IC_SERVER_PATH}.gz
gunzip ${POCKET_IC_SERVER_PATH}.gz
chmod +x ${POCKET_IC_SERVER_PATH}

# Set environment variables.
echo "POCKET_IC_BIN=${POCKET_IC_SERVER_PATH}" >> "$GITHUB_ENV"