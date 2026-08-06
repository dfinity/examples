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
     (publisher reached via `import Publisher "canister:publisher"`, resolved from
      PUBLIC_CANISTER_ID:publisher, injected by icp-cli)

2. publisher.publish({ topic = "Apples"; value = 2 })
     └─► subscriber.updateCount({ topic = "Apples"; value = 2 })   ← oneway callback (fire-and-forget)

3. subscriber.getCount()  →  2
```

The callback (`subscriber.updateCount`) is a **shared function reference** — a first-class value in Motoko that can be stored and called across canisters. The subscriber reaches the publisher through the typed `import Publisher "canister:publisher"` import — no actor type is declared in the code, and no principal is hardcoded or passed as an argument (see [How the publisher is resolved](#how-the-publisher-is-resolved)). ICP guarantees that messages are delivered to the target canister, but callbacks can still fail if the target traps or runs out of cycles — error handling should be considered in production use.

## How the publisher is resolved

The subscriber calls the publisher through `import Publisher "canister:publisher"`. The `--actor-env-alias publisher PUBLIC_CANISTER_ID:publisher publisher/publisher.did` flag in `mops.toml` tells the Motoko compiler to type that import against `publisher/publisher.did` at build time, and to resolve the publisher's **principal at runtime** from the canister environment variable `PUBLIC_CANISTER_ID:publisher`. Because the interface comes from the `.did`, no actor type is written in `subscriber/main.mo`.

The consequence is that the subscriber's Wasm contains **no publisher principal** — the compiler embeds only the environment-variable *name*, so **the same Wasm artifact runs unchanged in every environment**:

- **Build time (`mops build` / `icp build`):** `moc` compiles the `canister:publisher` import into calls to the IC environment-variable system API, typed against the `.did`. No principal and no target environment are known to the build.
- **Deploy time (`icp deploy`):** icp-cli sets the environment variable `PUBLIC_CANISTER_ID:publisher` on the subscriber canister to the publisher's principal. It is auto-injected for canisters defined in the project, and can be overridden per environment in `icp.yaml` (`environments[].settings.subscriber.environment_variables`) — for example to point the subscriber at a publisher that already exists on mainnet.
- **Runtime:** the canister reads `PUBLIC_CANISTER_ID:publisher` and binds the `Publisher` reference when it is installed or upgraded, then reuses that principal for every call. Changing the variable therefore takes effect on the next deploy/upgrade.

Note: `publish` fires callbacks asynchronously. There is a brief delay before the subscriber state is updated, which is why the tests sleep briefly after publishing.

> `bash test.sh` assumes a freshly deployed state. To re-run locally without restarting the network, reinstall the canisters first: `icp deploy --mode reinstall -y && bash test.sh`.

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
bash test.sh
icp network stop
```

## Updating the Candid interface

`publisher/publisher.did` is the interface the subscriber's `canister:publisher` import is typed against (see [How the publisher is resolved](#how-the-publisher-is-resolved)). If you change the publisher's public API, regenerate it so the subscriber keeps compiling against the correct interface:

```bash
mops generate candid publisher
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
