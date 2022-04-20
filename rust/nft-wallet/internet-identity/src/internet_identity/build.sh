#!/usr/bin/env bash
set -euo pipefail

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

# keep version in sync with Dockerfile
cargo install ic-cdk-optimizer --version 0.3.1 --root "$II_DIR"/../../target
STATUS=$?

if [ "$STATUS" -eq "0" ]; then
      "$II_DIR"/../../target/bin/ic-cdk-optimizer \
      "$II_DIR/../../target/$TARGET/release/internet_identity.wasm" \
      -o "$II_DIR/../../target/$TARGET/release/internet_identity.wasm"

  true
else
  echo Could not install ic-cdk-optimizer.
  false
fi
