# PubSub

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/pub-sub)

## Overview

This example demonstrates the publisher/subscriber pattern across two canisters on the Internet Computer. It shows how shared function references can be passed as callbacks in inter-canister calls, enabling a publisher canister to notify subscriber canisters whenever a message is published to a subscribed topic. Because ICP guarantees message delivery, the primary drawback of PubSub in distributed systems does not apply here.

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
