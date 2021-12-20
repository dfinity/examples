# Design Pattern: Pub/Sub

This sample project demonstrates like motoko pub-sub example.

## Overview

A common problem in both distributed and decentralized systems is keeping separate services (or canisters) synchronized with one another. While there are many potential solutions to this problem, a popular one is the Publisher/Subscriber pattern or "PubSub". PubSub is an especially valuable pattern on the Internet Computer as its primary drawback, message delivery failures, does not apply.

## Implementation

The first canister (Publisher) exposes a `subscribe` method that other canisters can call to register a callback to be executed whenever its other method `publish` is called with an event matching the subscribed topic.

The second canister (Subscriber) updates its internal count when its `update_count` method is called.

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
   dfx canister call subscriber setup_subscribe '(principal "your publisher canister id","Apples")'
   ```

1. Publish to the `"Apples"` topic.

   ```text
   dfx canister call publisher publish '(record { "topic" = "Apples"; "value" = 2 })'
   ```

1. Receive your subscription.

   ```text
   dfx canister call subscriber get_count
   ```

1. Observe the following result.

   ```
   (2)
   ```
