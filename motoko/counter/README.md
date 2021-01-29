# Counter

![Compatibility](https://img.shields.io/badge/compatibility-0.6.10-blue)

This sample project create a single actor named `+Counter+` and provides a simple example of how you can verify that the variable state—that is, the value of the variable between calls—persists.

The program uses the `+counter+` variable to contain a natural number that represents the current value of the counter.
This program supports the following types of function calls:

* The `+set+` function call updates the current value to an arbitrary numeric value you specify as an argument.
* The `+inc+` function call updates the current value, incrementing by 1 (no return value).
* The `+get+` function call queries and returns the current value.

## Prerequisites

Before building the example application, verify the following:

* You have downloaded and installed the DFINITY Canister SDK as described in [Download and install](https://sdk.dfinity.org/docs/quickstart/quickstart.html#download-and-install).
* You have stopped any Internet Computer network processes running on the local computer.

## Demo

1. Start a local internet computer.

   ```bash
   dfx start
   ```

1. Execute the following commands in another tab.

   ```bash
   dfx canister create counter
   dfx build
   dfx canister install counter
   dfx canister call counter set '(7)'
   dfx canister call counter inc
   dfx canister call counter get
   ```

1. Observe the following result.

   ```
   (8)
   ```
