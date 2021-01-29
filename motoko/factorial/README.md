# Factorial

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
   dfx canister create factorial
   dfx build
   dfx canister install factorial
   dfx canister call factorial fac '(20)'
   ```

1. Observe the following result.

   ```
   (2_432_902_008_176_640_000)
   ```
