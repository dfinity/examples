# Threshold ECDSA

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/threshold-ecdsa)

## Overview

This example demonstrates the [threshold ECDSA](https://internetcomputer.org/docs/building-apps/network-features/signatures/t-ecdsa) API on the Internet Computer. The canister acts as a signing oracle: callers can request a threshold ECDSA public key derived from their principal, and sign arbitrary messages using the corresponding private key — without the canister ever holding the key material itself.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/threshold-ecdsa
```

### Deploy and test
```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Key IDs

The canister is configured with `key_id = "test_key_1"` by default (the master test key on mainnet). To use a different environment, update `key_id` in `backend/app.mo`:

- `"dfx_test_key"` — local replica / dfx testing
- `"test_key_1"` — mainnet test key
- `"key_1"` — mainnet production key

## Obtaining public keys

Call `public_key()` to retrieve the ECDSA public key derived for the calling principal. Different callers receive different keys based on their principal used as the derivation path.

## Signing

Call `sign(message)` with any UTF-8 text message. The canister hashes the message with SHA-256 and requests a threshold ECDSA signature from the management canister. The signature can be verified with the public key returned by `public_key()`.

## Signature verification

Example verification in JavaScript using the [secp256k1](https://www.npmjs.com/package/secp256k1) npm package:

```javascript
const { ecdsaVerify } = require("secp256k1");
const crypto = require("crypto");

const public_key = /* Uint8Array from public_key() */;
const message = "hello world";
const message_hash = new Uint8Array(crypto.createHash("sha256").update(message, "utf-8").digest());
const signature = /* Uint8Array from sign(message) */;

const verified = ecdsaVerify(signature, message_hash, public_key);
console.log("verified =", verified); // true
```

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP dapp.
