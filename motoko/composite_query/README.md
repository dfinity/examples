# Composite queries

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

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/motoko/composite_query)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
