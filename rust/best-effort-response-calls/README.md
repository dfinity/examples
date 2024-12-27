# Best-effort response calls

## Overview 

These are basic examples that demonstrate how to issue and handle calls with best-effort responses.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx), version 0.24.3 or higher
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## First example: deadlines in best-effort response calls

In this example you can instruct the canister to issue a downstream call, which you can choose to be a best-effort response call, or a guaranteed response call. The canister will then return the deadline as observed by the receiver.

```bash
$ dfx start
$ dfx deploy
# "true" below indicates that the call should be a best-effort response call
$ dfx canister call berc_backend demonstrate_deadlines true
(1_634_120_000_000 : nat64)
# "false" below indicates that the call should be a guaranteed response call
$ dfx canister call berc_backend demonstrate_deadlines false
(0 : nat64)
```

As an alternative to using `dfx canister call`, you can also use the Candid UI to interact with the canister.

## Second example: timeouts in best-effort response calls

This example shows that the system will generate a timeout response if the downstream call takes too long to respond. In this example, the `demonstrate_timeouts` function will issue a downstream call with a 1 second timeout to a `busy` method that takes 5 rounds to respond. The canister will then return true if the downstream call timed out, and false otherwise.

```bash

If `dfx` is not yet running, start it and deploy the canister:
```bash
$ dfx start
$ dfx deploy
```

Then, run:

```
$ dfx canister call berc_backend demonstrate_timeouts
(true)
```

The above command returning true indicates that the downstream call timed out.
