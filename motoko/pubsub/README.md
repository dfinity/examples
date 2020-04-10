# Pub/Sub Pattern

This app demonstrates how you can pass functions in inter-canister calls to be used as callbacks.

The first canister (Publisher) exposes a `subscribe` method that other canisters can call to register a callback to be executed whenever its other method `publish` is called with an event matching the subscribed topic.

The second canister (Subscriber) updates its internal count when its `updateCount` method is called.

There are many obvious improvements (keying subscribers by topic in Publisher, validating the topic in the callback) but I've tried to keep things simple.

To test that the behavior is correct, run `test.sh`.
