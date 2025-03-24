# Threshold Schnorr

We present a minimal example canister smart contract for showcasing the [threshold Schnorr](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr) API.

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

This walkthrough focuses on the version of the sample canister code written in
Motoko programming language. There is also a
[Rust](https://github.com/dfinity/examples/tree/master/rust/threshold-schnorr)
version available in the same repo and follows the same commands for deploying.

## Local development

### Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`
- [x] For running tests also install [`npm`](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm).

Begin by opening a terminal window.

### Step 1: Setup the project environment

Navigate into the folder containing the project's files, start a local instance of the Internet Computer and with the commands:

```bash
cd examples/motoko/threshold-schnorr
dfx start --background
```

#### What this does
- `dfx start --background` starts a local instance of the IC via the IC SDK.

### Step 2: Deploy the canisters

```bash
make deploy
```

#### What this does
- `make deploy` deploys the canister code on the local version of the IC.

If deployment was successful, you should see something like this:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    schnorr_example_motoko: http://127.0.0.1:4943/?canisterId=t6rzw-2iaaa-aaaaa-aaama-cai&id=st75y-vaaaa-aaaaa-aaalq-cai
```

If you open the URL in a web browser, you will see a web UI that shows the
public methods the canister exposes. Since the canister exposes `public_key`, 
`sign`, and `verify`, those are rendered in the web UI.

### Step 3 (optional): Run tests

```bash
npm install
make test
```

#### What this does

- `npm install` installs test javascript dependencies
- `make test` deploys and tests the canister code on the local version of the
  IC


## Deploying the canister on the mainnet

To deploy this canister on the mainnet, one needs to do two things:

- Acquire cycles (equivalent of "gas" in other blockchains). This is necessary for all canisters.
- Update the sample source code to have the right key ID. This is unique to this canister.

### Acquire cycles to deploy

Deploying to the Internet Computer requires
[cycles](https://internetcomputer.org/docs/current/developer-docs/getting-started/tokens-and-cycles)
(the equivalent of "gas" on other blockchains).

### Update source code with the right key ID

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

IMPORTANT: To deploy to IC mainnet, one needs to replace `"insecure_test_key_1"` with
either `"test_key_1"` or `"key_1"` depending on the desired intent. Both uses of
key ID in `src/schnorr_example_motoko/src/main.mo` must be consistent.

### Deploying

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

### Using the Candid UI

If you deployed your canister locally or to the mainnet, you should have a URL to the Candid web UI where you can access the public methods. We can call the `public_key` method.

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
Open the file `main.mo`, which will show the following Motoko code that
demonstrates how to obtain a Schnorr public key.

```motoko
  public shared ({ caller }) func public_key(algorithm : SchnorrAlgorithm) : async {
    #ok : { public_key_hex : Text };
    #err : Text;
  } {
    try {
      let { public_key } = await ic.schnorr_public_key({
        canister_id = null;
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm; name = "insecure_test_key_1" };
      });
      #Ok({ public_key_hex = Hex.encode(Blob.toArray(public_key)) });
    } catch (err) {
      #err(Error.message(err));
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
  public shared ({ caller }) func sign(message_arg : Text, algorithm : SchnorrAlgorithm, bip341TweakHex : ?Text) : async {
    #ok : { signature_hex : Text };
    #err : Text;
  } {
    let aux = switch (Option.map(bip341TweakHex, tryHexToTweak)) {
      case (null) null;
      case (?#ok some) ?some;
      case (?#err err) return #err err;
    };

    try {
      Cycles.add<system>(25_000_000_000);
      let signArgs = {
        message = Text.encodeUtf8(message_arg);
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm; name = "insecure_test_key_1" };
        aux;
      };
      let { signature } = await ic.sign_with_schnorr(signArgs);
      #ok({ signature_hex = Hex.encode(Blob.toArray(signature)) });
    } catch (err) {
      #err(Error.message(err));
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

## Conclusion

In this walkthrough, we deployed a sample smart contract that:

* Signed with private Schnorr keys even though **canisters do not hold Schnorr keys themselves**.
* Requested a public key.
* Performed signature verification.
