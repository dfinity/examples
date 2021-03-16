# Who Am I?

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-whoami-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-whoami-example)

This example demonstrates how a canister can identify its caller and itself.

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

1. Invoke the `whoami` method.

   ```text
   dfx canister call whoami whoami
   ```

1. Observe your principal identifier.


1. Invoke the `id` method.

   ```text
   dfx canister call whoami id
   ```

1. Observe the principal identifier of your canister.
