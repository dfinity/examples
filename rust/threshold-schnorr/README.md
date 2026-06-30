# Threshold Schnorr

A minimal example canister demonstrating the [threshold Schnorr](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr) API on ICP.

The example canister acts as a signing oracle that creates Schnorr signatures with keys derived based on the canister ID and the chosen algorithm, either BIP340/BIP341 (secp256k1) or Ed25519.

More specifically:

- The canister receives a request providing a message and an algorithm ID.
- The canister uses the caller's principal bytes as the key derivation path.
- The canister requests a signature from the threshold Schnorr subnet, which computes it using threshold cryptography.

This walkthrough focuses on the [Rust](https://github.com/dfinity/examples/tree/master/rust/threshold-schnorr) implementation.
There is also a [Motoko version](https://github.com/dfinity/examples/tree/master/motoko/threshold-schnorr) in the same repo.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [ic-mops](https://mops.one/docs/install): `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/threshold-schnorr
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

### PocketIC integration tests

The canister includes integration tests that run against a local [PocketIC](https://github.com/dfinity/pocketic) instance with a fiduciary subnet, covering all algorithm and merkle root combinations including negative cases (corrupted signature, message, and key):

```bash
# Build the WASM via icp build (handles platform-specific C toolchain requirements
# for secp256k1), then run the integration tests
icp build backend
cargo test --package backend --test integration_tests
```

The PocketIC server binary is downloaded automatically on first run (or set `POCKET_IC_BIN` to an existing binary path).

## Key IDs

The key name defaults to `test_key_1` and can be overridden at deploy time via an init argument:

```bash
# Use the production key on mainnet
icp deploy --network ic --init-arg '(opt "key_1")'
```

Available key names:

- `insecure_test_key_1`: supported by the [chainkey testing canister](https://github.com/dfinity/chainkey-testing-canister/)
- `test_key_1`: default — master **test** key, works on both the local network and mainnet
- `key_1`: master **production** key on mainnet

PocketIC integration tests pass `opt "key_1"` as the init argument when installing the canister, since the PocketIC fiduciary subnet only provides `key_1`.

## How it works

### Obtaining public keys

The canister calls the `schnorr_public_key` method of the [IC management canister](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-management-canister) (`aaaaa-aa`). The management canister is a facade — it does not exist as a canister with isolated state; it is an ergonomic way for canisters to call the IC system API.

For the canister's root public key, the derivation path can be left empty. To obtain a key below the root in the BIP-32 hierarchy, specify a derivation path where each element is either a 32-bit integer (4 bytes, big endian) or a byte array of arbitrary length. This example uses the caller's principal bytes so that different callers get distinct keys.

### Signing

**Canisters do not hold Schnorr keys themselves.** Keys are derived from a master key held by dedicated subnets. When a canister requests a signature, the request is routed to the subnet holding the specified key, which computes the signature using threshold cryptography. The canister root key (or a derived key) is computed from a shared secret and the requesting canister's principal — meaning canisters control *when* their keys are used but never hold the private key material.

The threshold Schnorr API accepts optional auxiliary information for signing (not available for public key requests, since the public key can be used directly). Currently, the only supported auxiliary type is a [BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki) Merkle tree root hash for Bitcoin taproot addresses. The key is "tweaked" by adding a hash over the untweaked public key and the user-provided Merkle root. See the `basic_bitcoin` example for how this is used in practice.

### Signature verification

The example includes on-chain verification to demonstrate that signatures created for a given canister and derivation path can be verified against the corresponding public key. For BIP340, the first byte of the compressed public key (the 02/03 prefix) is dropped before verification.

The `test.sh` script performs additional off-chain verification using Node.js libraries (`@noble/ed25519`, `tiny-secp256k1`, `bitcoinjs-lib`) to confirm all three signature types (ed25519, bip340secp256k1, bip341).

## Security considerations and best practices

If you base your application on this example, familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all best practices.
