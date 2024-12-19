# PubSub

This sample project demonstrates how functions may be passed as arguments of inter-canister calls to be used as callbacks.

A common problem in both distributed and decentralized systems is keeping separate services (or canisters) synchronized with one another. While there are many potential solutions to this problem, a popular one is the Publisher/Subscriber pattern or "PubSub". PubSub is an especially valuable pattern on the Internet Computer as its primary drawback, message delivery failures, does not apply.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/getting-started/install).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

## Step 1: Setup project environment

Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```sh
cd examples/rust/pub-sub
dfx start --background
```

## Step 2: Get the publisher ID

```bash
dfx canister id publisher
```

## Step 3: Subscribe to the "Apples" topic

```bash
dfx canister call subscriber setup_subscribe '(principal "<publisher ID from Step 3>", "Apples")'
```

## Step 4: Publish to the "Apples" topic

```bash
dfx canister call publisher publish '(record { "topic" = "Apples"; "value" = 2 })'
```

## Step 5: Receive your subscription

```bash
dfx canister call subscriber get_count
```

The output should resemble the following:

```bash
(2 : nat64)
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it makes inter-canister calls: 
* [Be aware that state may change during inter-canister calls.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
* [Only make inter-canister calls to trustworthy canisters.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
* [Don’t panic after await and don’t lock shared resources across await boundaries.](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview)
