# Design Pattern: Pub/Sub

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-pub-sub-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-pub-sub-example)

This sample project demonstrates how functions may be passed as arguments of inter-canister calls to be used as callbacks.

## Overview

A common problem in both distributed and decentralized systems is keeping separate services (or canisters) synchronized with one another. While there are many potential solutions to this problem, a popular one is the Publisher/Subscriber pattern or "PubSub". PubSub is an especially valuable pattern on the Internet Computer as its primary drawback, message delivery failures, does not apply.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it makes inter-canister calls: 
* [Be aware that state may change during inter-canister calls](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#be-aware-that-state-may-change-during-inter-canister-calls)
* [Only make inter-canister calls to trustworthy canisters](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#only-make-inter-canister-calls-to-trustworthy-canisters)
* [Don’t panic after await and don’t lock shared resources across await boundaries](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#dont-panic-after-await-and-dont-lock-shared-resources-across-await-boundaries)

## Implementation

The first canister (Publisher) exposes a `subscribe` method that other canisters can call to register a callback to be executed whenever its other method `publish` is called with an event matching the subscribed topic.

The second canister (Subscriber) updates its internal count when its `updateCount` method is called.

Note: There are many obvious improvements (keying subscribers by topic in Publisher, validating the topic in the callback) and callbacks can do much more complex things than update counters but hopefully this example illustrates the concepts in a simple way.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create --all
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install --all
   ```

1. Subscribe to the `"Apples"` topic.

   ```text
   dfx canister call sub init '("Apples")'
   ```

1. Publish to the `"Apples"` topic.

   ```text
   dfx canister call pub publish '(record { "topic" = "Apples"; "value" = 2 })'
   ```

1. Receive your subscription.

   ```text
   dfx canister call sub getCount
   ```

1. Observe the following result.

   ```
   (2 : nat64)
   ```
