---
keywords: [beginner, motoko, hello, hello cycles, cycles]
---

# Hello, cycles!

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/hello_cycles)

## Overview

The `hello_cycle`s sample project provides a simple example to illustrate how you might add functions to receive cycles, transfer cycles, and check your cycle balance with a simple Motoko actor (canister).

This sample project assumes that you are using the default cycles wallet canister that is created for you.

This example consists of the following functions (see `src/hello_cycles/main.mo`):

- The `wallet_balance : () -> async Nat`: enables you to check the current cycle balance for the canister.

- The `wallet_receive : () -> { amount : Nat64 }`: enables the program to accept cycles that are sent to the canister from a wallet. Both the name and type of this function are dictated by the wallet's implementation (so don't mess with them).

- The `transfer : (shared () -> (), Nat) -> async { refunded : Nat }`: enables the program to transfer cycles to any shared function with candid signature `"() -> ()"` (assuming it accepts cycles). One example is the wallet's own `wallet_receive : () -> ()` function.

:::caution
The wallet's `wallet_receive` return type differs from hello_cycle's `wallet_receive`.
:::

This is a Motoko example that does not currently have a Rust variant. 

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Download the following project files from GitHub: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files and start a local instance of the replica with the command:

```bash
cd examples/motoko/hello_cycles
dfx start --background
```

### Step 2: Deploy the canister:

```bash
dfx deploy
```

### Step 3: Check that the current cycles balance of canister hello_cycles by running the following command:

```bash
dfx canister call hello_cycles wallet_balance
```

The output should resemble the following:

```bash
(3_091_662_816_985 : nat)
```

You can also see the cycles balance of hello_cycles (or any canister you control) by calling:

```bash
dfx canister status hello_cycles
```

The output of this command will be similar to:

```bash
Canister status call result for hello_cycles.
Status: Running
Controllers: 2vxsx-fae b77ix-eeaaa-aaaaa-qaada-cai
Memory allocation: 0
Compute allocation: 0
Freezing threshold: 2_592_000
Memory Size: Nat(2371252)
Balance: 3_092_278_099_590 Cycles
Module hash: 0x09198be65e161bdb5c75c705dfec4b707a8091ac5d1a095dd45c025142a1fc43
```

### Step 4: To display the default wallet and hello_cycles canister principals, run the commands:

```bash
dfx identity get-wallet
dfx canister id hello_cycles
```

The output should resemble the following:

```bash
b77ix-eeaaa-aaaaa-qaada-cai
dzh22-nuaaa-aaaaa-qaaoa-cai
```

Below, we'll frequently use `$(dfx identity get-wallet)` and `$(dfx canister id hello_cycles)` to splice canister principals into longer bash commands.

### Step 5: Attempt to send 2 trillion cycles from the default wallet to the hello_cycles canister by running the following command:

```bash
dfx canister call $(dfx identity get-wallet) wallet_send "(record { canister = principal \"$(dfx canister id hello_cycles)\"; amount = (2000000000000:nat64); } )"
```

The wallet's `wallet_send` function transfers the amount to the argument canister's `wallet_receive` function (see above) and returns a result signaling success or failure.

If successful, the output will look similar to:

```bash
(variant { 17_724 })
```

#### Step 6: Verify that the cycles balance for the hello_cycles canister has increased by 10_000_000 by running the following command:

```bash
dfx canister call hello_cycles wallet_balance
```

Output:

```bash
(5_091_662_569_379 : nat)
```

The amount is only increased by 10_000_000 because the implementation of `wallet_receive` is coded to accept at most 10_000_000 cycles, even when more cycles were transferred with the call. The unaccepted cycles are not lost but implicitly refunded to the caller (in this case, the wallet).

### Step 7: Send some cycles from the hello_cycles canister back to the wallet by running the command:

```bash
dfx canister call hello_cycles transfer "(func \"$(dfx identity get-wallet)\".\"wallet_receive\", 5000000)"
```

Output: 

```bash
(record { refunded = 0 : nat })
```

### Step 8: Verify that the cycles balance of the `hello_cycles` canister has decreased with:

```bash
dfx canister call hello_cycles wallet_balance
```

Output:

```bash
(5_091_657_208_987 : nat)
```

In this step, we are passing our own wallet's `wallet_receive` function as the first argument, followed by the amount. We, or a third party, could also pass any other function of the same signature, belonging to any other canister or wallet.

Without some additional access control checks (omitted here), a malicious client could abuse our naive transfer function to drain the canister of all of its cycles.


## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspect is particularly relevant for this app:
* [Protect against draining the cycles balance](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#protect-against-draining-the-cycles-balance), since this canister consumes cycles that are being transferred to other canisters. 

