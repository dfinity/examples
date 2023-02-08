# Quicksort

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-quicksort-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-quicksort-example)

This example implements the quicksort algorithm.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspect is particularly relevant for this app:
* [Validate inputs](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#validate-inputs), since lists of integers are provided as input to the sorting algorithm. 

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create quicksort
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install quicksort
   ```

1. Sort an array of integers.

   ```text
   dfx canister call quicksort sort '(vec { 5; 3; 0; 9; 8; 2; 1; 4; 7; 6 })'
   ```

1. Observe the following result.

   ```text
   (vec { 0; 1; 2; 3; 4; 5; 6; 7; 8; 9 })
   ```
