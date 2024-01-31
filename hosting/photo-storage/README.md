# Photo storage example

The example shows how to store photos on the IC in an asset canister with the `@dfinity/assets` package. The photo
storage app is deployed as a frontend in an asset canister which is also used for photo upload.

## Installation

This example project can be cloned, installed and deployed locally, for learning and testing purposes. The instructions
are based on running the example on either macOS or Linux, but when using WSL2 on Windows, the instructions will be the
same.

### Prerequisites

The example project requires the following installed:

- git
- dfx
- npm

git and npm can be installed from various package managers. DFX can be installed following the
instructions [here](https://internetcomputer.org/docs/current/developer-docs/build/install-upgrade-remove).

### Download the code

Clone the example dapp project:

```bash
$ git clone https://github.com/dfinity/examples
$ cd examples/hosting/photo-storage
```

## React build

The React frontend is build by running this command:

```bash
npm install
npm run build
```

## Deployment

The local network is started by running this command:

```bash
$ dfx start --clean --background
```

When the local network is up and running, run this command to deploy the canisters:

```bash
$ dfx deploy
```

## Authorization

To authorize the identity from this example project on a local network to upload files, it must be authorized first:

```bash
dfx canister call photo-storage authorize '(principal "535yc-uxytb-gfk7h-tny7p-vjkoe-i4krp-3qmcl-uqfgr-cpgej-yqtjq-rqe")'
```

Before deployment on the IC, the hardcoded identity (defined in `src/App.js`) should be replaced by an authentication
method e.g. Internet Identity.

## Cats

The example cat stock photos are from [Pexels](https://www.pexels.com/license/).

## License

This project is licensed under the Apache 2.0 license, see LICENSE.md for details. See CONTRIBUTE.md for details about
how to contribute to this project. 
