#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Install cmake
brew install cmake

# Install rust
curl --location --output install-rustup.sh "https://sh.rustup.rs"
bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Exit temporary directory.
popd
