# EVM Block Explorer

The EVM Block Explorer example demonstrates how an ICP smart contract can obtain information directly from other blockchain networks. Using HTTPS outcalls, smart contracts on ICP can interact with other networks without needing to go through a third-party service such as a bridge or an oracle. Supported interactions with other chains include querying network data, signing transactions, and submitting transactions directly to other networks.
In this example, you'll also see how to sign transactions with canister ECDSA or Schnorr signatures.

This application's logic is written in [Rust](https://internetcomputer.org/docs/building-apps/developer-tools/cdks/rust/intro-to-rust).

## Project structure

The `/backend` folder contains the Rust smart contract:

- `Cargo.toml`, which defines the crate that will form the backend
- `lib.rs`, which contains the actual smart contract, and exports its interface

The `/frontend` folder contains web assets for the application's user interface. The user interface is written using the React framework.

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Deploy" in the upper right corner.
To open this project in ICP Ninja, click [here](https://icp.ninja/i?url=https://github.com/dfinity/examples/tree/master/rust/evm_block_explorer).

To **download** or **reset** the project files, click the menu option next to the deploy button.

## Build and deploy from the command-line

To migrate your ICP Ninja project off of the web browser and develop it locally, follow these steps. These steps are necessary if you want to deploy this project for long-term, production use on the mainnet.

### 1. Download your project from ICP Ninja using the 'Download files' button on the upper left corner under the pink ninja star icon.

### 2. Open the `BUILD.md` file for further instructions.
