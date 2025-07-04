# EVM block explorer

The EVM Block Explorer example demonstrates how an ICP smart contract can obtain information directly from other blockchain networks. Using HTTPS outcalls, smart contracts on ICP can interact with other networks without needing to go through a third-party service such as a bridge or an oracle. Supported interactions with other chains include querying network data, signing transactions, and submitting transactions directly to other networks.
In this example, you'll also see how to sign transactions with canister ECDSA or Schnorr signatures.

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Run" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/motoko/evm_block_explorer)

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
