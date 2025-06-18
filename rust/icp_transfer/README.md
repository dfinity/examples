# ICP transfer

ICP transfer is a canister that can transfer ICP from its account to other accounts. It is an example of a canister that uses the ledger canister. Sample code is available in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/icp_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/icp_transfer).

The ICP ledger supports the ICRC1 standard, which is the recommended standard for token transfers. You can [read more about the differences](https://internetcomputer.org/docs/current/developer-docs/defi/overview) and find an example of how to transfer ICRC1 tokens from a canister in [Motoko](https://github.com/dfinity/examples/tree/master/motoko/token_transfer) and [Rust](https://github.com/dfinity/examples/tree/master/rust/token_transfer).

The sample code revolves around one core transfer function which takes as input the amount of ICP to transfer, the account (and optionally the subaccount) to which to transfer ICP and returns either success or an error in case e.g. the ICP transfer canister doesn’t have enough ICP to do the transfer. In case of success, a unique identifier of the transaction is returned. This identifier will be stored in the memo of the transaction in the ledger.

[Learn more about using the ICP ledger](https://internetcomputer.org/docs/defi/token-ledgers/usage/icp_ledger_usage).

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/icp_transfer)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Run `dfx start --background --clean && dfx deploy` to deploy the project to your local environment. 

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
