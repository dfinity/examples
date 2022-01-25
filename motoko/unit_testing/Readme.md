## Unit Testing

This example demonstrates how to set up a simple unit testing framework for your Motoko application. Unit tests can be run on modules, while your actor will require end-to-end testing.

To make the most of your unit tests, keep logic that does not relate to state or calling other canisters so that it can be tested.

## Setup

To test, you will need to install `moc` from the latest `motoko-<system>-<version>.tar.gz` release. https://github.com/dfinity/motoko/releases.

Then, install Vessel following the guide at https://github.com/dfinity/vessel.

You will also need to install `wasmtime`. For macOS, you can install with `brew install wasmtime`. For Linux, you can install with `sudo apt-get install wasmtime`.

To run tests, use `make test`.
