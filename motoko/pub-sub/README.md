# PubSub

This example demonstrates the publisher/subscriber (pub/sub) messaging pattern implemented across two canisters on the Internet Computer. The key ICP concept it shows: **shared function references can be passed as callbacks between canisters**, enabling a publisher to call back into a subscriber without knowing its type in advance.

## How it works

Two canisters are deployed:

- **`publisher`** — holds a list of `(topic, callback)` registrations. When `publish(counter)` is called, it invokes every callback whose topic matches.
- **`subscriber`** — registers itself with the publisher for a specific topic. When notified, it accumulates the published value into a local counter.

The call flow:

```
1. subscriber.subscribe("Apples")
     └─► publisher.subscribe({ topic = "Apples"; callback = subscriber.updateCount })
     (publisher ID read from PUBLIC_CANISTER_ID:publisher, injected by icp-cli)

2. publisher.publish({ topic = "Apples"; value = 2 })
     └─► subscriber.updateCount({ topic = "Apples"; value = 2 })   ← async callback

3. subscriber.getCount()  →  2
```

The callback (`subscriber.updateCount`) is a **shared function reference** — a first-class value in Motoko that can be stored and called across canisters. The subscriber discovers the publisher automatically: icp-cli injects `PUBLIC_CANISTER_ID:publisher` into every canister in the project during `icp deploy`, and the subscriber reads it at runtime via `Runtime.envVar`. No principal is hardcoded or passed as an argument. ICP guarantees that messages are delivered to the target canister, but callbacks can still fail if the target traps or runs out of cycles — error handling should be considered in production use.

Note: `publish` fires callbacks asynchronously. There is a brief delay before the subscriber state is updated, which is why the tests sleep briefly after publishing.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/pub-sub
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
