# Quicksort

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-quicksort-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-quicksort-example)

## Overview
This example implements the quick sort algorithm.

This is a Motoko example that does not currently have a Rust variant. 

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the command:

```
cd examples/motoko/quicksort
dfx start --background
```

### Step 2: Deploy the canister:

```
dfx deploy
```

### Step 3: Sort an array of integers.

```
dfx canister call quicksort sort '(vec { 5; 3; 0; 9; 8; 2; 1; 4; 7; 6 })'
```

The output will resemble the following:

```
(
  vec {
    0 : int;
    1 : int;
    2 : int;
    3 : int;
    4 : int;
    5 : int;
    6 : int;
    7 : int;
    8 : int;
    9 : int;
  },
)
```

## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspect is particularly relevant for this app:
* [Validate inputs](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#validate-inputs), since lists of integers are provided as input to the sorting algorithm. 
