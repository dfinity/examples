# Counter

This is a Rust port of the Mokoto Counter example. 

This example demonstrates a counter application. It uses an orthogonally
persistent `counter` variable to store an arbitrary precision natural number
that represents the current value of the counter.

Canisters in Rust are single-threaded, thus it is safe (and simplest) to
declare global state variable within a `thread_local!` block
and wrapping `COUNTER` in a `RefCell` pointer.

This is the equivalent of the `stable` keyword in Mokoto, and the value of 
such variables will automatically be preserved whenever the canister code is
upgraded.

The `counter.did` file represents the published API interface for the counter
canister, and must be expressed using Candid syntax and Candid types. 
Exposed functions in the Rust implementation should have equivalent types
and be annotated with a `#[ic_cdk_macros::query]`or `#[ic_cdk_macros::update]` macro.

To learn more about Candid, see:
- https://internetcomputer.org/docs/current/developer-docs/build/candid/candid-concepts
- https://internetcomputer.org/docs/current/references/candid-ref/#supported-types
- https://docs.rs/candid/latest/candid/

To learn more about programming the Internet Computer in Rust, see:
- https://internetcomputer.org/docs/current/developer-docs/build/cdks/cdk-rs-dfinity/rust-counter


## Introduction

The application provides an interface that exposes the following methods:

*  `set`, which sets the value of the counter (update)

*  `inc`, which increments the value of the counter (update)

*  `get`, which gets the value of the counter (query)

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Start a local internet computer.

   ```sh
   dfx start
   ```

1. Open a new terminal window.

1. Reserve an identifier for your canister.

   ```sh
   dfx canister create counter
   ```

1. Test and Build your canister.

   ```sh
   cargo test
   dfx build
   ```

1. Install your canister.

   ```sh
   dfx canister install counter
   ```

1. All in one command
   ```sh
   # dfx canister create --all
   # dfx build
   # dfx canister install --all   
   dfx deploy
   ```

1. Increment the value of the counter.

   ```sh
   dfx canister call counter inc
   ```

1. Get the value of the counter.

   ```sh
   dfx canister call counter get
   ```

1. Observe the following result.

   ```
   (8 : nat)
   ```
