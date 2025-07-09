---
keywords: [beginner, motoko, hello, hello world]
---

# Hello, world!

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/hello-world)

This example demonstrates a canister called `hello_world`, which exports a
method called `main`, which prints `Hello World!` to the console.

## Prerequisites

Verify the following before running this demo:

- [x] You have downloaded and installed [`dfx`](https://sdk.dfinity.org).

- [x] You have stopped any process that would create a port conflict on 8000.

- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on ICP. This example may not implement all the best practices.

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
   cd examples/motoko/hello-world
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

Don't see any output? Be sure to check the terminal where your local replica is running and not the terminal from which you invoked the `main`
method.
