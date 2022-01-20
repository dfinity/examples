#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Install cmake
sudo apt-get install --yes cmake

# Install rust
wget --output-document install-rustup.sh "https://sh.rustup.rs"
sudo bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Set environment variables.
echo "$HOME/.cargo/bin" >> $GITHUB_PATH

# Exit temporary directory.
popd
