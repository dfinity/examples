# ICP Transfer

ICP Transfer demonstrates how a canister can hold ICP and send it to other accounts using the [ICP ledger](https://dashboard.internetcomputer.org/canister/ryjl3-tyaaa-aaaaa-aaaba-cai).

## Account identifiers

The ICP ledger identifies accounts with a 32-byte **AccountIdentifier** — a hash of a principal and an optional subaccount. Centralized exchanges (CEXs) use this format for deposit addresses; wallets and newer integrations typically prefer the ICRC-1 account format (principal + subaccount directly). The `Principal.toLedgerAccount(subaccount)` method in Motoko performs this conversion.

The example exposes three functions to make this concrete:

- **`toAccountIdHex(principal, subaccount)`** — query returning the AccountIdentifier as a 64-char lowercase hex string, the format shown in block explorers and CEX deposit screens.
- **`transferToPrincipal(amount, principal, subaccount)`** — calls `toLedgerAccount` internally. Use this when you have a principal.
- **`transferToAccountId(amount, accountIdHex)`** — accepts the AccountIdentifier as a 64-char hex string (the format CEXs and block explorers provide).

> The ICP ledger also supports the [ICRC-1](https://github.com/dfinity/ICRC-1) standard via `icrc1_transfer`. For new token integrations that don't require AccountIdentifier compatibility, ICRC-1 is the recommended interface. A comprehensive ICRC ledger example is planned.

## Calling the ICP ledger

The backend reaches the ledger through the typed import `import IcpLedger "canister:icp_ledger"` — no ledger types are declared in the code. Because the ICP ledger lives at the **same well-known principal** (`ryjl3-tyaaa-aaaaa-aaaba-cai`) on both mainnet and the local development network, the `--actor-id-alias` flag in `mops.toml` binds the import to that fixed id and types it against the ledger's official Candid interface (`candid/icp_ledger.did`). The request/response types (`IcpLedger.Tokens`, `IcpLedger.TransferArgs`, …) come straight from that interface.

> `--actor-id-alias` fits here because the target's id is *fixed and universal*. When a target's id varies per environment, `--actor-env-alias` resolves it from an injected env var instead; when the target is chosen at runtime, generate bindings and construct `actor(principal)` per call.

`candid/icp_ledger.did` is the ledger's own interface, taken from the [ICP ledger suite release](https://github.com/dfinity/ic/releases/tag/ledger-suite-icp-2025-08-29) (`ledger.did`). To refresh it after a new ledger release, download the `ledger.did` asset from that release.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

`bash test.sh` funds the backend with 2 ICP, then:
1. Compares `icp identity account-id --format ledger` with `toAccountIdHex` to verify the CLI and the backend compute the same AccountIdentifier.
2. Calls `transferToPrincipal` — transfers 99_990_000 e8s (amount) + 10_000 e8s (fee) = exactly 1 ICP deducted from the backend; confirms both sides via `icp token balance`.
3. Calls `transferToAccountId` with the hex string from step 1 — same 1 ICP deduction, confirming both transfer paths reach the same account and the backend balance reaches zero.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app. For this example the following aspects are particularly relevant:

- [Securely handle traps in callbacks](https://docs.internetcomputer.org/guides/security/inter-canister-calls/#securely-handle-traps-in-callbacks): issues around inter-canister calls (here the ledger) can lead to time-of-check time-of-use or double-spending bugs.
- [Certified variables](https://docs.internetcomputer.org/guides/security/data-integrity-and-authenticity/#certified-variables): essential when displaying financial data that users act on.
