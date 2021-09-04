# Counter

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-counter-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-counter-example)

This example demonstrates a counter application. It uses an orthogonally
persistent `counter` variable to store an arbitrary precision natural number
that represents the current value of the counter.

By using the Motoko keyword `stable` when declaring the `counter` variable,
the value of this variable will automatically be preserved whenever your canister code is
upgraded. Without the `stable` keyword, a variable is deemed `flexible`, and its value
is reinitialized on every canister upgrade, i.e. whenever new code is deployed to the canister.

To learn more about these features of Motoko, see:
* https://sdk.dfinity.org/docs/language-guide/motoko.html#_orthogonal_persistence
* https://sdk.dfinity.org/docs/language-guide/upgrades.html#_declaring_stable_variables

## Introduction

The application provides an interface that exposes the following methods:

*  `set`, which sets the value of the counter;

*  `inc`, which increments the value of the counter; and

*  `get`, which gets the value of the counter.

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
   dfx canister create counter
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install counter
   ```

1. Set the value of the counter.

   ```text
   dfx canister call counter set '(7)'
   ```

1. Increment the value of the counter.

   ```text
   dfx canister call counter inc
   ```

1. Get the value of the counter.

   ```text
   dfx canister call counter get
   ```

1. Observe the following result.

   ```
   (8)
   ```
