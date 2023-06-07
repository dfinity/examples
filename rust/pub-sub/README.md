# Design Pattern: Pub/Sub

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-pub-sub-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-pub-sub-example)

## Overview
This sample project demonstrates how functions may be passed as arguments of inter-canister calls to be used as callbacks.

A common problem in both distributed and decentralized systems is keeping separate services (or canisters) synchronized with one another. While there are many potential solutions to this problem, a popular one is the Publisher/Subscriber pattern or "PubSub". PubSub is an especially valuable pattern on the Internet Computer as its primary drawback, message delivery failures, does not apply.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd examples/rust/pub-sub
dfx start --background
```

### Step 2: Deploy the canister:

```
dfx deploy
```

### Step 3: Subscribe to the "Apples" topic:

```
dfx canister call sub init '("Apples")'
```

### Step 4: Publish to the "Apples" topic:

```
dfx canister call pub publish '(record { "topic" = "Apples"; "value" = 2 })'
```

### Step 5: Receive your subscription:

```
dfx canister call sub getCount
```

The output should resemble the following:

```
(2 : nat)
```

## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it makes inter-canister calls: 
* [Be aware that state may change during inter-canister calls.](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#be-aware-that-state-may-change-during-inter-canister-calls)
* [Only make inter-canister calls to trustworthy canisters.](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#only-make-inter-canister-calls-to-trustworthy-canisters)
* [Don’t panic after await and don’t lock shared resources across await boundaries.](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#dont-panic-after-await-and-dont-lock-shared-resources-across-await-boundaries)
