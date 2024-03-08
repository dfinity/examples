---
keywords: [beginner, motoko, classes, actor classes]
---

# Classes

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/classes)

## Overview

This example demonstrates a simple use of actor classes, allowing a program to dynamically install new actors (that is, canisters). It also demonstrates a multi-canister project, and actors using inter-actor communication through shared functions.

The example defines two Motoko actors, `Map` and `Test`.

Map is a simple, distributed key-value store, mapping `Nat` to `Text` values, with entries stored in a small number of separate Bucket actors, installed on demand.

`Map.mo` imports a Motoko actor class `Bucket(i, n`) from the library `Buckets.mo`. It also imports the `ExperimentalCycles` base library, naming it `Cycles` for short, to share its cycles amongst the buckets it creates.

Each call to `Buckets.Bucket(n, i)` within Map instantiates a new Bucket instance (the i-th of n) dedicated to those entries of the Map whose key hashes to i (by taking the remainder of the key modulo division by n).

Each asynchronous instantiation of the Bucket actor class corresponds to the dynamic, programmatic installation of a new Bucket canister.

Each new Bucket must be provisioned with enough cycles to pay for its installation and running costs. Map achieves this by adding an equal share of Map's initial cycle balance to each asynchronous call to `Bucket(n, i)`, using a call to `Cycles.add(cycleShare`).

The `Test.mo` actor imports the (installed) `Map` canister, using `Maps` Candid interface to determine its Motoko type. `Test`'s run method simply puts 24 consecutive entries into Map. These entries are distributed evenly amongst the buckets making up the key-value store. Adding the first entry to a bucket take longer than adding a subsequent one, since the bucket needs to be installed on first use.

This is a Motoko example that does not currently have a Rust variant. 


## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```bash
cd examples/motoko/classes
dfx start --background
```

### Step 2: Deploy the canisters `Map` and `Test`:

```bash
dfx deploy
```

### Step 3: Invoke the run method of canister Test:

```bash
dfx canister call Test run '()'
```

The output should resemble the following:

```bash
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

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
