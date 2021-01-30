# Favorite Cities

![Compatibility](https://img.shields.io/badge/compatibility-0.6.20-blue)

This program provides functions that return multiple city names and illustrates how to use a type enclosed by square (`[ ]`) brackets to indicate an *array* of that type.

In this example, the `[Text]` notation indicates an array of a collection of UTF-8 characters, enabling the program to accept and return multiple text strings.

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

1. Say hello from your favorite cities.

   ```text
   dfx canister call favorite_cities hello_pretty '(vec {"Palo Alto"; "San Francisco"; "Zurich"})'
   ```

1. Observe the following result.

   ```text
   ("Hello from Palo Alto, San Francisco, and Zurich!")
   ```
