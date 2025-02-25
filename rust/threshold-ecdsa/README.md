# Threshold ECDSA sample

We present a minimal example canister smart contract for showcasing the [threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/integrations/t-ecdsa) API.

The example canister is a signing oracle that creates ECDSA signatures with keys derived from an input string.

More specifically:

- The sample canister receives a request that provides a message.
- The sample canister hashes the message and uses the key derivation string for the derivation path.
- The sample canister uses the above to request a signature from the threshold ECDSA [subnet](https://wiki.internetcomputer.org/wiki/Subnet_blockchain) (the threshold ECDSA is a subnet specializing in generating threshold ECDSA signatures).

This tutorial gives a complete overview of the development, starting with downloading the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/index.md), up to the deployment and trying out the code on the IC mainnet.

> [!TIP]
> This walkthrough focuses on the version of the sample canister code written in the Rust programming language. There is also a [Motoko](https://internetcomputer.org/docs/current/developer-docs/backend/motoko/index.md) version available in the same repo and follows the same commands for deploying.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install) v0.11.0 or newer.
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Local deployment

Begin by opening a terminal window.

### Step 1: Setup the project environment

Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the commands:

```bash
cd examples/rust/threshold-ecdsa
dfx start --background
```

### Step 2: Deploy the canisters

```bash
dfx deploy
```

If successful, you should see something like this:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    ecdsa_example_rust: http://127.0.0.1:4943/?canisterId=t6rzw-2iaaa-aaaaa-aaama-cai&id=st75y-vaaaa-aaaaa-aaalq-cai
```

If you open the URL in a web browser, you will see a web UI that shows the public methods the canister exposes. Since the canister exposes `public_key` and `sign` methods, those are rendered in the web UI.

## Deploying the canister on the mainnet

To deploy this canister to the mainnet, one needs to do two things:

- Acquire cycles (the equivalent of "gas" on other blockchains). This is necessary for all canisters.
- Update the sample source code to have the right key ID. This is unique to this canister.

### Acquire cycles to deploy

Deploying to the Internet Computer requires [cycles](https://internetcomputer.org/docs/current/developer-docs/getting-started/tokens-and-cycles) (the equivalent of "gas" on other blockchains).

### Update source code with the right key ID

To deploy the sample code, the canister needs the right key ID for the right environment. Specifically, one needs to replace the value of the `key_id` in the `src/ecdsa_example_rust/src/lib.rs` file of the sample code. Before deploying to the mainnet, one should modify the code to use the right name of the `key_id`.

There are three options:

* `dfx_test_key`: a default key ID that is used in deploying to a local version of IC (via IC SDK).
* `test_key_1`: a master **test** key ID that is used in mainnet.
* `key_1`: a master **production** key ID that is used in mainnet.

> [!WARNING]
> To deploy to IC mainnet, one needs to replace the value in `key_id `fields with the values `EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id()` (mapping to `dfx_test_key`) to instead have either `EcdsaKeyIds::TestKey1.to_key_id()` (mapping to `test_key_1`) or `EcdsaKeyIds::ProductionKey1.to_key_id()` (mapping to `key_1`) depending on the desired intent.

### Deploying

To [deploy via mainnet](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-mainnet.md), run the following commands:

```bash
dfx deploy --network ic
```
If successful, you should see something like this:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    ecdsa_example_rust: https://a3gq9-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=736w4-cyaaa-aaaal-qb3wq-cai
```

In the example above, `ecdsa_example_rust` has the URL https://a3gq9-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=736w4-cyaaa-aaaal-qb3wq-cai and serves up the Candid web UI for this particular canister deployed on mainnet.

## Obtaining public keys

### Using the Candid UI

If you deployed your canister locally or to the mainnet, you should have a URL to the Candid web UI where you can access the public methods. We can call the `public-key` method.

In the example below, the method returns `03c22bef676644dba524d4a24132ea8463221a55540a27fc86d690fda8e688e31a` as the public key.

```json
{
  "Ok":
  {
    "public_key_hex": "03c22bef676644dba524d4a24132ea8463221a55540a27fc86d690fda8e688e31a"
  }
}
```

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

## Conclusion

In this walkthrough, we deployed a sample smart contract that:

* Signed with private ECDSA keys even though **canisters do not hold ECDSA keys themselves**.
* Requested a public key.
* Performed signature verification.
