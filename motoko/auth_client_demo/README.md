# Auth-Client Demo

This is an example project, intended to demonstrate how an app developer might integrate with an [Internet Identity](https://identity.ic0.app).

For a non-typescript implementation, see https://github.com/krpeacock/auth-client-demo/tree/vanilla-js

[Live demo](https://vasb2-4yaaa-aaaab-qadoa-cai.ic0.app/)

This is an example showing how to use [@dfinity/auth-client](https://www.npmjs.com/package/@dfinity/auth-client).

## Setting up for local development

To get started, start a local dfx development environment in this directory with the following steps:

```bash
cd auth-client-demo/
dfx start --background --clean
dfx deps deploy
dfx deploy
```

Once deployed, start the development server with `npm start`.

You can now access the app at `http://127.0.0.1:5173/`.

## Multiple Versions

This demo has multiple versions, each of which demonstrates a different feature of the auth-client. `npm start` will run the vanilla JS version, but you can run the others by running `npm run start:version` where `version` is one of the following:

- React
- Vue
- Vanilla
- Svelte

## Pulling Internet Identity into your own project

To pull Internet Identity into your own project, you'll need to do the following:

1. Add Internet Identity to your `dfx.json` file:

```json
"internet-identity" : {
    "type": "pull",
    "id": "rdmx6-jaaaa-aaaaa-aaadq-cai"
}
```

2. Run the following commands to install the dependencies:

```bash
dfx deps pull
dfx deps init --argument '(null)' internet-identity
dfx deps deploy
```


