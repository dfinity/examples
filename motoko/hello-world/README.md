![Compatibility](https://img.shields.io/badge/compatibility-0.6.10-blue)
[![Build Status](https://travis-ci.org/dfinity-lab/examples.svg?branch=master)](https://travis-ci.org/dfinity-lab/examples?branch=master)

## Hello World


### Prerequisites

Before building the example application, verify the following:

* You have downloaded and installed the DFINITY Canister SDK as described in [Download and install](https://sdk.dfinity.org/docs/quickstart/quickstart.html#download-and-install).
* You have stopped any Internet Computer network processes running on the local computer.

### Demo

1. Open a new terminal in the project directory.

1. Start the Internet Computer netowrk locally by running the following command:

    ```bash
    dfx start
    ```

1. Open another terminal in the project directory.

1. Execute the following commands in the new terminal:

    ```bash
    dfx canister create --all
    dfx build
    dfx canister install --all
    dfx canister call hello-world main
    ```

1. Observe the result in the terminal where the Internet Computer network is running.

    ```
    debug.print: Hello World!
    ```
