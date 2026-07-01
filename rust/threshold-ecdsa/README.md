# Threshold ECDSA

This example demonstrates threshold ECDSA signing, part of ICP's [chain-key cryptography](https://docs.internetcomputer.org/concepts/chain-key-cryptography/#chain-key-signatures-threshold-ecdsa-and-schnorr). The canister acts as a signing oracle: callers can request a threshold ECDSA public key derived from their principal, and sign arbitrary messages using the corresponding private key — without the canister ever holding the key material itself.

See the [Motoko version](../../motoko/threshold-ecdsa) for a comparison.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/rust/threshold-ecdsa
```

### Deploy and test
```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## Key IDs

The canister is configured with `KEY_NAME = "test_key_1"` by default (the master test key, works on both the local network and mainnet). To use the production key, update `KEY_NAME` in `backend/lib.rs`:

- `"test_key_1"` — default, mainnet test key
- `"key_1"` — mainnet production key

## Obtaining public keys

Call `public_key()` to retrieve the ECDSA public key derived for the calling principal. The canister uses the caller's principal bytes as the derivation path, so different callers receive different keys.

### Key derivation

To obtain a key below the root in the BIP-32 hierarchy, a derivation path must be specified. Each element in the derivation path array is either a 32-bit integer encoded as 4 bytes in big-endian, or a byte array of arbitrary length.

## Signing

Computing threshold ECDSA signatures is the core functionality of this feature. **Canisters do not hold ECDSA keys themselves**, but keys are derived from a master key held by dedicated subnets. A canister can request the computation of a signature through the management canister API. The request is then routed to a subnet holding the specified key and the subnet computes the requested signature using threshold cryptography. Thereby, it derives the canister root key or a key obtained through further derivation, as part of the signature protocol, from a shared secret and the requesting canister's principal identifier. Thus, a canister can only request signatures to be created for its canister root key or a key derived from it. This means that canisters "control" their private ECDSA keys in that they decide when signatures are to be created with them, but don't hold a private key themselves.

## Signature verification

The created signatures can be verified with the public key corresponding to the same canister and derivation path. Example verification in JavaScript using the [secp256k1](https://www.npmjs.com/package/secp256k1) npm package:

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

Similar verifications can be done in many other languages with the help of cryptographic libraries that support the `secp256k1` curve.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
