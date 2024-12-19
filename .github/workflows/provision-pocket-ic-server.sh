#!/bin/bash

set -ex

POCKET_IC_SERVER_VERSION=${POCKET_IC_SERVER_VERSION:=6.0.0}
POCKET_IC_SERVER_PATH=${POCKET_IC_SERVER_PATH:="${HOME}/bin/pocket-ic-server"}

if [[ $OSTYPE == "linux-gnu"* ]] || [[ $RUNNER_OS == "Linux" ]]
then
    PLATFORM=linux
elif [[ $OSTYPE == "darwin"* ]] || [[ $RUNNER_OS == "macOS" ]]
then
    PLATFORM=darwin
else
    echo "OS not supported: ${OSTYPE:-$RUNNER_OS}"
    exit 1
fi

if [ ! -f "$POCKET_IC_SERVER_PATH" ]; then
  echo "Downloading PocketIC."
  mkdir -p "$(dirname "${POCKET_IC_SERVER_PATH}")"
  curl -sSL "https://github.com/dfinity/pocketic/releases/download/${POCKET_IC_SERVER_VERSION}/pocket-ic-x86_64-${PLATFORM}.gz" -o "${POCKET_IC_SERVER_PATH}".gz
  gunzip "${POCKET_IC_SERVER_PATH}.gz"
  chmod +x "${POCKET_IC_SERVER_PATH}"
else
    echo "PocketIC server already exists, skipping download."
fi

# Set environment variables.
echo "POCKET_IC_BIN=${POCKET_IC_SERVER_PATH}" >> "$GITHUB_ENV"
