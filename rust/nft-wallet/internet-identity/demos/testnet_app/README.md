# testnet_app

A minimal app that can be used to test that II is working on a given testnet.

It consists of a whoami Motoko backend and a small JS frontend served by an asset canister.

To deploy this app to the II testnet run the following commands from the II project root:
```bash
cd demos/testnet_app/
npm ci
dfx deploy --network identity
```

## Updating DFX

When a new `dfx` version is released, follow these steps:

1. Download the latest DFX executable.
1. Update the version number in `dfx.json` (`"dfx": "..."`).
1. Copy the `webpack.config.js` generated from a fresh `dfx new`.
1. Run `dfx generate` to get the latest candid bindings.
