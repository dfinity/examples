---
keywords: [motoko, beginner, motoko actor, actor reference]
---

# Actor reference

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/actor_reference)

## Overview

This example demonstrates a simple use of an actor reference to convert a textual representation of some principal to a value of an actor type, that, now typed as an actor, can be communicated with by calling the shared functions of its interface.

Actor references should be used sparingly, and only when necessary, to provide a typed interface to a raw principal identifier. Using an actor reference is regarded as unsafe since it is effectively an assurance, made by the programmer to the compiler, that the given principal obeys the given interface. It's the programmer's responsibility, as opposed to the compiler's, to ensure that the given principal obeys the given interface.

:::caution
Providing an incorrect interface may cause subsequent communication with the actor to fail with serialization (but not memory) errors.
:::

The example defines one Motoko actor: `main.mo` binds the name IC to the actor obtained by asserting an interface for the textual actor reference "aaaaa-aa". This is the identity, in textual format, of the well-known (that is, system-provided) management canister which is typically used to install, top up, and otherwise manage canisters on the IC.

The full interface of the management canister is provided in the Interface Computer interface specification. For this simple example, we only need a subset of the specified operations, and, due to Candid sub-typing, can even import them at less informative types than described in the full specification. To provide access to more operations, one would simply add them to the actor type, at the appropriate Motoko translation of the original Candid signature.

Our actor exposes a single burn method that uses its local IC actor reference to provision, create, query, stop, and delete a transient canister, to burn half of the actor's cycle balance.

This application of IC is meant to be illustrative, not necessarily useful.

This is a Motoko example that does not currently have a Rust variant. 


## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```bash
cd examples/motoko/actor-reference
dfx start --background
```

### Step 2: Deploy the canister:

```bash
dfx deploy
```

### Step 3: Invoke the `burn` method of canister `actor_reference`:

```bash
dfx canister call actor_reference burn '()'
```

The output will resemble the following:

```bash
[Canister by6od-j4aaa-aaaaa-qaadq-cai] balance before: 3091661916488
[Canister by6od-j4aaa-aaaaa-qaadq-cai] cycles: 1538138650552
[Canister by6od-j4aaa-aaaaa-qaadq-cai] balance after: 1545830657728
```

## Resources

- [Actors and sync data](https://internetcomputer.org/docs/current/motoko/main/actors-async).
- [Basic Motoko concepts and terms](https://internetcomputer.org/docs/current/motoko/main/basic-concepts).

# Security considerations best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on ICP. This example may not implement all the best practices.
