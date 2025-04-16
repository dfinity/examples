# Tokenmania!

Tokenmania is a simplified token minting application. When the application is ran, it will automatically mint tokens based on the backend smart contract's hardcoded configuration values for things such as token name, token symbol, and total supply.

> [!CAUTION]
> This sample application is not production-ready code. Actual tokens deployed on ICP will require a ledger and an index smart contract. For this example's demonstration, this functionality has been simplified and the ledger functionality has been included in the backend. Tokens deployed using this example are only available for 20 minutes and will be deleted afterwards. They should be treated as "testnet" assets and should not be given real value.
> For more information on creating tokens using a recommended production workflow, view the [create a token documentation](https://internetcomputer.org/docs/current/developer-docs/defi/tokens/create).

This application's logic is written in [Rust](https://internetcomputer.org/docs/building-apps/developer-tools/cdks/rust/intro-to-rust), a primary programming language for developing canisters on ICP.

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Deploy" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?url=https://github.com/dfinity/examples/rust/tokenmania)

## Project structure

The `/backend` folder contains the Rust smart contract:

- `Cargo.toml`, which defines the crate that will form the backend
- `lib.rs`, which contains the core logic of the smart contract, and exports its interface
- `types.rs`, which contains type declarations and some conversion functions to keep the main logic cleaner.

The `/frontend` folder contains web assets for the application's user interface. The user interface is written using the React framework.

## Build and deploy from the command-line

To migrate your ICP Ninja project off of the web browser and develop it locally, follow these steps. These steps are necessary if you want to deploy this project for long-term, production use on the mainnet.

### 1. Download your project from ICP Ninja using the 'Download files' button on the upper left corner under the pink ninja star icon.

### 2. Open the `BUILD.md` file for further instructions.
