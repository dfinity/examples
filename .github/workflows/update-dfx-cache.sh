#!/bin/bash

current_sha=$(sed <"$GITHUB_WORKSPACE/.ic-commit" 's/#.*$//' | sed '/^$/d')

# Download latest ic artifacts
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/replica.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/canister_sandbox.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/ic-admin.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/ic-btc-adapter.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/ic-https-outcalls-adapter.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/ic-nns-init.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/ic-starter.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/sandbox_launcher.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/x86_64-linux/sns.gz"

# Overwrite artifacts in dfx cache
gzip -d replica.gz
mv replica $(dfx cache show)
gzip -d canister_sandbox.gz
mv canister_sandbox $(dfx cache show)
gzip -d ic-admin.gz
mv ic-admin $(dfx cache show)
gzip -d ic-btc-adapter.gz
mv ic-btc-adapter $(dfx cache show)
gzip -d ic-https-outcalls-adapter.gz
mv ic-https-outcalls-adapter $(dfx cache show)
gzip -d ic-nns-init.gz
mv ic-nns-init $(dfx cache show)
gzip -d ic-starter.gz
mv ic-starter $(dfx cache show)
gzip -d sandbox_launcher.gz
mv sandbox_launcher $(dfx cache show)
gzip -d sns.gz
mv sns $(dfx cache show)