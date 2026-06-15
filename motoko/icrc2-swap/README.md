# ICRC-2 Swap

This example demonstrates how to safely work with ICRC-2 tokens on the Internet Computer. The swap canister handles depositing, swapping, and withdrawing ICRC-2 tokens using a simple 1:1 swap mechanism, illustrating correct design patterns for inter-canister calls in an asynchronous environment.

The asynchronous nature of the Internet Computer presents unique challenges compared to synchronous blockchains. This example highlights:

- **Deposit Tokens**: Users approve the swap canister to transfer tokens on their behalf (ICRC-2 `approve`), then call `deposit` to move tokens into the swap canister.
- **Swap Tokens**: Users swap their token balances 1:1. The `swap` function executes atomically (no `await` calls) to ensure consistency.
- **Withdraw Tokens**: Users withdraw their resulting token balances back to their wallet.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/icrc2-swap
```

### Deploy and test

```bash
icp network start -d
make deploy
make test
icp network stop
```

The `make deploy` target:
1. Deploys the two ICRC-1/ICRC-2 token ledger canisters (`token_a` and `token_b`) with the current identity as the minting account.
2. Deploys the `backend` (swap) canister with the token canister IDs as init args.

## Architecture

This example uses three canisters:

- **token_a** / **token_b**: Standard ICRC-1/ICRC-2 ledger canisters (pre-built from the DFINITY IC release).
- **backend**: The swap canister (`backend/app.mo`). Accepts deposits from both token ledgers, performs 1:1 swaps between users, and allows withdrawals.

## Known issues and limitations

- Any DeFi on the Internet Computer is experimental and should be treated as such.
- Due to asynchronous inter-canister messaging, malicious token canisters could cause this swap contract to deadlock. Only use with trusted token canisters.
- There are no limits on the state size of this canister. For a production canister, calculate and enforce a maximum state size.
- **Async Bug Trap**: The `deposit` function calls `icrc2_transfer_from`, but there is no guarantee that the callback will execute correctly if the canister runs out of cycles or encounters other side effects. To address these issues, refer to the [inter-canister calls security best practices](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/inter-canister-calls).

## Security considerations and best practices

When building production applications, review the [ICP security best practices](https://docs.internetcomputer.org/guides/security/overview).
