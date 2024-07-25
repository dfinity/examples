---
keywords: [advanced, motoko, threshold schnorr, schnorr, signature]
---

# Threshold Schnorr

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/threshold-schnorr)

## Overview

We present a minimal example canister smart contract for showcasing the
[threshold
Schnorr](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr)
API.

The example canister is a signing oracle that creates Schnorr signatures with
keys derived based on the canister ID and the chosen algorithm, either BIP340 or
Ed25519.

More specifically:

- The sample canister receives a request that provides a message and an algorithm ID.
- The sample canister uses the key derivation string for the derivation path.
- The sample canister uses the above to request a signature from the threshold
  Schnorr [subnet](https://wiki.internetcomputer.org/wiki/Subnet_blockchain)
  (the threshold Schnorr subnet is a subnet generating threshold Schnorr
  signatures).

This tutorial gives a complete overview of the development, starting with downloading [`dfx`](https://internetcomputer.org/docs/current/developer-docs/setup/index.md), up to the deployment and trying out the code on the mainnet.

This walkthrough focuses on the version of the sample canister code written in
Motoko programming language. There is also a
[Rust](https://github.com/dfinity/examples/tree/master/rust/threshold-schnorr)
version available in the same repo and follows the same commands for deploying.


## Prerequisites
-   [x] Download and [install the IC
    SDK](https://internetcomputer.org/docs/current/developer-docs/setup/index.md)
    if you do not already have it. For local testing, `dfx >= 0.22.0-beta.0` is
    required.
-   [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Getting started

Sample code for `threshold-schnorr-example` is provided in the [examples repository](https://github.com/dfinity/examples), under either [`/motoko`](https://github.com/dfinity/examples/tree/master/motoko/threshold-schnorr) or [`/rust`](https://github.com/dfinity/examples/tree/master/rust/threshold-schnorr) sub-directories.

### Deploy and test the canister locally 

This tutorial will use the Motoko version of the canister.

To deploy:
```bash
cd examples/motoko/threshold-schnorr
dfx start --background
make deploy
```

To test (includes deploying):
```bash
cd examples/motoko/threshold-schnorr
dfx start --background
npm install @noble/curves
make test
```

#### What this does
- `dfx start --background` starts a local instance of the IC via the IC SDK
- `make deploy` deploys the canister code on the local version of the IC
- `npm install @noble/curves` installs a test javascript dependency
- `make test` deploys and tests the canister code on the local version of the IC

If deployment was successful, you should see something like this:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    schnorr_example_motoko: http://127.0.0.1:4943/?canisterId=t6rzw-2iaaa-aaaaa-aaama-cai&id=st75y-vaaaa-aaaaa-aaalq-cai
```

If you open the URL in a web browser, you will see a web UI that shows the
public methods the canister exposes. Since the canister exposes `public_key` and
`sign`, those are rendered in the web UI.

### Deploying the canister on the mainnet

To deploy this canister the mainnet, one needs to do two things:

- Acquire cycles (equivalent of "gas" in other blockchains). This is necessary for all canisters.
- Update the sample source code to have the right key ID. This is unique to this canister.

#### Acquire cycles to deploy

Deploying to the Internet Computer requires [cycles](https://internetcomputer.org/docs/current/developer-docs/setup/cycles). You can get free cycles from the [cycles faucet](https://internetcomputer.org/docs/current/developer-docs/getting-started/cycles/cycles-faucet).

#### Update source code with the right key ID

To deploy the sample code, the canister needs the right key ID for the right environment. Specifically, one needs to replace the value of the `key_id` in the `src/schnorr_example_motoko/src/lib.rs` file of the sample code. Before deploying to mainnet, one should modify the code to use the right name of the `key_id`.

There are three options that are planed to be supported:

* `dfx_test_key`: a default key ID that is used in deploying to a local version of IC (via IC SDK).
* `test_key_1`: a master **test** key ID that is used in mainnet.
* `key_1`: a master **production** key ID that is used in mainnet.

For example, the default code in `src/schnorr_example_motoko/src/lib.rs`
hard-codes the used of `dfx_test_key` and derives the key ID as follows and can
be deployed locally:
```motoko
key_id = { algorithm = algorithm_arg; name = "dfx_test_key" }
```

IMPORTANT: To deploy to IC mainnet, one needs to replace `"dfx_test_key"` with
 either "test_key_1"` or `"key_1"` depending on the desired intent. Both uses of
key ID in `src/schnorr_example_motoko/src/main.mo` must be consistent.

#### Deploy to the mainnet via IC SDK

To [deploy via the mainnet](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-mainnet.md), run the following commands:

```bash
dfx deploy --network ic
```
If successful, you should see something like this:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    schnorr_example_motoko: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=enb64-iaaaa-aaaap-ahnkq-cai
```

The implementation of this canister in Rust is (`schnorr_example_rust`) is
deployed on mainnet. It has the URL
https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=enb64-iaaaa-aaaap-ahnkq-cai
and serves up the Candid web UI for this particular canister deployed on
mainnet.

## Obtaining public keys

### Using the Candid Web UI

If you deployed your canister locally or to the mainnet, you should have a URL to the Candid web UI where you can access the public methods. We can call the `public-key` method.

In the example below, the method returns
`6e48e755842d0323be83edc7fc8766a20423c8127f7731993873d2f123d01a34` as the
Ed25519 public key.

```json
{
  "Ok":
  {
    "public_key_hex": "6e48e755842d0323be83edc7fc8766a20423c8127f7731993873d2f123d01a34"
  }
}
```


### Code walkthrough
Open the file `lib.rs`, which will show the following Motoko code that
demonstrates how to obtain a Schnorr public key. 

```motoko
  public shared ({ caller }) func public_key(algorithm_arg : SchnorrAlgotirhm) : async {
    #Ok : { public_key_hex : Text };
    #Err : Text;
  } {
    try {
      let { public_key } = await ic.schnorr_public_key({
        canister_id = null;
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm = algorithm_arg; name = "dfx_test_key" };
      });
      #Ok({ public_key_hex = Hex.encode(Blob.toArray(public_key)) });
    } catch (err) {
      #Err(Error.message(err));
    };
  };
```

In the code above, the canister calls the `schnorr_public_key` method of the [IC management canister](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-management-canister) (`aaaaa-aa`). 


**The [IC management
canister](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-management-canister)
is just a facade; it does not exist as a canister (with isolated state, Wasm
code, etc.). It is an ergonomic way for canisters to call the system API of the
IC (as if it were a single canister). In the code below, we use the management
canister to create a Schnorr public key. Canister ID `"aaaaa-aa"`
declares the IC management canister in the canister code.**

### Canister root public key

For obtaining the canister's root public key, the derivation path in the API can be simply left empty.

### Key derivation

-   For obtaining a canister's public key below its root key in the BIP-32 key derivation hierarchy, a derivation path needs to be specified. As explained in the general documentation, each element in the array of the derivation path is either a 32-bit integer encoded as 4 bytes in big endian or a byte array of arbitrary length. The element is used to derive the key in the corresponding level at the derivation hierarchy.
-   In the example code above, we use the bytes extracted from the `msg.caller` principal in the `derivation_path`, so that different callers of `public_key()` method of our canister will be able to get their own public keys.

## Signing

Computing threshold Schnorr signatures is the core functionality of this feature. **Canisters do not hold Schnorr keys themselves**, but keys are derived from a master key held by dedicated subnets. A canister can request the computation of a signature through the management canister API. The request is then routed to a subnet holding the specified key and the subnet computes the requested signature using threshold cryptography. Thereby, it derives the canister root key or a key obtained through further derivation, as part of the signature protocol, from a shared secret and the requesting canister's principal identifier. Thus, a canister can only request signatures to be created for its canister root key or a key derived from it. This means, that canisters "control" their private Schnorr keys in that they decide when signatures are to be created with them, but don't hold a private key themselves.

```motoko
  public shared ({ caller }) func sign(message_arg : Text, algorithm_arg : SchnorrAlgotirhm) : async {
    #Ok : { signature_hex : Text };
    #Err : Text;
  } {
    try {
      Cycles.add(25_000_000_000);
      let { signature } = await ic.sign_with_schnorr({
        message = Text.encodeUtf8(message_arg);
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm = algorithm_arg; name = "dfx_test_key" };
      });
      #Ok({ signature_hex = Hex.encode(Blob.toArray(signature)) });
    } catch (err) {
      #Err(Error.message(err));
    };
  };
```

## Signature verification

For completeness of the example, we show that the created signatures can be
verified with the public key corresponding to the same canister and derivation
path in javascript. Note that in contrast to the Rust implementation of this
example, the signature verification is not part of the canister API and happens
externally.

Ed25519 can be verified as follows:
```javascript
import('@noble/curves/ed25519').then((ed25519) => { verify(ed25519.ed25519); })
  .catch((err) => { console.log(err) });

function verify(ed25519) {
  const test_sig = '1efa03b7b7f9077449a0f4b3114513f9c90ccf214166a8907c23d9c2bbbd0e0e6e630f67a93c1bd525b626120e86846909aedf4c58763ae8794bcef57401a301'
  const test_pubkey = '566d53caf990f5f096d151df70b2a75107fac6724cb61a9d6d2aa63e1496b003'
  const test_msg = Uint8Array.from(Buffer.from("hello", 'utf8'));

  console.log(ed25519.verify(test_sig, test_msg, test_pubkey));
  }
```

BIP340 can be verified as follows:
```javascript
import('@noble/curves/secp256k1').then((bip340) => { verify(bip340.schnorr); })
  .catch((err) => { console.log(err) });

function verify(bip340) {
  const test_sig = '1b64ca7a7f02c76633954f320675267685b3b80560eb6a35cda20291ddefc709364e59585771c284e46264bfbb0620e23eb8fb274994f7a6f2fcbc8a9430e5d7';
  // the first byte of the BIP340 public key is truncated
  const pubkey = '0341d7cf39688e10b5f11f168ad0a9e790bcb429d7d486eab07d2c824b85821470'.substring(2)
  const test_msg = Uint8Array.from(Buffer.from("hello", 'utf8'));

  console.log(bip340.verify(test_sig, test_msg, test_pubkey));
}
```

The call to `verify` function should always return `true` for correct parameters
and `false` or error otherwise.

Similar verifications can be done in many other languages with the help of
cryptographic libraries that support the `bip340secp256k1` signing *with
arbitrary message length* as specified in
[BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki#user-content-Messages_of_Arbitrary_Size)
and `ed25519` signing.

## Conclusion

In this walkthrough, we deployed a sample smart contract that:

* Signed with private Schnorr keys even though **canisters do not hold Schnorr keys themselves**.
* Requested a public key.
* Performed signature verification.
