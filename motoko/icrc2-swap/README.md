# ICRC-2 Swap

This example demonstrates how to safely work with [ICRC-2](https://docs.internetcomputer.org/references/digital-asset-standards) tokens on the Internet Computer, focusing on two critical inter-canister call safety patterns that differ from synchronous blockchains.

> Originally contributed by [0xAegir](https://github.com/AegirFinance).

## Key safety patterns

### 1. Debit before transfer (withdraw)

When sending tokens out of the canister, **deduct the user's internal balance first**, then perform the transfer on the token ledger. If the order is reversed and the transfer executes before the debit, a concurrent or reentering call could withdraw the same tokens twice.

See `backend/app.mo` `withdraw` for the implementation and detailed inline comments.

### 2. Atomic swap (no `await` in swap)

The `swap` function exchanges two users' balances **without any `await` calls**. On the IC, an `await` creates a commit point — if the function fails after an `await`, only the changes before it persist, leaving state inconsistent. By keeping `swap` entirely synchronous, either all balance changes apply or none do.

See `backend/app.mo` `swap` for the implementation and detailed inline comments.

For more background, see the [inter-canister calls security best practices](https://docs.internetcomputer.org/guides/security/inter-canister-calls).

## Architecture

Three canisters:

- **`token_a` / `token_b`**: Standard ICRC-1/ICRC-2 ledger canisters, pre-built from the DFINITY IC release.
- **`backend`**: The swap canister (`backend/app.mo`). Accepts deposits, performs 1:1 swaps, and processes withdrawals. It discovers the token canister principals automatically at runtime via `PUBLIC_CANISTER_ID:token_a` / `PUBLIC_CANISTER_ID:token_b` environment variables injected by icp-cli.

The backend imports the ICRC-1/ICRC-2 interface **directly from the committed Candid file** `candid/icrc.did`, using Motoko's `idl:` import (moc 1.13.0+):

```motoko
import ICRC "idl:../candid/icrc.did";
```

This yields the interface's named types and its service type `ICRC.Self` — no bindings are generated or committed, and no extra tooling is needed. The `idl:` import provides *types only*, so the backend supplies its own actor reference per token: `actor(<token-principal>) : ICRC.Self`. Every token reference is typed against that one shared type, so the same interface serves both `token_a` and `token_b` (and would serve any ICRC-1/2 ledger the backend is pointed at, e.g. ckBTC or an SNS token). This is the "one interface, many ledgers, principal chosen at runtime" pattern: because the target is dynamic, the types come from the standard Candid interface rather than being bound to a single canister id.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/icrc2-swap
```

### Deploy and test

```bash
icp network start -d
bash deploy.sh
bash test.sh
icp network stop
```

> **Use `bash deploy.sh`, not `icp deploy`.** The ICRC-1 ledger canisters require init args (initial balances, minting account) that include the `icrc2-alice` and `icrc2-bob` principals, which are only available after the identities are created by `bash deploy.sh`.

`bash deploy.sh`:
1. Creates two example identities (`icrc2-alice`, `icrc2-bob`) if they don't already exist.
2. Deploys `token_a` pre-funded for `icrc2-alice` and `token_b` pre-funded for `icrc2-bob`.
3. Deploys `backend` — no init args needed; it discovers the token principals via injected environment variables.

`bash test.sh` runs the full swap flow with `icrc2-alice` and `icrc2-bob` as the two parties. Test 2 verifies that swapping with no deposits returns `InsufficientBalance`. Tests 6 and 7 verify the actual token balance delta in the ledger after withdrawal, confirming the full round-trip. Tests are idempotent — they can be run multiple times without redeploying.

## Updating the token interface

`candid/icrc.did` is the ICRC-1/ICRC-2 interface the backend calls. It is imported directly via the `idl:` import (see [Architecture](#architecture)), so there is nothing to regenerate and no extra tooling to install — just edit `candid/icrc.did` and rebuild. moc reads it at build time and derives the Motoko types (snake_case Candid names become PascalCase automatically).

## Fee handling

ICRC-1 tokens charge a `transfer_fee` (10,000 e8s in this example) on every transfer through the ledger.

- **Deposit approve**: `approve` amount = deposit amount + fee (e.g. `100_010_000` to deposit `100_000_000`).
- **Withdrawal**: The backend deducts `amount + fee` from the user's internal balance before sending. To withdraw the full deposited amount you must leave enough to cover the fee (e.g. withdraw `99_990_000` when internal balance is `100_000_000`).

## Known limitations

- **Trusted token canisters only.** A malicious token ledger could trap during `icrc1_transfer` or `icrc2_transfer_from`. For `withdraw`, the balance is debited before the transfer call; a trap is caught and a refund is attempted, but the `try/catch` itself could theoretically trap in extreme circumstances. For `deposit`, if `icrc2_transfer_from` succeeds in the ledger but the ledger traps before sending the response, the canister receives no callback and the user's tokens are moved in the ledger but not credited internally. These are fundamental async messaging edge cases on the IC — always use trusted, audited token canisters.
- **No state size cap.** Each user's balance entry stays in the map. A production deployment should enforce per-user deposit limits.

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) and [inter-canister calls security best practices](https://docs.internetcomputer.org/guides/security/inter-canister-calls).
