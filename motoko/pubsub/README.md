# Design Pattern: Pub/Sub

[![Build Status](https://travis-ci.org/dfinity-lab/examples.svg?branch=master)](https://travis-ci.org/dfinity-lab/examples?branch=master)

This sample project demonstrates how functions may be passed as arguments of inter-canister calls to be used as callbacks.

### Overview

A common problem in both distributed and decentralized systems is keeping separate services (or canisters) synchronized with one another. While there are many potential solutions to this problem, a popular one is the Publisher/Subscriber pattern or "PubSub". PubSub is an especially valuable pattern on the Internet Computer as its primary drawback, message delivery failures, does not apply.

### Implementation

The first canister (Publisher) exposes a `subscribe` method that other canisters can call to register a callback to be executed whenever its other method `publish` is called with an event matching the subscribed topic.

The second canister (Subscriber) updates its internal count when its `updateCount` method is called.

Note: There are many obvious improvements (keying subscribers by topic in Publisher, validating the topic in the callback) and callbacks can do much more complex things than update counters but hopefully this example illustrates the concepts in a simple way.

### Prerequisites

You have downloaded and installed the SDK as described in [Getting started](https://sdk.dfinity.org/docs/developers-guide/getting-started.html).

### Demo

Start a local internet computer.

```bash
dfx start
```

Execute the following commands in another tab or run the `test.sh` script.

```bash
dfx build
dfx canister install --all
dfx canister call sub init
dfx canister call sub getCount
dfx canister call pub publish '(record { "topic" = "Apples"; "value" = 2 })'
sleep 2
dfx canister call sub getCount
dfx canister call pub publish '(record { "topic" = "Bananas"; "value" = 3 })'
sleep 2
dfx canister call sub getCount
```

Observe the following result.

```bash
()  # Subscriber is initialized
(0) # No count yet
()  # Publisher receives a topic relevant to the Subscriber
(2) # Subscriber has updated
()  # Publisher receives a topic irrelevant to the Subscriber
(2) # Subscriber has ignored the irrelevant topic
```
