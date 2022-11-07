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

## Dependencies
- [ic-cdk v0.6.5](https://crates.io/crates/ic-cdk/0.6.5) or above
- [dfx v0.12.0-beta.6](https://github.com/dfinity/sdk/releases/tag/0.12.0-beta.6) or above.
Use below command to install:
```DFX_VERSION=0.12.0-beta.6 sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"```
