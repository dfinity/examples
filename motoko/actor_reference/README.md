# Actor references

![Compatibility](https://img.shields.io/badge/compatibility-0.7.0-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-actor_reference-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-actor_reference-example)

This example demonstrates a simple use of an [actor reference](https://sdk.dfinity.org/docs/language-guide/language-manual.html#exp-actor) to convert
a textual representation of some principal to a value of an actor type, that, now typed
as an actor, can be communicated with by calling the `shared` functions of its interface.

Actor references should be used sparingly, and only when necessary, to provide a typed interface to a raw principal identifier.
Using an actor reference is regarded as unsafe since it is effectively an assurance, made by the programmer to the compiler, that the given principal obeys the given interface.
It's the programmer's responsibility, as opposed to the compiler's, to ensure that the given principal obeys the given interface.

Warning: Providing an incorrect interface may cause subsequent communication with the actor to fail with serialization (but not memory) errors.

The example defines one Motoko actor: [main.mo](./src/actor_reference/main.mo) binds the name
`IC` to the actor obtained by asserting an interface for the
textual actor reference "aaaaa-aa". This is the identity, in textual format, of the
well-known (that is, system provided) _managment canister_ which
is typically used to install, top up, and otherwise manage canisters on the IC.

The full interface of the management canister is provided in the [Interface Computer Interface Specification](https://sdk.dfinity.org/docs/interface-spec/index.html#ic-management-canister).
For this toy example, we only need a subset of the specified operations, and, due to Candid subtyping, can even import them at less informative types than described in the full specification.
To provide access to more operations, one would simply add them to the actor type, at the appropriate Motoko translation of the original Candid signature.

Our actor exposes a single `burn` method that uses its local `IC` actor reference
to provision, create, query, stop and delete a transient canister, in order
to burn half of the actor's cycle balance.

This application of `IC` is meant to be illustrative, not necessarily useful.

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

3. Deploy the canister `actor_reference`

   ```text
   dfx deploy
   ```

4. Invoke the `burn` method of canister `actor_reference`

   ```text
   dfx canister call actor_reference burn '()'
   ```

5. Observe a result similar to;

   ```text
   [Canister rrkah-fqaaa-aaaaa-aaaaq-cai] balance before: 85937500000
   [Canister rrkah-fqaaa-aaaaa-aaaaq-cai] cycles: 42968750000
   [Canister rrkah-fqaaa-aaaaa-aaaaq-cai] balance after: 42968750000
   ()
   ```

# Links

Specific links:

- [Actor references](https://sdk.dfinity.org/docs/language-guide/language-manual.html#exp-actor)
- [Managing Cycles](https://sdk.dfinity.org/docs/language-guide/cycles.html)

General background:

- [Manage Canisters](https://sdk.dfinity.org/docs/developers-guide/working-with-canisters.html)
- [Quick Start](https://sdk.dfinity.org/developers-guide/quickstart.html)
- [Developer's Guide](https://sdk.dfinity.org/developers-guide)
- [Language Guide](https://sdk.dfinity.org/language-guide)
- [Interface Computer Interface Specification, The IC management canister](https://sdk.dfinity.org/docs/interface-spec/index.html#ic-management-canister)
