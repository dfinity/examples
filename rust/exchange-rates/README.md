# Exchange Rates Canister

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/rust/exchange-rates)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

A Rust canister demonstrating how to query the [Exchange Rate Canister (XRC)](https://docs.internetcomputer.org/concepts/chain-fusion/exchange-rate-canister/) to get cryptocurrency and fiat currency exchange rates.

The XRC returns real-time rates sourced via HTTPS outcalls to external data providers, with decentralized consensus across the subnet.

For local development, an XRC mock canister is deployed automatically — it returns a fixed rate without making any HTTPS outcalls. On the IC mainnet, the production XRC canister (`uf6dk-hyaaa-aaaaq-qaaaq-cai`) is used instead.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/exchange-rates
```

### Deploy and test locally

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

The local deployment includes the XRC mock canister. `icp-cli` automatically injects the mock's canister ID into the backend via the `PUBLIC_CANISTER_ID:xrc` environment variable.

### Deploy to mainnet

To query live exchange rates, deploy to the IC mainnet:

```bash
icp deploy -e ic
icp canister call -e ic backend get_exchange_rate '(record { symbol = "ICP"; class = variant { Cryptocurrency } }, record { symbol = "USD"; class = variant { FiatCurrency } })'
```

On mainnet, the production XRC canister ID (`uf6dk-hyaaa-aaaaq-qaaaq-cai`) is injected automatically via `icp.yaml`.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
