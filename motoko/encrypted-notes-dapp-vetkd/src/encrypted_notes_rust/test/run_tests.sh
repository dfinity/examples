#!/bin/bash

set -x

dir="$(pwd)/$(dirname "$0")"
cargo test --manifest-path "${dir}/../Cargo.toml"