ARG BUILD_ENV=motoko

FROM node:17-slim as encrypted_notes_base

# Install a basic environment needed for our build tools
# build-essential only necessary if you need to build the Rust canister.
RUN \
    apt -yq update && \
    apt -yqq install --no-install-recommends curl rsync ca-certificates libdigest-sha-perl

# Install dfx; the version is picked up from the DFX_VERSION environment variable
# Lowercase [dfx_version] is an argument of this Dockerfile (with a default value)
# Uppercase [DFX_VERSION] is an environment variable for expected by the DFX installation script
ARG dfx_version=0.9.3
ENV DFX_VERSION=${dfx_version}
RUN sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
ENV NODE_OPTIONS=--openssl-legacy-provider
EXPOSE 8080 8000 3000 35729
WORKDIR /canister
ENTRYPOINT /bin/bash

# Motoko image is the base image plus testing-related packages
FROM encrypted_notes_base as encrypted_notes_motoko

ONBUILD RUN apt -yqq install --no-install-recommends xz-utils

ONBUILD ARG matchers_version=1.2.0
ONBUILD RUN curl -fsSLO "https://github.com/kritzcreek/motoko-matchers/archive/refs/tags/v${matchers_version}.tar.gz" && \
    tar -xzf "v${matchers_version}.tar.gz" && \
    rm "v${matchers_version}.tar.gz"
ONBUILD ENV MATCHERS="/canister/motoko-matchers-${matchers_version}/src"

ONBUILD ARG wasmtime_version=0.33.1
ONBUILD RUN curl -fsSLO "https://github.com/bytecodealliance/wasmtime/releases/download/v${wasmtime_version}/wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz" && \
    tar -xf "wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz" && \
    rm -f /bin/wasmtime && \
    ln -s "/canister/wasmtime-v${wasmtime_version}-x86_64-linux/wasmtime" /bin/wasmtime && \
    rm "wasmtime-v${wasmtime_version}-x86_64-linux.tar.xz"

# Install Rust and Cargo in /opt
# Specify the Rust toolchain version
# This is necessary only if you are going to be running the Rust version of the canister.

# Rust image is the base image plus build-essential, plus rustup
FROM encrypted_notes_base as encrypted_notes_rust

# The ONBUILD instructions ensure that the Rust-specific commands are only executed if this
ONBUILD RUN apt -yqq install --no-install-recommends build-essential
ONBUILD ARG rust_version=1.54.0
ONBUILD ENV RUSTUP_HOME=/opt/rustup \
            CARGO_HOME=/opt/cargo \
            PATH=/opt/cargo/bin:$PATH
ONBUILD RUN curl --fail https://sh.rustup.rs -sSf \
    | sh -s -- -y --default-toolchain ${rust_version}-x86_64-unknown-linux-gnu --no-modify-path && \
    rustup default ${rust_version}-x86_64-unknown-linux-gnu && \
    rustup target add wasm32-unknown-unknown

# Choose which image version to build: "motoko" or "rust"? 
FROM encrypted_notes_${BUILD_ENV}