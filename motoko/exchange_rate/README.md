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
