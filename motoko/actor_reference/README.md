# Actor references

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-actor_reference-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-actor_reference-example)

# Actor reference

## Overview

This example demonstrates a simple use of an actor reference to convert a textual representation of some principal to a value of an actor type, that, now typed as an actor, can be communicated with by calling the shared functions of its interface.

Actor references should be used sparingly, and only when necessary, to provide a typed interface to a raw principal identifier. Using an actor reference is regarded as unsafe since it is effectively an assurance, made by the programmer to the compiler, that the given principal obeys the given interface. It's the programmer's responsibility, as opposed to the compiler's, to ensure that the given principal obeys the given interface.

:::caution
Providing an incorrect interface may cause subsequent communication with the actor to fail with serialization (but not memory) errors.
:::

The example defines one Motoko actor: `main.mo` binds the name IC to the actor obtained by asserting an interface for the textual actor reference "aaaaa-aa". This is the identity, in textual format, of the well-known (that is, system provided) management canister which is typically used to install, top up, and otherwise manage canisters on the IC.

The full interface of the management canister is provided in the Interface Computer interface specification. For this simple example, we only need a subset of the specified operations, and, due to Candid sub-typing, can even import them at less informative types than described in the full specification. To provide access to more operations, one would simply add them to the actor type, at the appropriate Motoko translation of the original Candid signature.

Our actor exposes a single burn method that uses its local IC actor reference to provision, create, query, stop and delete a transient canister, in order to burn half of the actor's cycle balance.

This application of IC is meant to be illustrative, not necessarily useful.

This is a Motoko example that does not currently have a Rust variant. 


## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](../developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd examples/motoko/actor-reference
dfx start --background
```

### Step 2: Deploy the canister:

```
dfx deploy
```

### Step 3: Invoke the `burn` method of canister `actor_reference`:

```
dfx canister call actor_reference burn '()'
```

The output will resemble the following:

```
[Canister by6od-j4aaa-aaaaa-qaadq-cai] balance before: 3091661916488
[Canister by6od-j4aaa-aaaaa-qaadq-cai] cycles: 1538138650552
[Canister by6od-j4aaa-aaaaa-qaadq-cai] balance after: 1545830657728
```

## Resources

- [Actors and sync data](../motoko/main/actors-async.md).
- [Basic Motoko concepts and terms](../motoko/main/basic-concepts.md).

# Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
