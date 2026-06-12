# Superheroes CRUD

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/superheroes)

## Overview

This example demonstrates how to build a CRUD (Create, Read, Update, Delete) application on ICP using Motoko and React. The backend canister stores superhero records (a name and a list of superpowers), and exposes four methods: `create`, `read`, `update`, and `delete`. The React frontend provides a simple UI for interacting with each operation.

## Build and deploy from the command line

### Prerequisites

- Install [Node.js](https://nodejs.org/en/download/)
- Install [icp-cli](https://cli.internetcomputer.org): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install

Clone the example project:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/superheroes
```

### Deploy and test

```bash
icp network start -d
icp deploy
make test
icp network stop
```

The frontend is served by the asset canister. To run the Vite dev server with hot reload during frontend development:

```bash
npm run dev
```

## Updating the Candid interface

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build.

If you modify the backend's public API, regenerate the `.did` file using the Motoko compiler:

```bash
$(mops toolchain bin moc) --idl -o backend/backend.did backend/app.mo
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all the best practices.
