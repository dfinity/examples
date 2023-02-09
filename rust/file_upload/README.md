# Fileupload in Rust For Internet Computer  
The example dapp shows how to build a very basic dapp, which can upload image files from a frontend to an asset canister. 


## Introduction
The purpose of this example dapp is to build a dapp, based on the default dapp template, installed by `dfx` when creating a new project. The dapp is a simple website with file upload controls. An image file can be selected from the local computer, and uploaded to an asset canister. When the upload is complete, the image is shown on the page.

This example covers:

- Create new canister smart contract using the SDK (`dfx`)
- Use the default project as a template as the starting point for the new project
- Add backend functions for receiving chunked file uploads
- Implement backend functions in the frontend
- Deploy the canister smart contract locally
- Test backend with Candid UI and command line using DFX, and test frontend in browser

## Installation
This example project can be cloned, installed and deployed locally, for learning and testing purposes. The instructions are based on running the example on either macOS or Linux, but when using WSL2 on Windows, the instructions will be the same.

### Prerequisites
The example project requires the following installed:

- git
- Node.js
- dfx

Git and Node can be installed from various package managers. DFX can be installed following the instructions [here](https://smartcontracts.org/docs/quickstart/local-quickstart.html#download-and-install).

### Install
Install the example dapp project:

npm install

The project folder will then look like this:

![Project Files](README_images/project_files.png)


## Deployment
The local network is started by running this command:

```bash
$ dfx start --background
```

When the local network is up and running, run this command to deploy the canisters:

```bash
$ dfx deploy
```


## Testing
The functionality in this example dapp can be tested by using the frontend. Before the example dapp can be tested, it must be deployed (locally) as described in the above Deployment section.

The URL for the frontend depends on the canister ID, which can be retrieved from the `dfx canister id <canister_name>` command.

```bash
$ dfx canister id fileupload_assets
ryjl3-tyaaa-aaaaa-aaaba-cai
```
**http://127.0.0.1:8000/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai&id=ryjl3-tyaaa-aaaaa-aaaba-cai**


## License
This project is licensed under the Apache 2.0 license, see LICENSE.md for details. See CONTRIBUTE.md for details about how to contribute to this project.

## Credit
This project is inspired by, and based on, the project [Fileupload Motoko](https://github.com/dfinity/examples/pull/205).
