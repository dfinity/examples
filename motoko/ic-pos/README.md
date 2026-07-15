# IC-POS

![](./media/header.png)

IC-POS is a simple Point of Sale (POS) app that accepts [ICRC-1](https://github.com/dfinity/ICRC-1) token payments on the Internet Computer. A merchant logs in with [Internet Identity](https://internetcomputer.org/internet-identity), configures a store, and charges customers by showing a QR code; incoming payments show up in the store with a live balance and transaction history.

To let you try the full flow without spending real funds, this example uses the **TICRC1 test token** ([`3jkp5-oyaaa-aaaaj-azwqa-cai`](https://dashboard.internetcomputer.org/canister/3jkp5-oyaaa-aaaaj-azwqa-cai)) on mainnet, and a local ICRC-1 ledger + index when developing. Because the app talks to any ICRC-1 ledger, the same code works with real tokens (e.g. [ckBTC](https://internetcomputer.org/ckbtc/)) by pointing it at a different ledger.

> This example was created by [Kristofer Lund](https://github.com/kristoferlund) as a ckBTC point-of-sale demo and has been adapted here to run on [icp-cli](https://github.com/dfinity/icp-cli) with the TICRC1 test token.

## Features

- **Create store**: Log in with Internet Identity and configure the store with a name and notification settings.
- **Charge customer**: Enter an amount to generate a payment QR code (following the [ICRC-22](https://github.com/dfinity/ICRC/issues/22) standard) for the customer to scan and pay.
- **Send tokens**: Send tokens to other principals from within the app.
- **Transaction history**: View recent transactions and a live balance for the store, queried from the ICRC-1 index canister.

## How it works

### Backend (`src/icpos/`)

The backend is a single Motoko canister, `icpos`. It stores per-merchant configuration (`getMerchant` / `updateMerchant`) and runs a [timer](https://internetcomputer.org/docs/motoko/timers) that monitors the ICRC-1 ledger for incoming transfers. When a payment to a configured merchant is detected, it writes a log entry (`getLogs`) noting that a notification could be sent.

The ledger canister is resolved at runtime from the `PUBLIC_CANISTER_ID:icrc1_ledger` environment variable injected by icp-cli (the local ledger when developing, TICRC1 on mainnet).

> **Notifications.** The original app sent email/SMS via an HTTPS outcall to a third-party service. This version instead logs where a notification would be sent. To implement real notifications, use [HTTPS outcalls](https://docs.internetcomputer.org/guides/backends/https-outcalls).
>
> **Note.** The monitor scans the ledger's global transaction log sequentially, which is illustrative rather than production-grade — it does not scale to a busy shared ledger. A production app would query the index canister per merchant account (as this app's frontend already does).

### Frontend (`src/icpos_frontend/`)

A TypeScript + React + Vite + Tailwind app. It authenticates with Internet Identity, calls `icpos` for store configuration, and uses the ICRC-1 ledger (balance, transfers) and index (transaction history) canisters. Canister IDs are read at runtime from the environment injected by icp-cli — there are no hardcoded IDs.

## Build and deploy locally

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Deploy

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/ic-pos
icp network start -d
bash deploy.sh
```

`deploy.sh` installs the local ICRC-1 ledger + index, Internet Identity, the `icpos` backend, and the frontend. It sets the default identity as the ledger's minting account and pre-funds a separate `ic-pos-dev` identity with test tokens. Open the frontend URL printed by the script.

### Try a payment

1. Open the frontend, log in with Internet Identity, and give your store a name.
2. On the store page, click the principal pill to copy your store's principal.
3. Send tokens to the store from the pre-funded `ic-pos-dev` identity using icp-cli:

   ```bash
   icp canister call icrc1_ledger icrc1_transfer \
     '(record { to = record { owner = principal "<STORE_PRINCIPAL>" }; amount = 100_000 : nat })' \
     --identity ic-pos-dev
   ```

The payment appears in the store, and the backend logs a would-be notification (`icp canister call icpos getLogs`).

For frontend hot-reload development after deploying: `npm run dev`.

### Run the automated test

```bash
bash test.sh
```

`test.sh` configures a merchant and performs a real token payment via icp-cli, asserting the merchant's balance increases.

## Deploy to mainnet

```bash
icp deploy -e ic
```

On mainnet the app uses the shared **TICRC1** test token (ledger [`3jkp5-oyaaa-aaaaj-azwqa-cai`](https://dashboard.internetcomputer.org/canister/3jkp5-oyaaa-aaaaj-azwqa-cai), index [`qzre3-3iaaa-aaaai-aqmsa-cai`](https://dashboard.internetcomputer.org/canister/qzre3-3iaaa-aaaai-aqmsa-cai)) and the production Internet Identity — the local ledger, index, and II are not deployed.

To get TICRC1 tokens to test with:

1. Obtain your principal — for example from [OISY Wallet](https://oisy.com) or with `icp identity principal`.
2. Request TICRC1 tokens from the [faucet](https://faucet.internetcomputer.org) using that principal.
3. Send tokens to a store principal from your wallet (e.g. OISY) or with `icp canister call ... icrc1_transfer` against the TICRC1 ledger.

## Updating the Candid interface

The `src/icpos/icpos.did` file defines the backend's public interface; the frontend bindings are generated from it during the build. If you change the backend's public API, regenerate it:

```bash
mops generate candid icpos
```

## Possible improvements

- A transaction detail page and pagination (currently only the latest transactions are shown).
- A confirmation dialog before sending.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Author

Original IC-POS by [Kristofer Lund](https://github.com/kristoferlund) — [kristofer@fmckl.se](mailto:kristofer@fmckl.se).

## License

[MIT](https://github.com/dfinity/examples/blob/master/motoko/ic-pos/LICENSE)
