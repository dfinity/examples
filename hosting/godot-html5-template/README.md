---
keywords: [html5, html, godot, hosting, host a website, beginner]
---

# Godot HTML5 sample

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/hosting/godot-html5-template)

## Overview
The example shows how to deploy a Godot HTML5 build on the IC in an asset canister. The Godot HTML5 build is deployed as frontend, no backend is needed in this sample.

This example project can be cloned, installed, and deployed locally, for learning and testing purposes. The instructions are based on running the example on either macOS or Linux, but when using WSL2 on Windows, the instructions will be the same.

## Prerequisites

This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/).

- [x] Download and install [git](https://git-scm.com/downloads).

## Install

Clone the example dapp project:

```bash
git clone https://github.com/dfinity/examples
cd examples/hosting/godot-html5-template
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

If you get error code 500 after deploying to the IC mainnet, try to use `raw` keyword in the URL like this: `https://<canister-id>.raw.ic0.app`.

## License
This project is licensed under the Apache 2.0 license, see `LICENSE.md` for details. See `CONTRIBUTE.md` for details about how to contribute to this project. 
