# Factorial

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-factorial-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-factorial-example)

This example demonstrates a recursive mathematical function that calculates the
product of all positive integers less than or equal to its input.

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
   dfx canister create factorial
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install factorial
   ```

1. Calculate the factorial of 20.

   ```text
   dfx canister call factorial fac '(20)'
   ```

1. Observe the following result.

   ```text
   (2_432_902_008_176_640_000)
   ```
