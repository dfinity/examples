# Canister info

The purpose of this dapp is to give developers a small (backend) dapp that uses the IC's `canister_info` management call to retrieve information about canisters including canister history.

You can find a detailed description of its methods in the form of doc comments in the source code.

Please also refer to the [Interface Specification](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-canister-info) for details about the `canister_info` management call.

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Run" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/canister-info)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/developer-docs/security/) for developing on the Internet Computer. This example may not implement all the best practices.

