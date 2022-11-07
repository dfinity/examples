# Hello World

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)
[![Build Status](https://github.com/dfinity/examples/workflows/motoko-hello-world--example/badge.svg)](https://github.com/dfinity/examples/actions?query=workflow%3Amotoko-hello-world-example)

This example demonstrates a canister called `hello_world`, which exports a
method called `main`, which prints `Hello World!` to the console.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create hello_world
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install hello_world
   ```

1. Invoke the `main` method.

   ```text
   dfx canister call hello_world main
   ```

1. Observe the following output in the terminal where your local Internet
   Computer is running.

   ```text
   debug.print: Hello World!
   ```

# Issues

Don't see any output? Be sure to check the terminal where your local Internet
Computer is running and not the terminal from which you invoked the `main`
method.
