# Basic Bitcoin Example

## Local deployment
Instructions on how to deploy this example locally can be found [here](https://internetcomputer.org/docs/current/developer-docs/integrations/bitcoin/local-development)

## Production deployment
Instructions on how to deploy this example on mainnet can be found [here](https://internetcomputer.org/docs/current/samples/deploying-your-first-bitcoin-dapp)

This project showcases a very basic example of how to write a canister
that can receive and send Bitcoin.

The example leverages the [ECDSA API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-ecdsa_public_key)
and [Bitcoin API](https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin-api) of the Internet Computer.

This example is extensively documented in the following tutorials:

* [Deploying Your First Bitcoin Dapp](https://internetcomputer.org/docs/current/samples/deploying-your-first-bitcoin-dapp)
* [Developing Bitcoin Dapps Locally](https://internetcomputer.org/docs/current/developer-docs/integrations/bitcoin/local-development)

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since the app e.g. offers method to read balances.
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralized control may be essential for canisters holding Bitcoin on behalf of users.