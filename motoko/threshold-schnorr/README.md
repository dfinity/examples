# Threshold Schnorr

This example demonstrates how to use the ICP [threshold Schnorr](https://internetcomputer.org/docs/building-apps/network-features/signatures/t-schnorr) API from a Motoko canister. The canister acts as a signing oracle that creates Schnorr signatures using keys derived from the canister ID and the chosen algorithm (BIP340/BIP341 or Ed25519). Canisters do not hold private keys themselves — signing requests are routed to threshold Schnorr subnets that compute signatures using distributed key shares.

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/threshold-schnorr
```

### Deploy and test
```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Key ID configuration

The canister uses `test_key_1` by default (mainnet test key). To use a different key, update the `key_id` value in `backend/app.mo`:

- `test_key_1`: mainnet test key
- `key_1`: mainnet production key

## Obtaining public keys

Call the `public_key` method with the desired algorithm variant:

```bash
icp canister call backend public_key '(variant { ed25519 })'
icp canister call backend public_key '(variant { bip340secp256k1 })'
```

The derivation path uses the caller's principal, so different callers receive different public keys.

## Signing

Call the `sign` method with a message, algorithm, and optional BIP341 tweak. For `bip340secp256k1`, the message must be exactly 32 bytes:

```bash
icp canister call backend sign '("hello world", variant { ed25519 }, null)'
icp canister call backend sign '("hello world of BIP340-secp256k1!", variant { bip340secp256k1 }, null)'
```

For BIP341 (Taproot) signing with a tweak:
```bash
icp canister call backend sign '("hello world of BIP340-secp256k1!", variant { bip340secp256k1 }, opt "012345678901234567890123456789012345678901234567890123456789abcd")'
```

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
