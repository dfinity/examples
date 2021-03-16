# Hello cycles

![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)

The `hello_cycles` sample project provides a simple example to illustrate how you might add functions to receive cycles and check your cycle balance to the default template program.

This sample project assumes that you are using the default cycles wallet canister that is created for you.

This example consists of the following functions: 

* The `+wallet_balance+` function enables you to check the current cycle balance for the canister.
* The `+wallet_receive+` function enables the program to accept cycles that are sent to the canister.
* The `+greet+` function accepts a text argument and displays a greeting in a terminal.

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Open a terminal window.

1. Start the Internet Computer locally by running the following command:

   ```text
   dfx start --clean --background
   ```

1. Deploy the project locally by running the following command:

   ```text
   dfx deploy --argument '(2000000000000)'
   ```

   The `--argument` option is used because this is an actor class.

1. Verify the default `greet` function works as expected by running the following command:

   ```text
   dfx canister call hello_cycles greet '("everyone")'
   ```

1. Check that the current cycles balance for the `hello_cycles` canister is zero by running the following command:

   ```text
   dfx canister call hello_cycles wallet_balance
   ```

1. Run `more .dfx/local/wallets.json` to find the wallet canister identifier.

1. Send cycles from the default wallet to the `hello_cycles` canister by running a command similar to the following:

   ```text
   dfx canister call rwlgt-iiaaa-aaaaa-aaaaa-cai wallet_send '(record { canister = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"; amount = (2000000000000:nat64); } )'
   ```

1. Verify that the cycles balance for the `hello_cycles` canister has been update by running the following command:

   ```text
   dfx canister call hello_cycles wallet_balance
   ```
