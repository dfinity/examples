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
version=0.9.0
wget --output-document install-dfx.sh "https://sdk.dfinity.org/install.sh"
DFX_VERSION=$version bash install-dfx.sh < <(yes Y)
rm install-dfx.sh
dfx cache install

# Install cmake
sudo apt-get install --yes cmake

# Install rust
wget --output-document install-rustup.sh "https://sh.rustup.rs"
sudo bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Install wasmtime
wasmtime_version=0.33.1
curl -fsSLO "https://github.com/bytecodealliance/wasmtime/releases/download/v${wasmtime_version}/wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz" 
mkdir -p "${HOME}/bin"
tar -xf "wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz" --directory "${HOME}/bin/"
mv "${HOME}/bin/wasmtime-v${wasmtime_version}-x86_64-linux/wasmtime" "${HOME}/bin/wasmtime"
rm "wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz"

# Set environment variables.
echo "$HOME/bin" >> $GITHUB_PATH
echo "$HOME/.cargo/bin" >> $GITHUB_PATH

# Exit temporary directory.
popd
