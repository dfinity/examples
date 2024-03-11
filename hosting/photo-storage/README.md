---
keywords: [photo storage, store photos, photo app, photos, beginner]
---

# Photo storage example

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/photo-storage)

The example shows how to store photos on ICP in an asset canister with the `@dfinity/assets` package. The photo
storage app is deployed as a frontend in an asset canister which is also used for photo upload.

This example project can be cloned, installed, and deployed locally, for learning and testing purposes. The instructions
are based on running the example on either macOS or Linux, but when using WSL2 on Windows, the instructions will be the
same.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).

- [x] Download and install [git](https://git-scm.com/downloads).

- [x] Download and install [Node.js](https://nodejs.org/en).

## Install

Clone the example dapp project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/photo-storage
```

## React build

The React frontend is built by running:

```bash
npm install
npm run build
```

## Deployment

The local replica is started by running:

```bash
dfx start --clean --background
```

When the local replica is up and running, run this command to deploy the canisters:

```bash
dfx deploy
```

## Authorization

To authorize an identity to upload files, it must be authorized first:

```bash
dfx canister call photo-storage authorize '(principal "535yc-uxytb-gfk7h-tny7p-vjkoe-i4krp-3qmcl-uqfgr-cpgej-yqtjq-rqe")'
```

Before deployment on ICP, the hardcoded identity (defined in `src/App.js`) should be replaced by an authentication
method such as Internet Identity.

## Example photos

The example cat stock photos are from [Pexels](https://www.pexels.com/license/).

## License

This project is licensed under the Apache 2.0 license, see `LICENSE.md` for details. See `CONTRIBUTE.md` for details about
how to contribute to this project. 
