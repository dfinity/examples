# Basic VetKD Demo (Motoko)

## Overview

Demonstrates the raw VetKD management canister API — the lowest-level building block for verifiably encrypted threshold key derivation on the Internet Computer. The example shows two use cases: symmetric key derivation (AES-GCM-256) and identity-based encryption (IBE), both driven directly via the management canister interface without any SDK abstraction.

For a higher-level approach using the `@icp-sdk/vetkeys` SDK, see the other examples in the `vetkeys/` folder.

## Build and deploy from the command line

### Prerequisites

- [ ] Install Node.js
- [ ] Install icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [ ] Install mops: `npm install -g ic-mops`

### (Optionally) choose a different master key

This example uses `test_key_1` by default. To use a different [available master key](https://docs.internetcomputer.org/concepts/vetkeys/#api-overview), change the `VETKD_KEY_NAME` environment variable in `icp.yaml` before the first deploy.

Treat it as fixed once the canister holds data. The key name feeds vetKD key derivation, and this canister re-reads the variable on every upgrade, so a changed value takes effect immediately and orphans everything encrypted under the old key.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/vetkeys/basic_vetkd
```

### Deploy

```bash
icp network start -d
icp deploy
```

Open the frontend URL printed by `icp deploy`. When done: `icp network stop`

## Updating the Candid interface

```bash
mops generate candid backend
```

## Security considerations and best practices

See [https://docs.internetcomputer.org/guides/security/overview](https://docs.internetcomputer.org/guides/security/overview)
