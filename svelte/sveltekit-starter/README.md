# SvelteKit Dapp template

This repository is meant to give [SvelteKit](https://kit.svelte.dev/) developers an easy on-ramp to get started with developing decentralized applications (Dapps in short) for the Internet Computer blockchain.

## What is the Internet Computer?

The Internet Computer is a novel blockchain that has the unique capability to serve web content while not requiring the end users to use a browser extension, such as Metamask.

Coupled with super fast execution the Internet Computer provides the worlds first truly user friendly Web 3.0 experience.

## What are canisters?

Dapps on the Internet Computer live in canisters, which are special smart contracts that run WebAssembly, and can respond to regular HTTP requests, among other capabilities.

This repository uses Svelte for the frontend, and can upload it to an `asset` type canister that can host your frontend on the Internet Computer.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for creating frontends:

- [Use a well-audited authentication service and client side IC libraries](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#use-a-well-audited-authentication-service-and-client-side-ic-libraries)
- [Define security headers including a Content Security Policy (CSP)](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#define-security-headers-including-a-content-security-policy-csp)
- [Don’t load JavaScript (and other assets) from untrusted domains](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#dont-load-javascript-and-other-assets-from-untrusted-domains)

## Getting started

Make sure you have [node.js](https://nodejs.org/) installed.

```
git clone https://github.com/dfinity/examples
cd svelte/sveltekit-starter
npm ci
```

## DFX

Install `dfx` by running

```
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

### Start and stop the local replica

Open a new terminal window _in the project directory_, and run the following command to start the local replica. The replica will not start unless `dfx.json` exists in the current directory.

```
dfx start --background
```

When you're done with development, or you're switching to a different dfx project, running

```
dfx stop
```

from the project directory will stop the local replica.

## Build & run the dapp

To build and deploy the project locally run

```
dfx deploy
```

When the process completes you'll have a frontend canister running locally. To find the frontend canister's ID, run

```
dfx canister id frontend
```

It will output something similar to `rno2w-sqaaa-aaaaa-aaacq-cai`. Copy this ID and open it in the browser using `http://<canister ID>localhost:8000`, eg. `http://rno2w-sqaaa-aaaaa-aaacq-cai.localhost:8000`.

## Local development

You can serve the frontend in development mode like you normally develop an app using the command

```
npm run dev
```

it is not necessary to deploy it to the frontend canister during development.

## Deploying to the IC

To host the Svelte app on the IC, you'll need to have some cycles available. Cycles pay for the execution of your app, and they are also needed to create canisters.

You can get $20 worth of cycles for free from the [Cycles Faucet](https://faucet.dfinity.org).

You should have a canister running the cycles wallet on the IC at this point. The cycles wallet makes it easy to pay for canister creation.

You can check the balance by running

```
dfx wallet --network ic balance
```

After making sure you have cycles available you can run

```
dfx deploy --network ic
```

The command will build the project, create a new canister on the IC and deploy the Svelte app into it. The command will also create a new file `canister_ids.json` which will help the dfx tool deploy to the same canister in future updates. You can commit this file in your repository.

You can now open your Svelte app running on the IC. You can find the canister ID in the deploy command output, or use the ID in `canister_ids.json`.

The link to your app is `<canister_id>.ic0.app`. For example if your canister ID is `zgvi5-hiaaa-aaaam-aaasq-cai`, your app will be at `https://zgvi5-hiaaa-aaaam-aaasq-cai.ic0.app/`.
