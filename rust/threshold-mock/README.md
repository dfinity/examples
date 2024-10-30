# Chain-key testing canister

The chain-key testing canister is a canister smart contract that provides a [fake](https://www.martinfowler.com/articles/mocksArentStubs.html#TheDifferenceBetweenMocksAndStubs) implementation of the APIs of various chain-key-related features including threshold ECDSA, threshold Schnorr, and threshold key derivation (vetKeys) **for testing purposes**. The shortcut that this canister takes (compared to the production APIs) is that it relies on a cryptographic key that is hard-coded into the canister, rather than relying on a key that is distributed among the subnet nodes. With this, the implementation is inherently **insecure**: see [Disclaimer](#disclaimer).

The main advantage of using this canister is to *save costs during testing*, because there are [fees](https://internetcomputer.org/docs/current/references/t-sigs-how-it-works#api-fees) associated with using the chain-key features via the Internet Computer's management canister APIs.

In particular, the canister provides fake implementations of the following Internet Computer features:
* [Threshold ECDSA](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/signatures/t-ecdsa)
    * [ecdsa_public_key](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-ecdsa_public_key), [sign_with_ecdsa](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-sign_with_ecdsa)
* [Threshold Schnorr](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/signatures/t-schnorr)
    * [schnorr_public_key](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-schnorr_public_key), [sign_with_schnorr](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-sign_with_schnorr)
* Preview: [Threshold Key Derivation (vetKeys)](https://internetcomputer.org/docs/current/developer-docs/smart-contracts/encryption/vetkeys)
    * `vetkd_public_key`, `vetkd_encrypted_key` (see [API proposal PR](https://github.com/dfinity/interface-spec/pull/158))

## Usage

The canister is deployed on mainnet: the canister ID is `vrqyr-saaaa-aaaan-qzn4q-cai` ([dashboard](https://dashboard.internetcomputer.org/canister/vrqyr-saaaa-aaaan-qzn4q-cai), [candid-ui](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=vrqyr-saaaa-aaaan-qzn4q-cai)).

All APIs support a single key ID: `insecure_test_key_1`.

For the time being, no fees are charged. If canister usage becomes excessive, we will introduce fees but aim to keep these fees significantly lower than the [fees of the production APIs](https://internetcomputer.org/docs/current/references/t-sigs-how-it-works#api-fee). The community is invited to top up the canister with cycles.

As this repository contains the canister's source code, developers can also deploy their own, private instance of this canister.

## Disclaimer

The implementation underlying the chain-key testing canister is **unsafe** and for **testing purposes only**: the master secret keys are **hard-coded** in the canister, rather than distributed among the subnet nodes. **Do not use this in production or for sensitive data**!.

## License

Please see the [LICENSE](LICENSE) and [Contribution guidelines](CONTRIBUTING.md).