# PubSub

This example demonstrates the publisher/subscriber (pub/sub) messaging pattern implemented across two canisters on the Internet Computer. The key ICP concept it shows: **shared function references can be passed as callbacks between canisters**, enabling a publisher to call back into a subscriber without knowing its type in advance.

## How it works

Two canisters are deployed:

- **`publisher`** — holds a list of `(topic, callback)` registrations. When `publish(counter)` is called, it invokes every callback whose topic matches.
- **`subscriber`** — registers itself with the publisher for a specific topic. When notified, it accumulates the published value into a local counter.

The call flow:

```
1. subscriber.init(publisher_principal, "Apples")
     └─► publisher.subscribe({ topic = "Apples"; callback = subscriber.updateCount })

2. publisher.publish({ topic = "Apples"; value = 2 })
     └─► subscriber.updateCount({ topic = "Apples"; value = 2 })   ← async callback

3. subscriber.getCount()  →  2
```

The callback (`subscriber.updateCount`) is a **shared function reference** — a first-class value in Motoko that can be stored and called across canisters. The publisher principal is passed to `init` at runtime rather than hard-coded at compile time, making the subscriber reusable with any publisher. Because ICP guarantees message delivery, the typical reliability concern of pub/sub in distributed systems does not apply here.

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
