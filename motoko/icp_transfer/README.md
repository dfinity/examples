# ICP Transfer

ICP Transfer demonstrates how a canister can hold ICP and send it to other accounts using the [ICP ledger](https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai). The same example is also available in [Rust](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

## Account identifiers

The ICP ledger identifies accounts with a 32-byte **AccountIdentifier** — a hash of a principal and an optional subaccount. Centralized exchanges (CEXs) use this format for deposit addresses; wallets and newer integrations typically prefer the ICRC-1 account format (principal + subaccount directly). The `Principal.toLedgerAccount(subaccount)` method in Motoko performs this conversion.

The example exposes three functions to make this concrete:

- **`toAccountIdHex(principal, subaccount)`** — query returning the AccountIdentifier as a 64-char lowercase hex string, the format shown in block explorers and CEX deposit screens.
- **`transferToPrincipal(amount, principal, subaccount)`** — calls `toLedgerAccount` internally. Use this when you have a principal.
- **`transferToAccountId(amount, accountId)`** — accepts the raw blob directly. Use this when an exchange or external service gives you the destination as an AccountIdentifier rather than as a principal.

> The ICP ledger also supports the [ICRC-1](https://github.com/dfinity/ICRC-1) standard via `icrc1_transfer`. For new token integrations that don't require AccountIdentifier compatibility, ICRC-1 is the recommended interface. A comprehensive ICRC ledger example is planned.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

`make test` funds the backend with 2 ICP, then:
1. Compares `icp identity account-id --format ledger` with `toAccountIdHex` to verify the CLI and the backend compute the same AccountIdentifier.
2. Calls `transferToPrincipal` — transfers 99_990_000 e8s (amount) + 10_000 e8s (fee) = exactly 1 ICP deducted from the backend; confirms both sides via `icp token balance`.
3. Calls `transferToAccountId` with the AccountIdentifier as `vec nat8` — same 1 ICP deduction, confirming both transfer paths reach the same account and the backend balance reaches zero.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app. For this example the following aspects are particularly relevant:

- [Securely handle traps in callbacks](https://docs.internetcomputer.org/guides/security/inter-canister-calls/#securely-handle-traps-in-callbacks): issues around inter-canister calls (here the ledger) can lead to time-of-check time-of-use or double-spending bugs.
- [Certified variables](https://docs.internetcomputer.org/guides/security/data-integrity-and-authenticity/#certified-variables): essential when displaying financial data that users act on.
