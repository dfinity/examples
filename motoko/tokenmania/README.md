# Tokenmania!

Tokenmania is a simplified token minting application. When the application is ran, you will be prompted to sign in with Internet Identity. Once signed in, select the 'Mint' function. It will mint tokens based on the backend smart contract's hardcoded configuration values for things such as token name, token symbol, and total supply. The owner principal of the token will be your Internet Identity principal.

> [!CAUTION]
> This example is for demonstration purposes. It does not reflect a best practices workflow for creating and minting tokens on ICP.
> Actual production tokens deployed on ICP use a dedicated ledger smart contract and an index smart contract. For this example's demonstration, this functionality has been simplified and the ledger functionality is included in the backend smart contract.
> Tokens deployed using this example are only available for 20 minutes and will be deleted afterwards. They should be treated as "testnet" assets and should not be given real value.
> For more information on creating tokens using a recommended production workflow, view the [create a token documentation](https://internetcomputer.org/docs/current/developer-docs/defi/tokens/create).

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Run" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/motoko/tokenmania)

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
