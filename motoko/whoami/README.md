# Who am I?

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-whoami-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-whoami-example)

## Overview

This example demonstrates how a canister can identify its caller and itself.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd examples/motoko/whoami
dfx start --background
```

### Step 2: Build and deploy the canister:

```
dfx canister install whoami --argument='(principal "2mxjj-pyyts-rk2hl-2xyka-avylz-dfama-pqui5-pwrhx-wtq2x-xl5lj-qqe")'
dfx build
dfx deploy
```

### Step 3: Invoke the `whoami` method:

```
dfx canister call whoami whoami
```

### Step 4: Observe your principal identifier.

### Step 5: Invoke the `id` method:

```
dfx canister call whoami id
```

### Step 6: Observe the principal identifier of your canister.


## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Make sure any action that only a specific user should be able to do requires authentication](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#make-sure-any-action-that-only-a-specific-user-should-be-able-to-do-requires-authentication), since this example illustrates how to access the caller system API. 
* [Disallow the anonymous principal in authenticated calls](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#disallow-the-anonymous-principal-in-authenticated-calls), since the caller system API may return the anonymous principal.