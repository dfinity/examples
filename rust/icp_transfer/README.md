# ICP Transfer

ICP Transfer demonstrates how a canister can hold ICP and send it to other accounts using the [ICP ledger](https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai). The same example is also available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/icp_transfer).

## Account identifiers

The ICP ledger identifies accounts with a 32-byte **AccountIdentifier** — a hash of a principal and an optional subaccount. Centralized exchanges (CEXs) use this format for deposit addresses; wallets and newer integrations typically prefer the ICRC-1 account format (principal + subaccount directly). `AccountIdentifier::new(&principal, &subaccount)` in Rust performs this conversion.

The example exposes three functions to make this concrete:

- **`to_account_id_hex(principal, subaccount)`** — query returning the AccountIdentifier as a 64-char lowercase hex string, the format shown in block explorers and CEX deposit screens.
- **`transfer_to_principal(amount, principal, subaccount)`** — calls `AccountIdentifier::new` internally. Use this when you have a principal.
- **`transfer_to_account_id(amount, account_id)`** — accepts the raw 32-byte blob directly. Use this when an exchange or external service gives you the destination as an AccountIdentifier rather than as a principal.

> The ICP ledger also supports the [ICRC-1](https://github.com/dfinity/ICRC-1) standard via `icrc1_transfer`. For new token integrations that don't require AccountIdentifier compatibility, ICRC-1 is the recommended interface. A comprehensive ICRC ledger example is planned.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

`make test` funds the backend with 2 ICP, then:
1. Compares `icp identity account-id --format ledger` with `to_account_id_hex` to verify CLI and backend compute the same AccountIdentifier.
2. Calls `transfer_to_principal` — transfers 99_990_000 e8s (amount) + 10_000 e8s (fee) = exactly 1 ICP deducted from the backend; confirms both sides via `icp token balance`.
3. Calls `transfer_to_account_id` with the AccountIdentifier as `vec { N : nat8; ... }` — same 1 ICP deduction, confirming both transfer paths reach the same account and the backend balance reaches zero.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app. For this example the following aspects are particularly relevant:

- [Inter-canister calls and rollbacks](https://docs.internetcomputer.org/guides/security/overview): issues around inter-canister calls (here the ledger) can lead to time-of-check time-of-use or double-spending bugs.
- [Certify query responses if they are relevant for security](https://docs.internetcomputer.org/guides/security/overview): essential when displaying financial data that users act on.
