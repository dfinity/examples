#!/bin/bash

# Download latest ic artifacts
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/replica.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/canister_sandbox.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/compiler_sandbox.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/ic-admin.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/ic-btc-adapter.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/ic-https-outcalls-adapter.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/ic-nns-init.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/ic-starter.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/sandbox_launcher.gz"
curl -O "https://download.dfinity.systems/ic/$current_sha/binaries/$arch/sns.gz"

# Overwrite artifacts in dfx cache
gzip -d replica.gz && chmod +x replica && mv replica $(dfx cache show)
gzip -d canister_sandbox.gz && chmod +x canister_sandbox && mv canister_sandbox $(dfx cache show)
gzip -d compiler_sandbox.gz && chmod +x compiler_sandbox && mv compiler_sandbox $(dfx cache show)
gzip -d ic-starter.gz && chmod +x ic-starter && mv ic-starter $(dfx cache show)
gzip -d sandbox_launcher.gz && chmod +x sandbox_launcher && mv sandbox_launcher $(dfx cache show)
gzip -d ic-admin.gz && chmod +x ic-admin && mv ic-admin $(dfx cache show)
gzip -d ic-btc-adapter.gz && chmod +x ic-btc-adapter && mv ic-btc-adapter $(dfx cache show)
gzip -d ic-https-outcalls-adapter.gz && chmod +x ic-https-outcalls-adapter && mv ic-https-outcalls-adapter $(dfx cache show)
gzip -d ic-nns-init.gz && chmod +x ic-nns-init && mv ic-nns-init $(dfx cache show)
gzip -d sns.gz && chmod +x sns && mv sns $(dfx cache show)

echo "dfx cache updated with latest ic artifacts"
