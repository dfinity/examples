# Exchange Rates Canister

A Rust canister demonstrating how to query the Internet Computer Exchange Rate Canister (XRC) to get cryptocurrency and fiat currency exchange rates. The XRC canister (`uf6dk-hyaaa-aaaaq-qaaaq-cai`) is a system canister on the IC mainnet that returns real-time exchange rates sourced via HTTPS outcalls to external data providers, with decentralized consensus across the subnet.

> **Note:** Live exchange rate queries require deployment to the IC mainnet — the XRC canister does not exist on a local replica. Local deployment verifies the canister builds and runs correctly; calling `get_exchange_rate` on local will trap as expected.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/exchange-rates
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

### Query on mainnet

To query live exchange rates, deploy to the IC mainnet and call:

```bash
icp deploy -e ic
icp canister call -e ic backend get_exchange_rate \
  '(record { symbol = "ICP"; class = variant { Cryptocurrency } }, \
    record { symbol = "USD"; class = variant { FiatCurrency } })'
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
