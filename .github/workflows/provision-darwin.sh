#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Install Homebrew
curl --location --output install-brew.sh "https://raw.githubusercontent.com/Homebrew/install/master/install.sh"
bash install-brew.sh
rm install-brew.sh

# Install Node.
version=14.21.3
curl --location --output node.pkg "https://nodejs.org/dist/v$version/node-v$version.pkg"
sudo installer -pkg node.pkg -store -target /
rm node.pkg

# Install DFINITY SDK.
curl --location --output install-dfx.sh "https://internetcomputer.org/install.sh"
DFX_VERSION=${DFX_VERSION:=0.12.0} bash install-dfx.sh < <(yes Y)
rm install-dfx.sh
dfx cache install

# Install ic-repl
version=0.1.2
curl --location --output ic-repl "https://github.com/chenyan2002/ic-repl/releases/download/$version/ic-repl-macos"
mv ./ic-repl /usr/local/bin/ic-repl
chmod a+x /usr/local/bin/ic-repl

# Install cmake
brew install cmake

# Install rust
curl --location --output install-rustup.sh "https://sh.rustup.rs"
bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Install matchers
matchers_version=1.2.0
curl -fsSLO "https://github.com/kritzcreek/motoko-matchers/archive/refs/tags/v${matchers_version}.tar.gz" 
tar -xzf "v${matchers_version}.tar.gz" --directory "$(dfx cache show)"
rm "v${matchers_version}.tar.gz"
mv "$(dfx cache show)/motoko-matchers-${matchers_version}" "$(dfx cache show)/motoko-matchers"

# Install wasmtime
wasmtime_version=0.33.1
curl -fsSLO "https://github.com/bytecodealliance/wasmtime/releases/download/v${wasmtime_version}/wasmtime-v${wasmtime_version}-x86_64-macos.tar.xz" 
mkdir -p "${HOME}/bin"
tar -xf "wasmtime-v${wasmtime_version}-x86_64-macos.tar.xz" --directory "${HOME}/bin/"
mv "${HOME}/bin/wasmtime-v${wasmtime_version}-x86_64-macos/wasmtime" "${HOME}/bin/wasmtime"
rm "wasmtime-v${wasmtime_version}-x86_64-macos.tar.xz"

# Exit temporary directory.
popd
