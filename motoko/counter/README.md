# Counter

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-counter-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-counter-example)

## Overview

This example demonstrates a counter application. It uses an orthogonally persistent counter variable to store an arbitrary precision natural number that represents the current value of the counter.

By using the Motoko keyword stable when declaring the counter variable, the value of this variable will automatically be preserved whenever your canister code is upgraded. Without the stable keyword, a variable is deemed flexible, and its value is reinitialized on every canister upgrade, i.e. whenever new code is deployed to the canister.

The application provides an interface that exposes the following methods:

- `set`: sets the value of the counter.
- `inc`: increments the value of the counter.
- `get`: gets the value of the counter.

### Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

 ### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd examples/motoko/counter
dfx start --background
```

 ### Step 2: Deploy the canister:

```
dfx deploy
```

 ### Step 3: Set the value of the counter:

```
dfx canister call counter set '(7)'
```

 ### Step 4: Increment the value of the counter:

```
dfx canister call counter inc
```

 ### Step 5: Get the value of the counter:

```
dfx canister call counter get
```

The following output should be returned:

```
(8 : nat)
```

### Resources
To learn more about these features of Motoko, see:

- [Orthogonal persistence](https://internetcomputer.org/docs/current/motoko/main/motoko#orthogonal-persistence).
- [Declaring stable values](https://internetcomputer.org/docs/current/motoko/main/upgrades#declaring-stable-variables).


## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.
