# Certified variables

This example demonstrates how to use **certified variables** on the Internet Computer. The backend canister holds a single 32-bit counter and keeps its value in sync with the system-level certified-data hash. Clients can call `get` as a query and cryptographically verify the returned value against the IC root key — without waiting for a full consensus round — by checking the embedded certificate.

The frontend demonstrates the entire client-side verification flow in the browser, making it clear what it takes to trust a fast query response the same way you would trust a slower update call.

## How it works

When you click "Set and get!", the frontend performs four checks on the query response:

1. **Verify system certificate** — The IC signs a certificate over the canister's certified data tree. `Certificate.create()` verifies this BLS signature against the network's root key.
2. **Check timestamp** — The certificate contains the IC's current time (LEB128-encoded). The frontend decodes it and confirms the certificate is fresh (within 5 seconds of the client's clock).
3. **Check canister ID** — The certificate's state tree contains a path `canister/<id>/certified_data`. The frontend looks up the canister ID to confirm the certificate covers our specific canister.
4. **Check certified data** — The 4-byte little-endian blob stored in certified data is decoded as a Candid `Nat32` and compared against the `value` field in the query response. If they match, the response is authentic.

These four checks are sufficient for a single-variable canister. More complex examples would additionally re-compute a Merkle witness and verify query parameters against the witness.

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
bash test.sh
icp network stop
```

`bash test.sh` verifies **functional correctness** only — that values are stored and returned correctly. Cryptographic certificate verification (confirming the query response is authentically signed by the IC) is performed in the browser frontend, not the CLI tests. The icp-cli does not verify certificates on your behalf.

For local frontend development with hot reload:

```bash
npm run dev --prefix frontend
```

## Updating the Candid interface

If you change the Motoko source, regenerate `backend/backend.did`:

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.

This example specifically demonstrates [certifying query responses](https://docs.internetcomputer.org/guides/security/overview), which is especially important when query results influence security-relevant decisions.
