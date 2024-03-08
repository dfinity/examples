---
keywords: [intermediate, motoko, authentication, internet identity, integrate, auth, user auth]
---

# Auth-client 

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/motoko/auth_client_demo)

This is an example project intended to demonstrate how a developer might integrate with [Internet Identity](https://identity.ic0.app).

:::info
This example uses TypeScript. See an alternative [vanilla JS example](https://github.com/krpeacock/auth-client-demo/tree/vanilla-js).
:::

[View a live demo of this sample](https://vasb2-4yaaa-aaaab-qadoa-cai.ic0.app/).

This example shows how to use [@dfinity/auth-client](https://www.npmjs.com/package/@dfinity/auth-client).

## Setting up for local development

To get started, start a local `dfx` development environment in this directory with the following steps:

```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/auth-client-demo/
dfx start --background --clean
dfx deps deploy
dfx deploy
```

Once deployed, start the development server with `npm start`.

You can now access the app at `http://127.0.0.1:5173/`.

## Multiple versions

This demo has multiple versions, each of which demonstrates a different feature of the auth-client. `npm start` will run the vanilla JS version, but you can run the others by running `npm run start:version` where `version` is one of the following:

- React
- Vue
- Vanilla
- Svelte

## Pulling Internet Identity into your project

To pull Internet Identity into your project, you'll need to do the following:

- #### Step 1: Add Internet Identity to your `dfx.json` file:

```json
"internet-identity" : {
    "type": "pull",
    "id": "rdmx6-jaaaa-aaaaa-aaadq-cai"
}
```

- #### Step 2: Run the following commands to install the dependencies:

```bash
dfx deps pull
dfx deps init --argument '(null)' internet-identity
dfx deps deploy
```


