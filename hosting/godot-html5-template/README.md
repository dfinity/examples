# Godot HTML5 Sample

The example shows how to deploy a Godot HTML5 build on the IC in an asset canister. The Godot HTML5 build is deployed as frontend, no backend is needed in this sample.


## Installation
This example project can be cloned, installed and deployed locally, for learning and testing purposes. The instructions are based on running the example on either macOS or Linux, but when using WSL2 on Windows, the instructions will be the same.

### Prerequisites
The example project requires the following installed:

- git
- dfx 

git can be installed from various package managers. DFX can be installed following the instructions [here](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove).

### Download the code
Clone the example dapp project:

```bash
$ git clone https://github.com/dfinity/examples
$ cd examples/hosting/godot-html5-template
```

## Deployment
The local network is started by running this command:

```bash
$ dfx start --background
```

When the local network is up and running, run this command to deploy the canisters:

```bash
$ dfx deploy
```

If you get error code 500 after deploying to the IC mainnet, try to use `raw` keyword in the URL like this: `https://\<canister-id\>.raw.ic0.app`.

## License
This project is licensed under the Apache 2.0 license, see LICENSE.md for details. See CONTRIBUTE.md for details about how to contribute to this project. 
