# Canister info

The purpose of this dapp is to give developers a small (backend) dapp that uses the IC's `canister_info` management call to retrieve information about canisters including canister history.

You can find a detailed description of its methods in the form of doc comments in the source code.

Please also refer to the [Interface Specification](https://internetcomputer.org/docs/current/references/ic-interface-spec#ic-canister-info) for details about the `canister_info` management call.

## Deploying from ICP Ninja

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/editor?g=https://github.com/dfinity/examples/tree/master/rust/canister-info)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Run `dfx start --background --clean && dfx deploy` to deploy the project to your local environment. 

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.
