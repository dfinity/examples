# Hello, cycles!

The `hello_cycles` sample project provides a simple example to illustrate how you might add functions to receive cycles, transfer cycles, and check your cycle balance with a simple Motoko actor (canister).

This sample project assumes that you are using the default cycles wallet canister that is created for you.

This example consists of the following functions (see `src/hello_cycles/main.mo`):

- The `wallet_balance : () -> async Nat`: enables you to check the current cycle balance for the canister.

- The `wallet_receive : () -> { amount : Nat64 }`: enables the program to accept cycles that are sent to the canister from a wallet. Both the name and type of this function are dictated by the wallet's implementation (so don't mess with them).

- The `transfer : (shared () -> (), Nat) -> async { refunded : Nat }`: enables the program to transfer cycles to any shared function with candid signature `"() -> ()"` (assuming it accepts cycles). One example is the wallet's own `wallet_receive : () -> ()` function.

:::caution
The wallet's `wallet_receive` return type differs from hello_cycle's `wallet_receive`.
:::

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/motoko/hello_cycles)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.

