# Counter

![Compatibility](https://img.shields.io/badge/compatibility-0.6.20-blue)

This sample project create a single actor named `+Counter+` and provides a simple example of how you can verify that the variable state—that is, the value of the variable between calls—persists.

The program uses the `+counter+` variable to contain a natural number that represents the current value of the counter.
This program supports the following types of function calls:

* The `+set+` function call updates the current value to an arbitrary numeric value you specify as an argument.
* The `+inc+` function call updates the current value, incrementing by 1 (no return value).
* The `+get+` function call queries and returns the current value.

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

1. Set the value of the counter.

   ```text
   dfx canister call counter set '(7)'
   ```

1. Increment the value of the counter.

   ```text
   dfx canister call counter inc
   ```

1. Get the value of the counter.

   ```text
   dfx canister call counter get
   ```

1. Observe the following result.

   ```
   (8)
   ```
