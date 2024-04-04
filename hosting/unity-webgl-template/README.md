---
keywords: [unity, unity webgl, beginner]
---

# Unity WebGL sample

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/unity-webgl-template)

The example shows how to deploy a simple Unity WebGL build on ICP in an asset canister. It just shows a Unity WebGL build with the URP template installed. 

The Unity WebGL build is deployed as frontend, no backend is needed in this sample.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).

- [x] Download and install [git](https://git-scm.com/downloads).

## Install
Install the example dapp project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/unity-webgl-template
```

## Deployment
The local replica is started by running this command:

```bash
dfx start --background
```

When the local replica is up and running, run this command to deploy the canisters:

```bash
dfx deploy
```

If you get error code 500 after deploying to the IC mainnet, try to use `raw` keyword in the URL like this: 

```
https://<canister-id>.raw.ic0.app
```

## License
This project is licensed under the Apache 2.0 license, see `LICENSE.md` for details. See `CONTRIBUTE.md` for details about how to contribute to this project. 
