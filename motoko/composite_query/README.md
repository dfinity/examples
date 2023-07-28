# Composite Queries

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-classes-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-classes-example)

This example modifies the simple actor class example to demonstrate the implementation of composite queries.

The original example demonstrates a simple use of actor classes, which allow a program to dynamically install new actors (that is, canisters). It also demonstrates a multi-canister project, and actors using inter-actor communication through `shared` functions.

In the original example, shared functions `Map.get` and `Bucket.get` were both implemented as
update methods so that `Map.get` could call `Bucket.get`.

In this version `Bucket.get` is implemented as a query function and `Map.get` as a composite query function.
Although queries and composite queries are fast, composite queries can only be invoked as ingress messages, either
using `dfx` (see below) or an agent through, for example, a browser front-end (not illustrated here).

In detail, the example provides actor `Map`.
`Map` is a dead-simple, distributed key-value store, mapping `Nat` to `Text` values, with entries stored in a small number of separate `Bucket` actors, installed on demand.

[Map.mo](./src/map/Map.mo) imports a Motoko _actor class_ `Bucket(i, n)`
from library [Buckets.mo](./src/map/Buckets.mo).
It also imports the `ExperimentalCycles` base library, naming it `Cycles` for short, in order to share its cycles amongst the bucket it creates.

Each call to `Buckets.Bucket(n, i)` within `Map` instantiates a new `Bucket` instance (the `i`-th of `n`) dedicated to those entries of the `Map` whose key _hashes_ to `i` (by taking the remainder of the key modulo division by `n`).

Each asynchronous instantiation of the `Bucket` actor class corresponds to the dynamic, programmatic installation of a new `Bucket` canister.

Each new `Bucket` must be provisioned with enough cycles to pay for its installation and running costs.
`Map` achieves this by adding an equal share of `Map`'s initial cycle balance to each asynchronous call to `Bucket(n, i)`, using a call to `Cycles.add(cycleShare)`.

`Map`'s `test` method simply `put`s 16 consecutive entries into `Map`. These entries are distributed evenly amongst the buckets making up the key-value store. Adding the first entry to a bucket takes longer than adding a subsequent one, since the bucket needs to be installed on first use.


## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the
   [DFINITY Canister SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

   (Alternatively, the example will run faster if you use the emulator, not a full replica:
   ```
     dfx start --emulator
   ```
   )

2. Open a new terminal window.

3. Deploy the `Map` canister:

   ```text
   dfx deploy
   ```

4. Invoke the `test` method of canister `Map` to add some entries

   ```text
   dfx canister call Map test '()'
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
   ()
   ```

6. Invoke the `get` composite query method of canister `Main`

   ```text
   dfx canister call --query Map get '(15)'
   ```

7. Observe the following result.

   ```
   (opt "15")
   ```



# Links

Specific links:

- [Actor classes](https://internetcomputer.org/docs/current/motoko/main/actor-classes)
- [Managing Cycles](https://internetcomputer.org/docs/current/motoko/main/cycles)
- [Composite Queries](https://internetcomputer.org/docs/current/motoko/main/actors-async#composite-query-functions)

