Welcome to the `motoko` version of `exchange_rate` sample dapp for Canister HTTP feature.

This code demonstrates usage of Canister HTTP feature with `motoko` language. There is 
also demonstration usage with `rust` language, please check out `/rust/exchange_rate` folder. 
The two versions of backend canister implementation produces exact same Candid APIs, and reuses
the frontend canister implementation is in `/rust/exchange_rate/src/frontend` folder. 

For deploying this `motoko` version of dapp, please run: `./deploy.sh {env}`, with `env` being
the environment you intend to deploy the dapp to. For local environment, the `deploy.sh` script
will create local replica environment before deploying the dapp onto it.

With more background already covered in `/rust/exchange_rate/README.md`, the README file is
intentionally kept short.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspect is particularly relevant for this app:
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data (in this case exchange rates) in the frontend that may be used by users to decide on future transactions (based on the rate information).

## Dependencies
- [ic-cdk v0.6.5](https://crates.io/crates/ic-cdk/0.6.5) or above
- [dfx v0.12.0](https://github.com/dfinity/sdk/releases/tag/0.12.0) or above.
