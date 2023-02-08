# Calc

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-calc-example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-calc-example)

This example demonstrates a four-function calculator application. It uses an
orthogonally persistent `cell` variable to store an arbitrary precision integer
that represents the result of the most recent calculation.

## Introduction

The application provides an interface that exposes the following methods:

*  `add`, which accepts input and performs addition;

*  `sub`, which accepts input and performs subtraction;

*  `mul`, which accepts input and performs multiplication;

*  `div`, which accepts input, performs division, and returns an optional type
   to guard against division by zero; and

*  `clearall`, which clears the `cell` variable by setting its value to zero.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create calc
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install calc
   ```

1. Multiply 2 by 3.

   ```text
   dfx canister call calc add '(2)'
   dfx canister call calc mul '(3)'
   ```

1. Observe the following result.

   ```text
   (6)
   ```

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Consider using stable memory, version it, test it](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#consider-using-stable-memory-version-it-test-it), since this canister uses canister memory, and not stable memory. 
* [Validate inputs](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#validate-inputs), since this canister accepts user input which requires input validation (e.g. div by 0 is not allowed). 