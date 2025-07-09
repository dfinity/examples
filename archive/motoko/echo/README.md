---
keywords: [beginner, motoko, echo]
---

# Echo

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/echo)

## Overview

This example demonstrates a simple echo effect, where an application sends back the data it receives.

This is a Motoko example that does not currently have a Rust variant. 


## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).
- [ ] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```bash
cd examples/motoko/echo
dfx start --background
```

### Step 2: Deploy the canisters `Map` and `Test`:

```bash
dfx deploy
```

### Step 3: Invoke the `say` method:

```bash
dfx canister call echo say '("This is a test.")'
```


The following output will be returned: 

```bash
("This is a test.")
```
