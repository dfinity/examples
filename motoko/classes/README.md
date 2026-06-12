# Motoko Actor Classes

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/classes)

## Overview

This example demonstrates a simple use of actor classes, allowing a program to dynamically install new actors (canisters) at runtime. It implements a distributed key-value store that maps `Nat` to `Text` values, with entries spread across a small number of separately deployed `Bucket` actor-class canisters created on demand.

## Build and deploy from the command line

### Prerequisites

- [Node.js](https://nodejs.org)
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/classes
```

### Deploy and test

```bash
icp network start -d
icp deploy --cycles 30t
make test
icp network stop
```

The `--cycles 30t` flag funds the backend canister with 30 trillion cycles so it can forward cycles when dynamically creating `Bucket` child canisters.

> If tests fail with an out-of-cycles error, run `make topup` to add 30 trillion cycles to the backend canister and retry.

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
