# Certified variables

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/cert-var)

## Overview

This example demonstrates how to use **certified variables** on the Internet Computer. The backend canister holds a single 32-bit counter and keeps its value in sync with the system-level certified-data hash. Clients that call `get` as a query can cryptographically verify the returned value against the IC root key — without waiting for a full consensus round — by checking the embedded certificate.

The example shows:

- Using `mo:core/CertifiedData` to set and expose a certified hash alongside query responses.
- Encoding a `Nat32` as a little-endian blob so that frontend code can decode it directly with the Candid codec.
- Structuring mutating operations (`inc`, `set`) to always update the certificate immediately after changing state.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/cert-var
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.

This example specifically demonstrates [certifying query responses](https://docs.internetcomputer.org/guides/security/overview), which is especially important when query results influence security-relevant decisions.
