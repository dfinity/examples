# Hello cycles
![Compatibility](https://img.shields.io/badge/compatibility-0.6.25-blue)

The `hello_cycles` sample project provides a simple example to illustrate how you might add functions to receive cycles, transfer cycles, and check your cycle balance with a simple Motoko actor (canister).

This sample project assumes that you are using the default cycles wallet canister that is created for you.

This example consists of the following functions: 

* The `+wallet_balance : () -> async Nat+` function enables you to check the current cycle balance for the canister.
* The `+wallet_receive : () -> { amount : Nat64 }+` function enables the program to accept cycles that are sent to the canister from a wallet.
  Both the name and type of this function are
  dictated by the wallet's implementation (so don't mess with them).

* The `+transfer : (shared () -> (), Nat) -> async { refunded : Nat }+` function enables the program to transfer cycles to any
  shared function with candid signature "() -> ()" (assuming it accepts cycles).
  One example is the wallet's own `wallet_receive : () -> ()` function.

  (_Beware_: the wallet's `wallet_receive` return type differs from `hello_cycle`'s `wallet_receive`.)

## Prerequisites

Verify the following before running this demo:

*  You have downloaded and installed the [DFINITY Canister
   SDK](https://sdk.dfinity.org).

*  You have stopped any Internet Computer or other network process that would
   create a port conflict on 8000.

## Demo

1. Open a terminal window.

2. Start the Internet Computer locally by running the following command:

   ```text
   dfx start --clean --background
   ```

3. Deploy the project locally by running the following command:

   ```text
   dfx deploy
   ```

4. Check that the current cycles balance for the `hello_cycles` by running the following command:

   ```text
   dfx canister call hello_cycles wallet_balance
   ```

   You can also see the cycles balance the hello_cyles (or any canister you control) by calling:

   ```text
   dfx status hello_cycles
   ```

5. Run:
   ```
   dfx identity get-wallet
   dfx canister id hello_cycles
   ```
   to display the default wallet and `hello_cycles` canister identifiers.
   

   Below, we'll frequently use `$(dfx identity get_wallet)`  and `$(dfx canister id hello_cycles)` to splice canister identifiers into longer bash commands.

6. Attempt to send 2 trillion cycles from the default wallet to the `hello_cycles` canister by running the following command:

   ```text
   dfx canister call $(dfx identity get-wallet) wallet_send "(record { canister = principal \"$(dfx canister id hello_cycles)\"; amount = (2000000000000:nat64); } )"
   ```

   The wallet's `wallet_send` function transfers the amount to the argument canister's `wallet_receive` function (see above), and echoes the amount accepted.

7. Verify that the cycles balance for the `hello_cycles` canister has been update by `10_000_000` running the following command:

   ```text
   dfx canister call hello_cycles wallet_balance
   ```

8. Send cycles from the `hello_cycles` canister back to the wallet
   by running the command:

   ```text
   dfx canister call hello_cycles transfer "(func \"$(dfx identity get-wallet)\".\"wallet_receive\", 5000000)"
   ```text

9. Verify that the cycles balance of `hello_cycles` canister has been decreased
   with:

   ```text
   dfx canister call hello_cycles wallet_balance
   ```

