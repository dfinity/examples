#!/bin/bash

set -x

MOC=${MOC:-"$(dfx cache show)"/moc}
MATCHERS=${MATCHERS:-"$(dfx cache show)"/motoko-matchers/src}
WASMTIME=${WASMTIME:-wasmtime}
WASMTIME_OPTIONS="--disable-cache"
DIR="$(pwd)/$(dirname "$0")"
PKGS="--package base $(dfx cache show)/base --package matchers ${MATCHERS}"

${MOC} -c ${PKGS} -wasi-system-api -o "${DIR}/test.wasm" "${DIR}/test.mo"
${WASMTIME} run ${WASMTIME_OPTIONS} "${DIR}/test.wasm"