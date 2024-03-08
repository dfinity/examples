---
keywords: [beginner, motoko, factorial, factor]
---

# Factorial

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/factorial)

## Overview

This example demonstrates a recursive mathematical function that calculates the product of all positive integers less than or equal to its input.

This is a Motoko example that does not currently have a Rust variant. 

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```bash
cd examples/motoko/factorial
dfx start --background
```

### Step 2: Deploy the canister:

```bash
dfx deploy
```

### Step 3: Calculate the factorial of 20:

```bash
dfx canister call factorial fac '(20)'
```

The following output will be returned: 

```bash
(2_432_902_008_176_640_000 : nat)
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
