# Echo

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-echo-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-echo-example)

This example demonstrates a simple echo effect, where an application sends back
the data it receives.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create echo
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install echo
   ```

1. Invoke the `say` method.

   ```text
   dfx canister call echo say '("This is a test.")'
   ```

1. Observe the following result.

   ```text
   ("This is a test.")
   ```
