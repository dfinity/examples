#!/usr/bin/env bash
set -euo pipefail

# Checking for dependencies
# XXX: we currently cannot check for the exact version of ic-cdk-optimizer
# because of https://github.com/dfinity/cdk-rs/issues/181
# Once the issue is fixed, we can ensure that the correct version is installed
if ! command -v ic-cdk-optimizer
then
    echo could not find ic-cdk-optimizer
    echo "ic-cdk-optimizer version 0.3.1 is needed, please run the following command:"
    echo "  cargo install ic-cdk-optimizer --version 0.3.1"
    exit 1
fi

# Compile frontend assets to dist
echo Compiling frontend assets
npm run build

II_DIR="$(dirname "$0")"
TARGET="wasm32-unknown-unknown"

cargo_build_args=(
    --manifest-path "$II_DIR/Cargo.toml"
    --target "$TARGET"
    --release
    -j1
    )

# This enables the "dummy_captcha" feature which makes sure the captcha string
# is always "a".
# WARNING: this MUST be opt-in, because we DO NOT want this in production,
# EVAR.
if [ "${USE_DUMMY_CAPTCHA:-}" == "1" ]
then
    cargo_build_args+=( --features dummy_captcha )
fi

echo Running cargo build "${cargo_build_args[@]}"

cargo build "${cargo_build_args[@]}"

CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$II_DIR/../../target/}"

ic-cdk-optimizer \
    "$CARGO_TARGET_DIR/$TARGET/release/internet_identity.wasm" \
    -o "$II_DIR/../../internet_identity.wasm"
