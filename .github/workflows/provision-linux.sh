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

# Install Vessel and Alias Moc
curl --location --output vessel-linux64 "https://github.com/dfinity/vessel/releases/download/v0.6.2/vessel-linux64"
chown -R "$(whoami)" ./vessel-linux64 && chmod -R +x ./vessel-linux64
alias vessel=$(pwd)/vessel-linux64
alias moc=$(vessel bin)/moc
alias mo-doc=$(vessel bin)/mo-doc
alias mo-ide=$(vessel bin)/mo-ide

# Install cmake
sudo apt-get install --yes cmake

# Install rust
wget --output-document install-rustup.sh "https://sh.rustup.rs"
sudo bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Set environment variables.
echo "$HOME/bin" >> $GITHUB_PATH
echo "$HOME/.cargo/bin" >> $GITHUB_PATH

# Exit temporary directory.
popd
