---
keywords: [beginner, motoko, composite queries, queries]
---

# Composite queries

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/composite_query)

## Overview

This example modifies the simple actor class example to demonstrate the implementation of composite queries.

The original example demonstrates a simple use of actor classes, allowing a program to dynamically install new actors (that is, canisters). It also demonstrates a multi-canister project, and actors using inter-actor communication through `shared` functions.

In the original example, shared functions `Map.get` and `Bucket.get` were both implemented as
update methods so that `Map.get` could call `Bucket.get`.

In this version `Bucket.get` is implemented as a query function and `Map.get` as a composite query function.
Although queries and composite queries are fast, composite queries can only be invoked as ingress messages, either
using `dfx` (see below) or an agent through, for example, a browser front-end (not illustrated here).

In detail, the example provides actor `Map`.
`Map` is a dead-simple, distributed key-value store, mapping `Nat` to `Text` values, with entries stored in a small number of separate `Bucket` actors, installed on demand.

[Map.mo](./src/map/Map.mo) imports a Motoko _actor class_ `Bucket(i, n)`
from library [Buckets.mo](./src/map/Buckets.mo).
It also imports the `ExperimentalCycles` base library, naming it `Cycles` for short, to share its cycles amongst the buckets it creates.

Each call to `Buckets.Bucket(n, i)` within `Map` instantiates a new `Bucket` instance (the `i`-th of `n`) dedicated to those entries of the `Map` whose key _hashes_ to `i` (by taking the remainder of the key modulo division by `n`).

Each asynchronous instantiation of the `Bucket` actor class corresponds to the dynamic, programmatic installation of a new `Bucket` canister.

Each new `Bucket` must be provisioned with enough cycles to pay for its installation and running costs.
`Map` achieves this by adding an equal share of `Map`'s initial cycle balance to each asynchronous call to `Bucket(n, i)`, using a call to `Cycles.add(cycleShare)`.

`Map`'s `test` method simply `put`s 16 consecutive entries into `Map`. These entries are distributed evenly amongst the buckets making up the key-value store. Adding the first entry to a bucket takes longer than adding a subsequent one, since the bucket needs to be installed on first use.

## Prerequisites

Verify the following before running this demo:

- [x] You have downloaded and installed [dfx](https://sdk.dfinity.org).

- [x] You have stopped any process that would create a port conflict on `8000`.

- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Install

### Step 1: Start a local internet computer.

```bash
dfx start
```

### Step 2: Open a new terminal window.

### Step 3: Deploy the `Map` canister:

```bash
cd examples/motoko/composite_query
dfx deploy
```

### Step 4: Invoke the `test` method of canister `Map` to add some entries:

```bash
dfx canister call Map test '()'
```

### Step 5: Observe the following result.

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
()
```

### Step 6: Invoke the `get` composite query method of canister `Main`:

```bash
dfx canister call --query Map get '(15)'
```

### Step 7: Observe the following result:

```bash
(opt "15")
```

## Resources

- [Actor classes](https://internetcomputer.org/docs/current/motoko/main/actor-classes).
- [Managing cycles](https://internetcomputer.org/docs/current/motoko/main/cycles).
- [Composite queries](https://internetcomputer.org/docs/current/motoko/main/actors-async#composite-query-functions).

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on ICP. This example may not implement all the best practices.
