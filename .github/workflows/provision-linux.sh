#!/bin/bash

set -ex

# Enter temporary directory.
pushd /tmp

# Install Node.
wget --output-document nodesource_setup.sh "https://deb.nodesource.com/setup_lts.x"
sudo bash nodesource_setup.sh
sudo apt-get install --yes nodejs
rm nodesource_setup.sh

# Install DFINITY SDK.
wget --output-document install-dfx.sh "https://raw.githubusercontent.com/dfinity/sdk/master/public/install-dfxvm.sh"
DFX_VERSION=${DFX_VERSION:=0.29.1} DFXVM_INIT_YES=true bash install-dfx.sh
rm install-dfx.sh
echo "$HOME/.local/share/dfx/bin" >>$GITHUB_PATH
source "$HOME/.local/share/dfx/env"
dfx cache install
# check the current ic-commit found in the main branch, check if it differs from the one in this PR branch
# if so, update the  dfx cache with the latest ic artifacts
if [ -f "${GITHUB_WORKSPACE}/.ic-commit" ]; then
  stable_sha=$(curl https://raw.githubusercontent.com/dfinity/examples/master/.ic-commit)
  current_sha=$(sed <"$GITHUB_WORKSPACE/.ic-commit" 's/#.*$//' | sed '/^$/d')
  arch="x86_64-linux"
  if [ "$current_sha" != "$stable_sha" ]; then
    export current_sha
    export arch
    sh "$GITHUB_WORKSPACE/.github/workflows/update-dfx-cache.sh"
  fi
fi

# Install ic-repl
version=0.7.7
curl --location --output ic-repl "https://github.com/dfinity/ic-repl/releases/download/$version/ic-repl-linux64"
mv ./ic-repl /usr/local/bin/ic-repl
chmod a+x /usr/local/bin/ic-repl

# Install cmake
sudo apt-get install --yes cmake

# Install rust
wget --output-document install-rustup.sh "https://sh.rustup.rs"
sudo bash install-rustup.sh -y
rustup target add wasm32-unknown-unknown

# Install matchers
matchers_version=1.2.0
curl -fsSLO "https://github.com/kritzcreek/motoko-matchers/archive/refs/tags/v${matchers_version}.tar.gz"
tar -xzf "v${matchers_version}.tar.gz" --directory "$(dfx cache show)"
rm "v${matchers_version}.tar.gz"
mv "$(dfx cache show)/motoko-matchers-${matchers_version}" "$(dfx cache show)/motoko-matchers"

# Install wasmtime
wasmtime_version=0.33.1
curl -fsSLO "https://github.com/bytecodealliance/wasmtime/releases/download/v${wasmtime_version}/wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz"
mkdir -p "${HOME}/bin"
tar -xf "wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz" --directory "${HOME}/bin/"
mv "${HOME}/bin/wasmtime-v${wasmtime_version}-x86_64-linux/wasmtime" "${HOME}/bin/wasmtime"
rm "wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz"

# Install wasi2ic
git clone https://github.com/wasm-forge/wasi2ic
cargo install --path wasi2ic --root "${HOME}" --locked

# Install wasm-opt
version=117
curl -fsSLO "https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_${version}-x86_64-linux.tar.gz"
tar -xzf "binaryen-version_${version}-x86_64-linux.tar.gz" --directory "${HOME}/" --strip-components 1
rm "binaryen-version_${version}-x86_64-linux.tar.gz"

# Set environment variables.
echo "$HOME/bin" >>$GITHUB_PATH
echo "$HOME/.cargo/bin" >>$GITHUB_PATH

# Exit temporary directory.
popd
