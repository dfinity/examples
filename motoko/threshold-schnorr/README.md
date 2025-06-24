# Threshold Schnorr

We present a minimal example canister smart contract for showcasing the [threshold Schnorr](https://internetcomputer.org/docs/building-apps/network-features/signatures/t-schnorr) API.

The example canister is a signing oracle that creates Schnorr signatures with
keys derived based on the canister ID and the chosen algorithm, either BIP340/BIP341 or
Ed25519.

More specifically:

- The sample canister receives a request that provides a message and an algorithm ID.
- The sample canister uses the key derivation string for the derivation path.
- The sample canister uses the above to request a signature from the threshold
  Schnorr [subnet](https://wiki.internetcomputer.org/wiki/Subnet_blockchain)
  (the threshold Schnorr subnet is a subnet generating threshold Schnorr
  signatures).

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/motoko/threshold-schnorr)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

### 5. Update source code with the right key ID

To deploy the sample code, the canister needs the right key ID for the right environment. Specifically, one needs to replace the value of the `key_id` in the `src/schnorr_example_motoko/src/lib.rs` file of the sample code. Before deploying to mainnet, one should modify the code to use the right name of the `key_id`.

There are four options that are supported:

* `insecure_test_key_1`: the key ID supported by the `chainkey_testing_canister`
  ([link](https://github.com/dfinity/chainkey-testing-canister/)).
* `dfx_test_key`: a default key ID that is used in deploying to a local version of IC (via IC SDK).
* `test_key_1`: a master **test** key ID that is used in mainnet.
* `key_1`: a master **production** key ID that is used in mainnet.

For example, the default code in `src/schnorr_example_motoko/src/main.mo`
hard-codes the use of `insecure_test_key_1` and derives the key ID as follows and can
be deployed locally:

```motoko
key_id = { algorithm = algorithm_arg; name = "insecure_test_key_1" }
```

> [!WARNING]
> IMPORTANT: To deploy to IC mainnet, one needs to replace `"insecure_test_key_1"` with either `"test_key_1"` or `"key_1"` depending on the desired intent. Both uses of key ID in `src/schnorr_example_motoko/src/main.mo` must be consistent.

### Deploying

To [deploy via mainnet](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-mainnet.md), run the following commands:

```bash
npm install
dfx deploy --network ic
```

## Obtaining public keys

If you deployed your canister locally or to the mainnet, you should have a URL to the Candid web UI where you can access the public methods. You can call the `public_key` method.

### Canister root public key

For obtaining the canister's root public key, the derivation path in the API can be simply left empty.

### Key derivation

-   For obtaining a canister's public key below its root key in the BIP-32 key derivation hierarchy, a derivation path needs to be specified. As explained in the general documentation, each element in the array of the derivation path is either a 32-bit integer encoded as 4 bytes in big endian or a byte array of arbitrary length. The element is used to derive the key in the corresponding level at the derivation hierarchy.
-   In the example code above, we use the bytes extracted from the `msg.caller` principal in the `derivation_path`, so that different callers of `public_key()` method of our canister will be able to get their own public keys.

## Signing

Computing threshold Schnorr signatures is the core functionality of this feature. **Canisters do not hold Schnorr keys themselves**, but keys are derived from a master key held by dedicated subnets. A canister can request the computation of a signature through the management canister API. The request is then routed to a subnet holding the specified key and the subnet computes the requested signature using threshold cryptography. Thereby, it derives the canister root key or a key obtained through further derivation, as part of the signature protocol, from a shared secret and the requesting canister's principal identifier. Thus, a canister can only request signatures to be created for its canister root key or a key derived from it. This means, that canisters "control" their private Schnorr keys in that they decide when signatures are to be created with them, but don't hold a private key themselves.

## Signature verification

For completeness of the example, we show that the created signatures can be
verified with the public key corresponding to the same canister and derivation
path in javascript. Note that in contrast to the Rust implementation of this
example, the signature verification is not part of the canister API and happens
externally.

Ed25519 can be verified as follows:
```javascript
    async function run() {
        try {
            const ed25519 = await import('@noble/ed25519');
            const sha512 = await import('@noble/hashes/sha512');

            ed25519.etc.sha512Sync = (...m) => sha512.sha512(ed25519.etc.concatBytes(...m));

            const test_sig = '1efa03b7b7f9077449a0f4b3114513f9c90ccf214166a8907c23d9c2bbbd0e0e6e630f67a93c1bd525b626120e86846909aedf4c58763ae8794bcef57401a301';
            const test_pubkey = '566d53caf990f5f096d151df70b2a75107fac6724cb61a9d6d2aa63e1496b003'
            const test_msg = Uint8Array.from(Buffer.from("hello", 'utf8'));

            console.log(ed25519.verify(test_sig, test_msg, test_pubkey));
        }
        catch(err) {
            console.log(err);
        }
    }

    run();
```

BIP340 can be verified as follows:
```javascript
    async function run() {
        try {
            const ecc = await import('tiny-secp256k1');

            const test_sig = Buffer.from('311e1dceddd1380d0424e01b19711e926ca2f26c0dda57b405bec1359510674871a22487c96afa4a4bf47858d1d79caa400bb51ab793d9fad2a689f8bfc681aa', 'hex');
            const test_pubkey = Buffer.from('02472bb4da5c5ce627d599feba90d0257a558d4e226f9fc7914f811e301ad06f38'.substring(2), 'hex');
            const test_msg = Uint8Array.from(Buffer.from("hellohellohellohellohellohello12", 'utf8'));

            console.log(ecc.verifySchnorr(test_msg, test_pubkey, test_sig));
        }
        catch(err) {
            console.log(err);
        }
    }

    run();
```

BIP341 can be verified as follows:
```javascript
    async function run() {
        try {
            const bip341 = await import('bitcoinjs-lib/src/payments/bip341.js');
            const bitcoin = await import('bitcoinjs-lib');
            const ecc = await import('tiny-secp256k1');

            bitcoin.initEccLib(ecc);

            const test_tweak = Buffer.from('012345678901234567890123456789012345678901234567890123456789abcd', 'hex');
            const test_sig = Buffer.from('3c3e51fc771a5a8cb553bf2dd151bb02d0f473ff274a92d32310267977918d72121f97c318226422c033d33daf376d42c9a07e71643ff332cb30611fe5e163da', 'hex');
            const test_pubkey = Buffer.from('02472bb4da5c5ce627d599feba90d0257a558d4e226f9fc7914f811e301ad06f38'.substring(2), 'hex');
            const test_msg = Uint8Array.from(Buffer.from("hellohellohellohellohellohello12", 'utf8'));

            const tweaked_test_pubkey = bip341.tweakKey(test_pubkey, test_tweak).x;

            console.log(ecc.verifySchnorr(test_msg, tweaked_test_pubkey, test_sig));
        }
        catch(err) {
            console.log(err);
        }
    }

    run();
```

The call to `verify/verifySchnorr` function should always return `true` for correct parameters
and `false` or error otherwise.

Similar verifications can be done in many other languages with the help of
cryptographic libraries that support the BIP340/BIP341 and `ed25519` signing.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
