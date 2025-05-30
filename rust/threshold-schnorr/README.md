# Threshold Schnorr

We present a minimal example canister smart contract for showcasing the [threshold Schnorr](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-sign_with_schnorr) API.

The example canister is a signing oracle that creates Schnorr signatures with
keys derived based on the canister ID and the chosen algorithm, either
BIP340/BIP341 or Ed25519.

More specifically:

- The sample canister receives a request that provides a message and an algorithm ID.
- The sample canister uses the key derivation string for the derivation path.
- The sample canister uses the above to request a signature from the threshold
  Schnorr [subnet](https://wiki.internetcomputer.org/wiki/Subnet_blockchain)
  (the threshold Schnorr subnet is a subnet generating threshold Schnorr
  signatures).

This walkthrough focuses on the version of the sample canister code written in
[Rust](https://github.com/dfinity/examples/tree/master/rust/threshold-schnorr).
There is also a Motoko version available in the same repo and follows the same commands for deploying.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Local deployment 

Begin by opening a terminal window.

### Step 1: Setup the project environment

Navigate into the folder containing the project's files, start a local instance of the Internet Computer and with the commands:

```bash
cd examples/rust/threshold-schnorr
dfx start --background
```

#### What this does
- `dfx start --background` starts a local instance of the IC via the IC SDK

### Step 2: Deploy the canisters

```bash
make deploy
```

To test (includes deploying):
```bash
make test
```

#### What this does
- `make deploy` deploys the canister code on the local version of the IC.

If deployment was successful, you should see something like this:

```bash
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    schnorr_example_rust: http://127.0.0.1:4943/?canisterId=t6rzw-2iaaa-aaaaa-aaama-cai&id=st75y-vaaaa-aaaaa-aaalq-cai
```

If you open the URL in a web browser, you will see a web UI that shows the
public methods the canister exposes. Since the canister exposes `public_key`,
`sign`, and `verify` methods, those are rendered in the web UI.

## Deploying the canister on the mainnet

To deploy this canister the mainnet, one needs to do two things:

- Acquire cycles (equivalent of "gas" in other blockchains). This is necessary for all canisters.
- Update the sample source code to have the right key ID. This is unique to this canister.

### Acquire cycles to deploy

Deploying to the Internet Computer requires [cycles](https://internetcomputer.org/docs/current/developer-docs/getting-started/tokens-and-cycles) (the equivalent of "gas" on other blockchains).

### Update source code with the right key ID

To deploy the sample code, the canister needs the right key ID for the right environment. Specifically, one needs to replace the value of the `key_id` in the `src/schnorr_example_rust/src/lib.rs` file of the sample code. Before deploying to mainnet, one should modify the code to use the right name of the `key_id`.

There are four options that are supported:

* `insecure_test_key_1`: the key ID supported by the `chainkey_testing_canister`
  ([link](https://github.com/dfinity/chainkey-testing-canister/)).
* `dfx_test_key`: a default key ID that is used in deploying to a local version of IC (via IC SDK).
* `test_key_1`: a master **test** key ID that is used in mainnet.
* `key_1`: a master **production** key ID that is used in mainnet.

For example, the default code in `src/schnorr_example_rust/src/lib.rs` derives
the key ID as follows and can be deployed locally:
`SchnorrKeyIds::TestKeyLocalDevelopment.to_key_id(algorithm)`

IMPORTANT: To deploy to IC mainnet, one needs to replace `"dfx_test_key"` with
either `"test_key_1"` or `"key_1"` depending on the desired intent. Both uses of
key ID in `src/schnorr_example_rust/src/lib.rs` must be consistent.

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
    schnorr_example_rust: https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=enb64-iaaaa-aaaap-ahnkq-cai
```

The implementation of this canister in Rust (`schnorr_example_rust`) is
deployed on mainnet. It has the URL
https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=enb64-iaaaa-aaaap-ahnkq-cai
and serves up the Candid web UI for this particular canister deployed on
mainnet.

## Obtaining public keys

### Using the Candid UI

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

Open the file `wasm_only.rs`, which will show the following Rust code that
demonstrates how to obtain a Schnorr public key. 

```rust
#[update]
async fn public_key(algorithm: SchnorrAlgorithm) -> Result<PublicKeyReply, String> {
    let request = ManagementCanisterSchnorrPublicKeyRequest {
        canister_id: None,
        derivation_path: vec![ic_cdk::api::caller().as_slice().to_vec()],
        key_id: SchnorrKeyIds::ChainkeyTestingCanisterKey1.to_key_id(algorithm),
    };

    let (res,): (ManagementCanisterSchnorrPublicKeyReply,) =
        ic_cdk::call(mgmt_canister_id(), "schnorr_public_key", (request,))
            .await
            .map_err(|e| format!("schnorr_public_key failed {}", e.1))?;

    Ok(PublicKeyReply {
        public_key_hex: hex::encode(&res.public_key),
    })
}
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

The threshold Schnorr signature API allows to pass auxiliary information for
signing. This is different in the API for obtaining the public key, where the
auxiliary information can be used directly on the public key because the public
key is known by the user. In signing, no one knows the private key in the clear,
and, therefore, the auxiliary information needs to be used on the key shares.

Currently, the only type of auxiliary information supported on ICP is a
[BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki) Merkle
tree root hash, which is part of Bitcoin taproot addresses. For BIP341, the key
is "tweaked" by adding to it a hash over the untweaked public key and the
user-provided Merkle tree root. Also see the `basic_bitcoin` example to find out
more about how this is used in practice.

```rust
#[update]
async fn sign(
    message: String,
    algorithm: SchnorrAlgorithm,
    opt_merkle_tree_root_hex: Option<String>,
) -> Result<SignatureReply, String> {
    let aux = opt_merkle_tree_root_hex
        .map(|hex| {
            hex::decode(&hex)
                .map_err(|e| format!("failed to decode hex: {e:?}"))
                .and_then(|bytes| {
                    if bytes.len() == 32 || bytes.is_empty() {
                        Ok(SignWithSchnorrAux::Bip341(SignWithBip341Aux {
                            merkle_root_hash: ByteBuf::from(bytes),
                        }))
                    } else {
                        Err(format!(
                            "merkle tree root bytes must be 0 or 32 bytes long but got {}",
                            bytes.len()
                        ))
                    }
                })
        })
        .transpose()?;

    let internal_request = ManagementCanisterSignatureRequest {
        message: message.as_bytes().to_vec(),
        derivation_path: vec![ic_cdk::api::caller().as_slice().to_vec()],
        key_id: SchnorrKeyIds::ChainkeyTestingCanisterKey1.to_key_id(algorithm),
        aux,
    };

    let (internal_reply,): (ManagementCanisterSignatureReply,) =
        ic_cdk::api::call::call_with_payment(
            mgmt_canister_id(),
            "sign_with_schnorr",
            (internal_request,),
            26_153_846_153,
        )
        .await
        .map_err(|e| format!("sign_with_schnorr failed {e:?}"))?;

    Ok(SignatureReply {
        signature_hex: hex::encode(&internal_reply.signature),
    })
}
```


## Signature verification

For completeness of the example, we show that the created signatures can be
verified with the public key corresponding to the same canister and derivation
path. Note that the first byte of the BIP340 public key needs to be removed for
verification, which is done by the verification function below internally.

```rust
#[query]
async fn verify(
    signature_hex: String,
    message: String,
    public_key_hex: String,
    opt_merkle_tree_root_hex: Option<String>,
    algorithm: SchnorrAlgorithm,
) -> Result<SignatureVerificationReply, String> {
    let sig_bytes = hex::decode(&signature_hex).expect("failed to hex-decode signature");
    let msg_bytes = message.as_bytes();
    let pk_bytes = hex::decode(&public_key_hex).expect("failed to hex-decode public key");

     match algorithm {
        SchnorrAlgorithm::Bip340Secp256k1 => match opt_merkle_tree_root_hex {
            Some(merkle_tree_root_hex) => {
                let merkle_tree_root_bytes = hex::decode(&merkle_tree_root_hex)
                    .expect("failed to hex-decode merkle tree root");
                verify_bip341_secp256k1(&sig_bytes, msg_bytes, &pk_bytes, &merkle_tree_root_bytes)
            }
            None => verify_bip340_secp256k1(&sig_bytes, msg_bytes, &pk_bytes),
        },
        SchnorrAlgorithm::Ed25519 => {
            if let Some(_) = opt_merkle_tree_root_hex {
                return Err("ed25519 does not support merkle tree root verification".to_string());
            }
            verify_ed25519(&sig_bytes, &msg_bytes, &pk_bytes)
        }
    }
}

fn verify_bip340_secp256k1(
    sig_bytes: &[u8],
    msg_bytes: &[u8],
    secp1_pk_bytes: &[u8],
) -> Result<SignatureVerificationReply, String> {
    assert_eq!(secp1_pk_bytes.len(), 33);
    assert_eq!(sig_bytes.len(), 64);

    let sig =
        k256::schnorr::Signature::try_from(sig_bytes).expect("failed to deserialize signature");

    let vk = k256::schnorr::VerifyingKey::from_bytes(&secp1_pk_bytes[1..])
        .expect("failed to deserialize BIP340 encoding into public key");

    let is_signature_valid = vk.verify_raw(&msg_bytes, &sig).is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}

fn verify_bip341_secp256k1(
    sig_bytes: &[u8],
    msg_bytes: &[u8],
    secp1_pk_bytes: &[u8],
    merkle_tree_root_bytes: &[u8],
) -> Result<SignatureVerificationReply, String> {
    assert_eq!(secp1_pk_bytes.len(), 33);

    let pk = XOnlyPublicKey::from_slice(&secp1_pk_bytes[1..]).unwrap();
    let tweaked_pk_bytes = {
        let secp256k1_engine = Secp256k1::new();
        let merkle_root = if merkle_tree_root_bytes.len() == 0 {
            None
        } else {
            Some(
                bitcoin::hashes::Hash::from_slice(&merkle_tree_root_bytes)
                    .expect("failed to create TapBranchHash"),
            )
        };

        pk.tap_tweak(&secp256k1_engine, merkle_root)
            .0
            .to_inner()
            .serialize()
    };

    let sig =
        k256::schnorr::Signature::try_from(sig_bytes).expect("failed to deserialize signature");

    let vk = k256::schnorr::VerifyingKey::from_bytes(&tweaked_pk_bytes)
        .expect("failed to deserialize BIP340 encoding into public key");

    let is_signature_valid = vk.verify_raw(&msg_bytes, &sig).is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}

fn verify_ed25519(
    sig_bytes: &[u8],
    msg_bytes: &[u8],
    pk_bytes: &[u8],
) -> Result<SignatureVerificationReply, String> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let pk: [u8; 32] = pk_bytes
        .try_into()
        .expect("ed25519 public key incorrect length");
    let vk = VerifyingKey::from_bytes(&pk).unwrap();

    let signature = Signature::from_slice(sig_bytes).expect("ed25519 signature incorrect length");

    let is_signature_valid = vk.verify(msg_bytes, &signature).is_ok();

    Ok(SignatureVerificationReply { is_signature_valid })
}
```

The call to `verify` function should always return `true` for correct parameters
and `false` or trap on errors otherwise.

Similar verifications can be done in many other languages with the help of
cryptographic libraries that support the `bip340secp256k1` signing *with
arbitrary message length* as specified in
[BIP340](https://github.com/bitcoin/bips/blob/master/bip-0340.mediawiki#user-content-Messages_of_Arbitrary_Size)/
[BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki) and
`ed25519` signing.

## Conclusion

In this walkthrough, we deployed a sample smart contract that:

* Signed with private Schnorr keys even though **canisters do not hold Schnorr keys themselves**.
* Requested a public key.
* Performed signature verification.
