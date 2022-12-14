Timer
=====

This example demonstrates an application with some periodic job triggered
every ~10 seconds using Canister Timers feature.
The timer job uses an orthogonally persistent `COUNTER` variable to count
how many times it was executed.

The example is based on the Rust Counter example, so for more information
about orthogonal persistency and basics of canisters see:

* [Rust CDK Counter Example](https://github.com/dfinity/examples/tree/master/rust/counter)

To learn more about programming the Internet Computer in Rust, see:

* [Developers Docs: Canister Timers](https://internetcomputer.org/docs/current/developer-docs/build/cdks/cdk-rs-dfinity/timer)

Introduction
------------

The application provides an interface that exposes the following methods:

* `get` &mdash; gets the value of the `COUNTER` (query)

Security Considerations and Security Best Practices
---------------------------------------------------

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

Prerequisites
-------------

Verify the following before running this demo:

* You have downloaded and installed the [DFINITY Canister
   SDK](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove).

* You have stopped any Internet Computer or other network process that would
   create a port conflict on `8000`.

Demo
----

1. Start a local internet computer:

   ```sh
   dfx start
   ```

2. Open a new terminal window.

3. Reserve an identifier for your canister:

   ```sh
   dfx canister create --all
   ```

4. Test and Build your canister:

   ```sh
   cargo test
   dfx build
   ```

5. Install your canister:

   ```sh
   dfx canister install timer
   ```

6. All in one command:

   ```sh
   # dfx canister create --all
   # dfx build
   # dfx canister install --all   
   dfx deploy
   ```

7. The value of the `COUNTER` will be increasing every ~10 seconds.

8. After 10 seconds, get the value of the `COUNTER`:

   ```sh
   dfx canister call timer get
   ```

9. Observe a non-zero result.
