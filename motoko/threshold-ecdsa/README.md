# Threshold ECDSA sample

We present a minimal example canister smart contract for showcasing the [threshold ECDSA](https://internetcomputer.org/docs/building-apps/network-features/signatures/t-ecdsa) API.

The example canister is a signing oracle that creates ECDSA signatures with keys derived from an input string.

More specifically:

- The sample canister receives a request that provides a message.
- The sample canister hashes the message and uses the key derivation string for the derivation path.
- The sample canister uses the above to request a signature from the threshold ECDSA [subnet](https://wiki.internetcomputer.org/wiki/Subnet_blockchain) (the threshold ECDSA is a subnet specializing in generating threshold ECDSA signatures).

This tutorial gives a complete overview of the development, starting with downloading the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install), up to the deployment and trying out the code on the IC mainnet.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/motoko/threshold-ecdsa)

### 1. Update source code with the right key ID

To deploy the sample code from ICP Ninja, the canister needs the right key ID for the right environment. Specifically, one needs to replace the value of the `key_id` in the `src/ecdsa_example_motoko/main.mo` file of the sample code. Before deploying to the mainnet from ICP Ninja, one should modify the code to use the right name of the `key_id`.

There are three options:

* `dfx_test_key`: a default key ID that is used in deploying to a local version of IC (via IC SDK).
* `test_key_1`: a master **test** key ID that is used in mainnet.
* `key_1`: a master **production** key ID that is used in mainnet.

For example, the default code in `src/ecdsa_example_motoko/main.mo` includes the following lines and can be deployed locally:

:::caution
The following example is two **code snippets** that are part of a larger code file. These snippets may return an error if run on their own.
:::

```motoko
let { public_key } = await ic.ecdsa_public_key({
  canister_id = null;
  derivation_path = [ caller ];
  key_id = { curve = #secp256k1; name = "dfx_test_key" };
});
```

```motoko
let { signature } = await ic.sign_with_ecdsa({
  message_hash;
  derivation_path = [ caller ];
  key_id = { curve = #secp256k1; name = "dfx_test_key" };
});
```

> [!WARNING]
> To deploy to IC mainnet, one needs to replace the value in `key_id` fields with the values `"dfx_test_key"` to instead have either `"test_key_1"` or `"key_1"` depending on the desired intent.


## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Obtaining public keys

If you deployed your canister locally or to the mainnet, you should have a URL to the Candid web UI where you can access the public methods. We can call the `public-key` method.

### Canister root public key

For obtaining the canister's root public key, the derivation path in the API can be simply left empty.

### Key derivation

-   For obtaining a canister's public key below its root key in the BIP-32 key derivation hierarchy, a derivation path needs to be specified. As explained in the general documentation, each element in the array of the derivation path is either a 32-bit integer encoded as 4 bytes in big endian or a byte array of arbitrary length. The element is used to derive the key in the corresponding level at the derivation hierarchy.
-   In the example code above, we use the bytes extracted from the `msg.caller` principal in the `derivation_path`, so that different callers of `public_key()` method of our canister will be able to get their own public keys.

## Signing

Computing threshold ECDSA signatures is the core functionality of this feature. **Canisters do not hold ECDSA keys themselves**, but keys are derived from a master key held by dedicated subnets. A canister can request the computation of a signature through the management canister API. The request is then routed to a subnet holding the specified key and the subnet computes the requested signature using threshold cryptography. Thereby, it derives the canister root key or a key obtained through further derivation, as part of the signature protocol, from a shared secret and the requesting canister's principal identifier. Thus, a canister can only request signatures to be created for its canister root key or a key derived from it. This means that canisters "control" their private ECDSA keys in that they decide when signatures are to be created with them, but don't hold a private key themselves.

## Signature verification

For completeness of the example, we show that the created signatures can be verified with the public key corresponding to the same canister and derivation path.

The following shows how this verification can be done in Javascript, with the [secp256k1](https://www.npmjs.com/package/secp256k1) npm package:

```javascript
let { ecdsaVerify } = require("secp256k1")

let public_key = ... // Uint8Array type, the result of calling the above canister "public_key" function.
let hash = ...       // 32-byte Uint8Array representing a binary hash (e.g. sha256).
let signature = ...  // Uint8Array type, the result of calling the above canister "sign" function on `hash`.

let verified = ecdsaVerify(signature, hash, public_key)
```

The call to `ecdsaVerify` function should always return `true`.

Similar verifications can be done in many other languages with the help of cryptographic libraries that support the `secp256k1` curve.

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
