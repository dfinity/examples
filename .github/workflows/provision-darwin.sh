#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Create folder for binaries to add to path
mkdir $HOME/bin

# Install Homebrew
curl --location --output install-brew.sh "https://raw.githubusercontent.com/Homebrew/install/master/install.sh"
bash install-brew.sh
rm install-brew.sh

# Install Node.
version=14.15.4
curl --location --output node.pkg "https://nodejs.org/dist/v$version/node-v$version.pkg"
sudo installer -pkg node.pkg -store -target /
rm node.pkg

# Install DFINITY SDK.
version=0.8.5
curl --location --output install-dfx.sh "https://sdk.dfinity.org/install.sh"
DFX_VERSION=$version bash install-dfx.sh < <(yes Y)
rm install-dfx.sh

# Install Vessel
curl --location --output vessel-macos "https://github.com/dfinity/vessel/releases/download/v0.6.2/vessel-macos"
mv ./vessel-macos $HOME/bin/vessel

# Install Moc
curl --location --output motoko-macos.tar.gz "https://github.com/dfinity/motoko/releases/download/0.6.20/motoko-macos-0.6.20.tar.gz"
gunzip motoko-macos.tar.gz
tar -xvf motoko-macos.tar
mv moc $HOME/bin/moc
rm motoko-macos.tar.gz
rm motoko-macos.tar

# Install cmake
brew install cmake

# Install rust
curl --location --output install-rustup.sh "https://sh.rustup.rs"
bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Update permissions for binaries
chown -R "$(whoami)" $HOME/bin && chmod -R +x $HOME/bin

# Set environment variables.
echo "$HOME/bin" >> $GITHUB_PATH
echo "$HOME/.cargo/bin" >> $GITHUB_PATH

# Exit temporary directory.
popd
