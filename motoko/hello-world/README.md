# Hello World

![Compatibility](https://img.shields.io/badge/compatibility-0.6.20-blue)

This example illustrates a canister called `hello_world`, which exports a
method called `main`, which prints `Hello World!` to the console.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer related processes that may conflict
   with the following.

## Demo

1. Start a local internet computer.

   ```text
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```text
   dfx canister create --all
   ```

1. Build your canister.

   ```text
   dfx build
   ```

1. Deploy your canister.

   ```text
   dfx canister install --all
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
