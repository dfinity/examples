#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Create folder for binaries to add to path
mkdir $HOME/bin

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

# Install Vessel
curl --location --output vessel-linux64 "https://github.com/dfinity/vessel/releases/download/v0.6.2/vessel-linux64"
mv ./vessel-linux64 $HOME/bin/vessel

# Install Moc
curl --location --output /tmp/motoko-linux64.tar.gz "https://github.com/dfinity/motoko/releases/download/0.6.20/motoko-linux64-0.6.20.tar.gz"
gunzip motoko-linux64.tar.gz
chown -R "$(whoami)" /tmp && chmod -R +x /tmp
tar -xvf ./motoko-linux64.tar
mv /tmp/moc $HOME/bin/moc
rm /tmp/motoko-linux64.tar.gz
rm /tmp/motoko-linux64.tar

# Install cmake
sudo apt-get install --yes cmake

# Install rust
wget --output-document install-rustup.sh "https://sh.rustup.rs"
sudo bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Update permissions for binaries
chown -R "$(whoami)" $HOME/bin && chmod -R +x $HOME/bin

# Set environment variables.
echo "$HOME/bin" >> $GITHUB_PATH
echo "$HOME/.cargo/bin" >> $GITHUB_PATH

# Exit temporary directory.
popd
