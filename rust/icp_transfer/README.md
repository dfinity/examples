# ICP transfer

ICP transfer is a canister that can transfer ICP from its account to other accounts. It is an example of a canister that uses the ICP ledger. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/icp_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

> **Note:** The ICP ledger also supports the ICRC-1 standard, which is the recommended standard for new token integrations. You can [read more about the differences](https://internetcomputer.org/docs/current/developer-docs/defi/overview) and find examples of how to transfer ICRC-1 tokens from a canister in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer).

## Architecture

The sample code revolves around one core `transfer` function which takes as input the amount of ICP to transfer, the destination account (and optionally a subaccount), and returns either a unique block index on success or an error string on failure. The block index is stored in the transaction memo in the ledger, making every transfer auditable.

The canister uses `MAINNET_LEDGER_CANISTER_ID` from `ic-ledger-types`, which is the well-known principal `ryjl3-tyaaa-aaaaa-aaaba-cai`. In production this resolves to the real ICP ledger; in local testing the ledger is deployed at the same principal ID using `provisional_create_canister_with_cycles`.

> **Important:** Transfers from the minting account create Mint transactions. Transfers to the minting account create Burn transactions.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- Python 3 (for computing ledger account identifiers in `make test`)

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/icp_transfer
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

`make test` performs the full end-to-end workflow:

1. Downloads the ICP ledger WASM and deploys it locally at `ryjl3-tyaaa-aaaaa-aaaba-cai`
2. Seeds the default identity with 100 ICP (1 ICP = 10^8 e8s)
3. Funds the backend canister with 1 ICP via the ledger
4. Calls `backend.transfer` to send 0.5 ICP back to the default identity
5. Verifies each step produces the expected result

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:

- **Inter-canister calls and rollbacks**: issues around inter-canister calls (here the ledger) can lead to time-of-check time-of-use or double spending security bugs.
- **Certify query responses if they are relevant for security**: this is essential when displaying important financial data in a frontend that may be used to inform future transactions.
- **Use a decentralized governance system like SNS to make a canister have a decentralized controller**: decentralizing control is a fundamental aspect of decentralized finance applications.
