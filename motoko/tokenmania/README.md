# Tokenmania!

Tokenmania is a simplified token minting application. When the application is ran, you will be prompted to sign in with Internet Identity. Once signed in, select the 'Mint' function. It will mint tokens based on the backend smart contract's hardcoded configuration values for things such as token name, token symbol, and total supply. The owner principal of the token will be your Internet Identity principal.

> [!CAUTION]
> This example is for demonstration purposes. It does not reflect a best practices workflow for creating and minting tokens on ICP.
> Actual production tokens deployed on ICP use a dedicated ledger smart contract and an index smart contract. For this example's demonstration, this functionality has been simplified and the ledger functionality is included in the backend smart contract.
> Tokens deployed using this example are only available for 20 minutes and will be deleted afterwards. They should be treated as "testnet" assets and should not be given real value.
> For more information on creating tokens using a recommended production workflow, view the [create a token documentation](https://internetcomputer.org/docs/current/developer-docs/defi/tokens/create).

This application's logic is written in [Motoko](https://internetcomputer.org/docs/motoko/main/getting-started/motoko-introduction), a programming language designed specifically for developing canisters on ICP.

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Deploy" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?url=https://github.com/dfinity/examples/motoko/tokenmania)

## Project structure

The `/backend` folder contains the Motoko canister, `app.mo`. The `/frontend` folder contains web assets for the application's user interface. The user interface is written using the React framework. Edit the `mops.toml` file to add [Motoko dependencies](https://mops.one/) to the project.

## Build and deploy from the command-line

To migrate your ICP Ninja project off of the web browser and develop it locally, follow these steps. These steps are necessary if you want to deploy this project for long-term, production use on the mainnet.

### 1. Download your project from ICP Ninja using the 'Download files' button on the upper left corner under the pink ninja star icon.

### 2. Open the `BUILD.md` file for further instructions.
