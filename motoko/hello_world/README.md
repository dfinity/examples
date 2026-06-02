# Hello, world!

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/hello_world)

## Overview

This example demonstrates a simple "Hello, world!" application for ICP with both a Motoko backend canister and a frontend UI.

The backend canister stores a customizable greeting prefix (default: "Hello, ") as a stable variable, and exposes two methods:

- `setGreeting(prefix)` — updates the greeting prefix (persisted across canister upgrades).
- `greet(name)` — returns the greeting combined with the given name (e.g., "Hello, World!").

The frontend provides a simple form where users can enter their name and receive a personalized greeting from the backend canister.

This application's logic is written in [Motoko](https://docs.internetcomputer.org/motoko/home), a programming language designed specifically for developing canisters on ICP.

## Project structure

The `/backend` folder contains the Motoko canister, `app.mo`. The `/frontend` folder contains web assets for the application's user interface. The user interface is written with plain JavaScript, but any frontend framework can be used.

Edit the `mops.toml` file to add [Motoko dependencies](https://mops.one/) to the project.

## Build and deploy from the command line

### Prerequisites

- [x] Install [Node.js](https://nodejs.org/en/download/)
- [x] Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/hello_world
```

### Deployment

Start the local network and deploy:

```bash
icp network start -d
icp deploy
```

The frontend is served by the asset canister. To run the Vite dev server with hot reload during frontend development:

```bash
npm run dev
```

When done, stop the local network to free the port and clear state:

```bash
icp network stop
```

## Updating the Candid interface

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file using the Motoko compiler:

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
