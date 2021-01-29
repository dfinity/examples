# Echo

![Compatibility](https://img.shields.io/badge/compatibility-0.6.20-blue)

## Prerequisites

Before building the example application, verify the following:

* You have downloaded and installed the DFINITY Canister SDK as described in [Download and install](https://sdk.dfinity.org/docs/quickstart/quickstart.html#download-and-install).
* You have stopped any Internet Computer network processes running on the local computer.

## Demo

1. Start a local internet computer.

   ```bash
   dfx start
   ```

1. Execute the following commands in another tab.

   ```bash
   dfx canister create echo
   dfx build
   dfx canister install echo
   dfx canister call echo say '("This is a test.")'
   ```

1. Observe the following result.

   ```
   ("This is a test.")
   ```
