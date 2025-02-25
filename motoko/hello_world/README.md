# Hello, world!

![Hello, world!](/examples/_attachments/hello_world.png)

This simple example demonstrates how applications on ICP are structured using a backend smart contract and a frontend smart contract, and provides an introduction to the programming language Motoko.

This variation of "Hello, world!" is written in [Motoko](https://internetcomputer.org/docs/current/motoko/main/getting-started/motoko-introduction), a programming language designed specifically for developing smart contracts (referred to as **canisters**) on ICP.

### Project structure

The `/backend` folder contains the Motoko canister, `app.mo`. The `/frontend` folder contains web assets for the application's user interface. The user interface is written with plain JavaScript, but any frontend framework can be used.

Edit the `mops.toml` file to add [Motoko dependencies](https://mops.one/) to the project.

## Continue building locally

To migrate your ICP Ninja project off of the web browser and develop it locally, follow these steps.

### 1. Download your project from ICP Ninja using the 'Download files' button.

![ICP Ninja download](/examples/_attachments/icp_ninja_download_files.png)

### 2. Open the `BUILD.md` file for further instructions.

The `BUILD.md` file included in your download will provide information about using `dfx`.
