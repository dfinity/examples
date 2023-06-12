# Threshold ECDSA Signing Demo for Motoko

This example demonstrates how to request public key and signature signing from a Motoko canister, utilizing the latest [Threshold ECDSA] API of the Internet Computer.

The API comprises two methods, `ecdsa_public_key` for retrieving threshold ECDSA public keys, and `create_ecdsa_signature` for requesting threshold ECDSA signatures to be computed from the subnet holding the secret-shared private threshold ECDSA key. Their types are given below in [Candid format]:

```
ecdsa_public_key : (record {
    canister_id : opt canister_id;
    derivation_path : vec blob;
    key_id : record { curve: ecdsa_curve; name: text };
  }) -> (record { public_key : blob; chain_code : blob; });

sign_with_ecdsa : (record {
    message_hash : blob;
    derivation_path : vec blob;
    key_id : record { curve: ecdsa_curve; name: text };
  }) -> (record { signature : blob });
```

Each API call refers to a threshold ECDSA master key by virtue of a 2-part identifier comprising a curve and a key id as outlined above. Derivation paths are used to refer to keys below a canister\'s root key in the key derivation hierarchy. The key derivation from the master key to the canister root key is implicit in the API.

-   `ecdsa_public_key`: This method returns a SEC1-encoded ECDSA public key for the given canister using the given derivation path. If the `canister_id` is unspecified, it will default to the canister id of the caller. The `derivation_path` is a vector of variable length byte strings. The `key_id` is a struct specifying both a curve and a name. The availability of a particular `key_id` depends on implementation.<br/>
For `curve secp256k1`, the public key is derived using a generalization of BIP32 (see ia.cr/2021/1330, Appendix D). To derive (non-hardened) BIP-0032-compatible public keys, each byte string (blob) in the `derivation_path` must be a 4-byte big-endian encoding of an unsigned integer less than 2<sup>31</sup>.<br/>
The return result is an extended public key consisting of an ECDSA `public_key`, encoded in SEC1 compressed form, and a `chain_code`, which can be used to deterministically derive child keys of the `public_key`.\
This call requires that the ECDSA feature is enabled, and the `canister_id` meets the requirement of a canister id. Otherwise it will be rejected.
-   `sign_with_ecdsa`: This method returns a new ECDSA signature of the given `message_hash` that can be separately verified against a derived ECDSA public key. This public key can be obtained by calling `ecdsa_public_key` with the caller\'s `canister_id`, and the same `derivation_path` and `key_id` used here.<br/>
The signatures are encoded as the concatenation of the SEC1 encodings of the two values `r` and `s`. For curve `secp256k1`, this corresponds to 32-byte big-endian encoding.<br/>
This call requires that the ECDSA feature is enabled, the caller is a canister, and `message_hash` is 32 bytes long. Otherwise it will be rejected.

**Security Considerations and Security Best Practices**

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it makes inter-canister calls:
* [Be aware that state may change during inter-canister calls](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#be-aware-that-state-may-change-during-inter-canister-calls)
* [Only make inter-canister calls to trustworthy canisters](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#only-make-inter-canister-calls-to-trustworthy-canisters)
* [Don’t panic after await and don’t lock shared resources across await boundaries](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#dont-panic-after-await-and-dont-lock-shared-resources-across-await-boundaries)

**Usage**

The installation requires SDK version 0.11.1 or above. Simply run `dfx deploy` to deploy the example canister to to a locally running dfx instance.

There is also an script showing how to make calls to the canister to request a signature, and then verify the signature using the canister's public key.
It requires [Node.js] to be installed in your local environment.

Here is an example of installing node dependencies and running `test.sh`:

```
$ npm install
added 11 packages, and audited 12 packages in 544ms
found 0 vulnerabilities

$ ./test.sh
USAGE: ./test.sh <message to sign and verify>

$ ./test.sh "Hello World"
message=Hello World
signature_hex=334fa100f68367aa2892d75b614c2915ae573895922cc5e6a196984d65df25753e756cb1ae4d406dcebccdd23151545b960c2a9c92f35e885ccdd188fd513bb0
public_key_hex=02d98815741cae65d8bde06739e083584d3d83962ba623d8edd813656156f4c18a
verified =  true
```

**Deploy to Internet Computer**

The same code also works when deployed to the main Internet Computer network.
But please note you must edit the `key_id` name in the source code to refer to a key that exists on the main network.
As of writing, a test key called "`test_key_1`" can be used for testing purpose on the main network, and a production key `key_1` can be used for real integrations with Bitcoin mainnet and other use cases of interest.

[Node.js]: https://nodejs.org
[Threshold ECDSA]: https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key
[Candid format]: https://internetcomputer.org/docs/current/references/candid-ref
