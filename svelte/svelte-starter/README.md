# Svelte template

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/svelte/svelte-starter)

This repository is meant to give [Svelte](https://svelte.dev/) developers an easy on-ramp to get started with developing decentralized applications (Dapps in short) for ICP. Dapps, also known as smart contracts, are specialized software that runs on a blockchain.

This template contains a Svelte app under `src/frontend` that can be hosted onchain on ICP.

You can see a deployed version of this template here: https://zgvi5-hiaaa-aaaam-aaasq-cai.ic0.app/

### What is the Internet Computer?

The Internet Computer (ICP) is a novel blockchain that has the unique capability to serve web content while not requiring the end users to use a browser extension, such as Metamask.

Coupled with super-fast execution, ICP provides the world's first truly user-friendly Web 3.0 experience.

### What are canisters?

Dapps on ICP live in canisters, which are special smart contracts that run WebAssembly, and can respond to regular HTTP requests, among other capabilities.

This repository uses Svelte for the frontend, and that is uploaded to an `asset` type canister once deployed.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on ICP. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for creating frontends:
* [Use a well-audited authentication service and client-side IC libraries](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#use-a-well-audited-authentication-service-and-client-side-ic-libraries).
* [Define security headers, including a Content Security Policy (CSP)](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#define-security-headers-including-a-content-security-policy-csp).
* [Don’t load JavaScript (and other assets) from untrusted domains](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#dont-load-javascript-and-other-assets-from-untrusted-domains).

## Install dependencies

Make sure you have [node.js](https://nodejs.org/) installed.

### Clone the template

To clone this template without downloading the entire repository, run the following command:

```
npx degit dfinity/examples/svelte-starter svelte-starter
```

### DFX

Install `dfx` by running

```
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

## Start the local replica

Open a new terminal window in the project directory, and run the following command to start the local replica. The replica will not start unless `dfx.json` exists in the current directory.

```
dfx start --background
```

When you're done with development, or you're switching to a different dfx project, running

```
dfx stop
```

from the project directory will stop the local replica.

## Build and run the dapp

Make sure you switch back to the project root directory.

First, install the frontend dependencies by running:

```
cd src/frontend
npm install
cd ..
```

To build and deploy the project:

```
dfx deploy
```

When the process completes, you'll have a frontend canister running locally. To find the frontend canister's ID, run

```
dfx canister id frontend
```

It will output something similar to `rno2w-sqaaa-aaaaa-aaacq-cai`. Copy this ID and open it in the browser using `http://localhost:8000?canisterId=<canister ID>`, eg. `http://localhost:8000?canisterId=rno2w-sqaaa-aaaaa-aaacq-cai`.

## Local development

You can serve the frontend in development mode like you normally develop a Svelte app using the command:

```
npm run dev
```

## Deploying to the mainnet

To host the Svelte app on ICP, you'll need to have some cycles available. Cycles pay for the execution of your app, and they are also needed to create canisters.

You can get cycles for free from the cycles faucet. To claim them, follow [this guide](https://internetcomputer.org/docs/current/developer-docs/setup/cycles/cycles-faucet).

After following that guide, you should have a balance of cycles to use. You can check the balance by running:

```
dfx wallet --network ic balance
```

After making sure you have cycles available, you can run

```
dfx deploy --network ic
```

The command will build the project, create a new canister on ICP and deploy the Svelte app into it. The command will also create a new file `canister_ids.json` which will help the dfx tool deploy to the same canister in future updates. You can commit this file in your repository.

You can now open your Svelte app running onchain. You can find the canister ID in the deploy command output, or use the ID in `canister_ids.json`.

The link to your app is `<canister_id>.ic0.app`. For example, if your canister ID is `zgvi5-hiaaa-aaaam-aaasq-cai`, your app will be at `https://zgvi5-hiaaa-aaaam-aaasq-cai.ic0.app/`.
