# Random Maze

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/random_maze)

## Overview

This example generates a random maze using cryptographic randomness. It demonstrates how to use `Random.crypto()` from `mo:core` to obtain an `AsyncRandom` instance that automatically fetches entropy from the Internet Computer management canister on demand, and how to generate bounded discrete random numbers using `await* random.natRange(low, high)`.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/random_maze
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

## Updating the Candid interface

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.

For this example, the following aspects are particularly relevant since it employs cryptographic algorithms:

- [Don't implement crypto yourself.](https://docs.internetcomputer.org/guides/security/general-security-best-practices#dont-implement-crypto-yourself)
- [Use secure cryptographic schemes.](https://docs.internetcomputer.org/guides/security/general-security-best-practices#use-secure-cryptographic-schemes)
