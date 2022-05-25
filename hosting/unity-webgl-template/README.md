# Unity WebGL Sample

The example shows how to deploy a simple Unity WebGL build on the IC in an asset canister. It just shows a Unity WebGL build with the URP template installed. 

The Unity WebGL build is deployed as frontend, no backend is needed in this sample.


## Installation
This example project can be cloned, installed and deployed locally, for learning and testing purposes. The instructions are based on running the example on either macOS or Linux, but when using WSL2 on Windows, the instructions will be the same.

### Prerequisites
The example project requires the following installed:

- git
- dfx 

git can be installed from various package managers. DFX can be installed following the instructions [here](https://smartcontracts.org/docs/quickstart/local-quickstart.html#download-and-install).

### Install
Install the example dapp project:

```bash
$ git clone https://github.com/dfinity/examples
$ cd examples/hosting/unity-webgl-template
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

## License
This project is licensed under the Apache 2.0 license, see LICENSE.md for details. See CONTRIBUTE.md for details about how to contribute to this project. 
