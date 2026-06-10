# Basic VetKD Demo (Motoko)

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/vetkeys/basic_vetkd)

Also available in: [Rust](../../rust/vetkeys/basic_vetkd)

## Overview

Demonstrates the raw VetKD management canister API — the lowest-level building block for verifiably encrypted threshold key derivation on the Internet Computer. The example shows two use cases: symmetric key derivation (AES-GCM-256) and identity-based encryption (IBE), both driven directly via the management canister interface without any SDK abstraction.

For a higher-level approach using the `@icp-sdk/vetkeys` SDK, see the other examples in the `vetkeys/` folder.

## Build and deploy from the command line

### Prerequisites

- [ ] Install Node.js
- [ ] Install icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [ ] Install mops: `npm install -g ic-mops`

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
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

See [https://docs.internetcomputer.org/guides/security/overview](https://docs.internetcomputer.org/guides/security/overview)
