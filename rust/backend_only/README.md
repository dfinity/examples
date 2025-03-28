# Rust Backend

This backend only project demonstrates how to write a simple smart contract for ICP.

This application's logic is written in [Rust](https://internetcomputer.org/docs/building-apps/developer-tools/cdks/rust/intro-to-rust), a primary programming language for developing canisters on ICP.

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Deploy" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?url=https://github.com/dfinity/examples/rust/backend_only)

## Project structure

The `/backend` folder contains the Rust smart contract:

- `Cargo.toml`, which defines the crate that will form the backend
- `lib.rs`, which contains the actual smart contract, and exports its interface

## Build and deploy from the command-line

To migrate your ICP Ninja project off of the web browser and develop it locally, follow these steps. These steps are necessary if you want to deploy this project for long-term, production use on the mainnet.

### 1. Download your project from ICP Ninja using the 'Download files' button on the upper left corner under the pink ninja star icon.

### 2. Open the `BUILD.md` file for further instructions.
