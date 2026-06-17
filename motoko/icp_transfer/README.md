# ICP Transfer

ICP Transfer demonstrates how a canister can hold ICP and send it to other accounts using the [ICP ledger](https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai). The same example is also available in [Rust](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

## Account identifiers

The ICP ledger identifies accounts with a 32-byte **AccountIdentifier** — a hash of a principal and an optional subaccount. This is the native format used by most exchanges and wallets. The `Principal.toLedgerAccount(subaccount)` method in Motoko (or `AccountIdentifier::new` in Rust) performs this conversion.

The example exposes three functions to make this concrete:

- **`toAccountId(principal, subaccount)`** — query that returns the AccountIdentifier blob for any principal + subaccount pair, so you can inspect what the conversion produces.
- **`transferToPrincipal(amount, principal, subaccount)`** — calls `toLedgerAccount` internally. Use this when you have a principal.
- **`transferToAccountId(amount, accountId)`** — accepts the blob directly. Use this when an exchange or external service gives you an account identifier rather than a principal.

The `make test` target calls both transfer functions for the same recipient and verifies that the balance deltas are identical, demonstrating the equivalence of the two paths.

> The ICP ledger also supports the [ICRC-1](https://github.com/dfinity/ICRC-1) standard via `icrc1_transfer`. For new token integrations that don't require AccountIdentifier compatibility, ICRC-1 is the recommended interface. A comprehensive ICRC ledger example is planned.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/icp_transfer
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

`make test` funds the backend with 4 ICP, then:
1. Calls `toAccountId` to show the AccountIdentifier blob for the caller's principal.
2. Calls `transferToPrincipal` (principal path) and checks the balance delta on both sides via `icrc1_balance_of`.
3. Calls `transferToAccountId` with the blob from step 1 (account identifier path) and checks the same deltas — confirming both paths reach the same account.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app. For this example the following aspects are particularly relevant:

- [Inter-canister calls and rollbacks](https://docs.internetcomputer.org/guides/security/overview): issues around inter-canister calls (here the ledger) can lead to time-of-check time-of-use or double-spending bugs.
- [Certify query responses if they are relevant for security](https://docs.internetcomputer.org/guides/security/overview): essential when displaying financial data that users act on.
