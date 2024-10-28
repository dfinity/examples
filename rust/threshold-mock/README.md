---
keywords: [advanced, rust, vetkeys, vetkd]
---

# Mock canister for tECDSA, tSchnorr, and vetKD

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/threshold-mock)

Provides an insecure mock implementation of the following features of the Internet Computer for testing purpuses:
* [Threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/signatures/t-ecdsa)
    * `ecdsa_public_key`, `sign_with_ecdsa`
* [Threshold Schnorr](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/signatures/t-schnorr)
    * `schnorr_public_key`, `sign_with_schnorr`
* Preview: [Threshold Key Derivation (vetKeys)](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/vetkeys)
    * `vetkd_public_key`, `vetkd_encrypted_key`

## Mainnet deployment

On the mainnet, the code of this **insecure** canister (see Disclaimer below) is deployed as canister with ID `vrqyr-saaaa-aaaan-qzn4q-cai` ([dashboard](https://dashboard.internetcomputer.org/canister/vrqyr-saaaa-aaaan-qzn4q-cai), [candid-ui](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=vrqyr-saaaa-aaaan-qzn4q-cai))

## Key IDs

All APIs support a single key ID: `insecure_mock_key_1`.

## Disclaimer

The implementation of the above-mentioned APIs is **unsafe** and for **testing purposes only**: the master secret keys are hard-coded in the canister, rather than distributed among the subnet nodes. **Do not use this in production or for sensitive data**!.