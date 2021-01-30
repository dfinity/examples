# Factorial

![Compatibility](https://img.shields.io/badge/compatibility-0.6.20-blue)

## Prerequisites

Verify the following before running this demo.

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer related processes that may conflict
   with the following.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create --all
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install --all
   ```

1. Invoke the `fac` method of your canister.

   ```bash
   dfx canister call factorial fac '(20)'
   ```

1. Observe the following result.

   ```
   (2_432_902_008_176_640_000)
   ```
