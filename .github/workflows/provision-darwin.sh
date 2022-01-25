#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

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
version=0.8.4
curl --location --output install-dfx.sh "https://sdk.dfinity.org/install.sh"
DFX_VERSION=$version bash install-dfx.sh < <(yes Y)
rm install-dfx.sh

# Install Moc
curl --location --output motoko-macos.tar.gz "https://github.com/dfinity/motoko/releases/download/0.6.20/motoko-macos-0.6.20.tar.gz"
gunzip motoko-macos.tar.gz
tar -xvf motoko-macos.tar
rm motoko-macos.tar.gz
rm motoko-macos.tar

# Install Vessel and Alias Moc
curl --location --output vessel-macos "https://github.com/dfinity/vessel/releases/download/v0.6.2/vessel-macos"
chown -R "$(whoami)" ./vessel-macos && chmod -R +x ./vessel-macos
alias vessel=$(pwd)/vessel-macos
alias moc=$(vessel bin)/moc
alias mo-doc=$(vessel bin)/mo-doc
alias mo-ide=$(vessel bin)/mo-ide

# Exit temporary directory.
popd
