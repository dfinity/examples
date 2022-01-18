#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Install Node.
wget --output-document install-node.sh "https://deb.nodesource.com/setup_14.x"
sudo bash install-node.sh
sudo apt-get install --yes nodejs
rm install-node.sh

# Install DFINITY SDK.
version=0.8.5
wget --output-document install-dfx.sh "https://sdk.dfinity.org/install.sh"
DFX_VERSION=$version bash install-dfx.sh < <(yes Y)
rm install-dfx.sh

# Install cmake
sudo apt-get install --yes cmake

# Install rust
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -y
rustup target add wasm32-unknown-unknown


# Set environment variables.
echo "$HOME/bin" >> $GITHUB_PATH

# Exit temporary directory.
popd
