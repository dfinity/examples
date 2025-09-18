#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Install Homebrew
curl --location --output install-brew.sh "https://raw.githubusercontent.com/Homebrew/install/master/install.sh"
bash install-brew.sh
rm install-brew.sh

# Install Node.
version=${NODE_VERSION:=14.21.3}
curl --location --output node.pkg "https://nodejs.org/dist/v$version/node-v$version.pkg"
sudo installer -pkg node.pkg -store -target /
rm node.pkg

# Install DFINITY SDK.
curl --location --output install-dfx.sh "https://raw.githubusercontent.com/dfinity/sdk/master/public/install-dfxvm.sh"
DFX_VERSION=${DFX_VERSION:=0.29.1} DFXVM_INIT_YES=true bash install-dfx.sh
rm install-dfx.sh
echo "$HOME/Library/Application Support/org.dfinity.dfx/bin" >> $GITHUB_PATH
source "$HOME/Library/Application Support/org.dfinity.dfx/env"
dfx cache install

# check the current ic-commit found in the main branch, check if it differs from the one in this PR branch
# if so, update the  dfx cache with the latest ic artifacts
if [ -f "${GITHUB_WORKSPACE}/.ic-commit" ]; then
    stable_sha=$(curl https://raw.githubusercontent.com/dfinity/examples/master/.ic-commit)
    current_sha=$(sed <"$GITHUB_WORKSPACE/.ic-commit" 's/#.*$//' | sed '/^$/d')
    arch="x86_64-darwin"
    if [ "$current_sha" != "$stable_sha" ]; then
      export current_sha
      export arch
      sh "$GITHUB_WORKSPACE/.github/workflows/update-dfx-cache.sh"
    fi
fi

# Install ic-repl
version=0.7.7
curl --location --output ic-repl "https://github.com/dfinity/ic-repl/releases/download/$version/ic-repl-macos"
mv ./ic-repl /usr/local/bin/ic-repl
chmod a+x /usr/local/bin/ic-repl

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

# Install wasi2ic
git clone https://github.com/wasm-forge/wasi2ic
cargo install --path wasi2ic --root "${HOME}" --locked

# Install wasm-opt
version=117
curl -fsSLO "https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_${version}-x86_64-macos.tar.gz" 
tar -xzf "binaryen-version_${version}-x86_64-macos.tar.gz" --directory "${HOME}/" --strip-components 1
rm "binaryen-version_${version}-x86_64-macos.tar.gz"

# Exit temporary directory.
popd
