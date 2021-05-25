# Classes

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-echo-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-echo-example)

This example demonstrates a simple use of actor classes, which allow a program to dynamically install new actors (i.e. canisters).

The example define two Motoko actors (i.e. canisters), `Map` and `Test`.

`Map` is a dead simple, distributed key-value store, mapping `Nat` to `Text` values.

[Map.mo](./src/map/Map.mo) imports a Motoko actor class `Bucket(i, n)`
from library [Buckets.mo](./src/map/Buckets.mo).
It also imports the `ExperimentalCycles` base library in order to share its
cycles amongst the bucket it creates.

Each call to `Buckets.Bucket(n, i)` within `Map` instantiates a new
`Bucket` instance (the `i`-th of `n`)
dedicated to those entries of the `Map` that hash to `i` (by simple division of the key modulo `n`).

Each asynchronous instantiation of the actor class corresponds to the dynamic, programmatic installation of a new `Bucket` canister.

Each new Bucket must be provisioned with enough cycles to pay for installation.
`Map.mo` achieves this by `add`-ing an equal share of `Map`'s initial Cycle balance to each asynchronous call to `Bucket(n, i)`.

The `Test` in [Test.mo](./src/test/Test.mo) canister imports the `Map` canister.
Its `run` method simply `put`s 24 consecutive entries into `Map`. The entries are distributed evenly amongst the buckets making up the key-value store.

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

2. Open a new terminal window.

3. Deploy the canisters `Map` and `Test`

   ```text
   dfx deploy
   ```

4. Invoke the `run` method of canister `Test`

   ```text
   dfx canister call Test run '()'
   ```

5. Observe the following result.

   ```text
   debug.print: putting: (0, "0")
   debug.print: putting: (1, "1")
   debug.print: putting: (2, "2")
   debug.print: putting: (3, "3")
   debug.print: putting: (4, "4")
   debug.print: putting: (5, "5")
   debug.print: putting: (6, "6")
   debug.print: putting: (7, "7")
   debug.print: putting: (8, "8")
   debug.print: putting: (9, "9")
   debug.print: putting: (10, "10")
   debug.print: putting: (11, "11")
   debug.print: putting: (12, "12")
   debug.print: putting: (13, "13")
   debug.print: putting: (14, "14")
   debug.print: putting: (15, "15")
   debug.print: putting: (16, "16")
   debug.print: putting: (17, "17")
   debug.print: putting: (18, "18")
   debug.print: putting: (19, "19")
   debug.print: putting: (20, "20")
   debug.print: putting: (21, "21")
   debug.print: putting: (22, "22")
   debug.print: putting: (23, "23")
   ()
   ```

# Links

Specific links:

- [Actor classes](https://sdk.dfinity.org/docs/language-guide/actor-classes.html)
- [Managing Cycles](https://sdk.dfinity.org/docs/language-guide/cycles.html)

General background:

- [Manage Canisters](https://sdk.dfinity.org/docs/developers-guide/working-with-canisters.html)
- [Quick Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Guide](https://sdk.dfinity.org/language-guide)
