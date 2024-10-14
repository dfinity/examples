# Svelte Motoko example

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/svelte/svelte-motoko-starter)

This repository is meant to give [Svelte](https://svelte.dev/) developers an easy on-ramp to get started with developing decentralized applications (Dapps in short) for ICP. Dapps, also known as smart contracts, are specialized software that runs on a blockchain.

This template contains:

- A Svelte frontend app under `src/frontend` to be hosted onchain, with support for authentication using Internet Identity.
- A Motoko dapp under `src/backend` to serve as a backend to the Svelte frontend.

You can see a deployed version of this template here: https://zixfv-4yaaa-aaaam-aaatq-cai.ic0.app/

### What is the Internet Computer?

The Internet Computer (ICP) is a novel blockchain that has the unique capability to serve web content while not requiring the end users to use a browser extension, such as Metamask.

Coupled with super-fast execution, ICP provides the world's first truly user-friendly Web 3.0 experience.

### What are canisters?

Dapps on ICP live in canisters, which are special smart contracts that run WebAssembly, and can respond to regular HTTP requests, among other capabilities.

This repository uses the following canisters:

- `backend`: Written in Motoko and will hold the business logic of your dapp.
- `frontend`: The Svelte app, uploaded into a `frontend asset` canister.
- `internet_identity`: An authentication provider written in Rust.

### What is Motoko?

Motoko is a new language designed for the Internet Computer. It is easy to learn for JavaScript and Solidity developers. It was created by the Motoko team at the DFINITY Foundation, led by WebAssembly co-creator Andreas Rossberg. To learn more about the language, check out the [documentation](https://internetcomputer.org/docs/current/motoko/main/motoko).

### What is Internet Identity?

This starter template also includes integration with Internet Identity. Internet Identity is a new authentication framework similar to Github or Google login, but providing complete anonymity to the users. To learn more about Internet Identity check out the [documentation](https://wiki.internetcomputer.org/wiki/What_is_Internet_Identity).

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on ICP. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for creating frontends:
* [Use a well-audited authentication service and client-side IC libraries](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#use-a-well-audited-authentication-service-and-client-side-ic-libraries).
* [Define security headers, including a Content Security Policy (CSP)](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#define-security-headers-including-a-content-security-policy-csp).
* [Don’t load JavaScript (and other assets) from untrusted domains](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#dont-load-javascript-and-other-assets-from-untrusted-domains).


## Install dependencies

Make sure you have [node.js](https://nodejs.org/) installed.

### Clone this template

To clone this template without downloading the entire repository, run the following command:

```
npx degit dfinity/examples/svelte/svelte-motoko-starter svelte-motoko-starter
```

### DFX

Install `dfx` by running

```
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

### Rust

To compile a local version of Internet Identity, you need to have [Rust](https://www.rust-lang.org/learn/get-started) installed.

Also install that target `wasm32-unknown-unknown` by running the command:

```
rustup target add wasm32-unknown-unknown
```

## Start the local replica

Open a new terminal window in the project directory, and run the following command to start the local replica. The replica will not start unless `dfx.json` exists in the current directory.

```
dfx start --background
```

When you're done with development or switching to a different dfx project, run:

```
dfx stop
```

from the project directory will stop the local replica.

## Install Internet Identity

To use Internet Identity during development you need to have it running on your local replica. This repository includes it in a submodule.

To clone the II repository, run:

```
git submodule update --init --recursive
```

When the repository is cloned, switch to its directory and install it:

```
cd internet-identity
npm install
II_FETCH_ROOT_KEY=1 dfx deploy --no-wallet --argument '(null)'
```

This will take several minutes to complete.

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

When the process completes you'll have a backend and a frontend canister running locally. To find the frontend canister's ID, run:

```
dfx canister id frontend
```

It will output something similar to `rno2w-sqaaa-aaaaa-aaacq-cai`. Copy this ID and open it in the browser using `http://localhost:8000?canisterId=<canister ID>`, eg. `http://localhost:8000?canisterId=rno2w-sqaaa-aaaaa-aaacq-cai`.

## Local development

During local development you will be building and deploying the Motoko backend to the local replica. Building the backend will generate declaration files that are Candid and JavaScript files helping the frontend communicate to the backend.

### Motoko backend

If you're using Visual Studio Code it is recommended to use the [Motoko extension](https://marketplace.visualstudio.com/items?itemName=dfinity-foundation.vscode-motoko) developed by the DFINITY Foundation.

To build the backend canister and regenerate the Candid interface declaration files for the frontend run the command:

```
dfx build backend
```

To deploy the backend canister to the local replica you have several options:

`dfx deploy backend` will upgrade your backend canister. In short, upgrading will keep the contents of the variables you marked as stable, in contrast to reinstalling, which will clear the state of your canister.

`dfx deploy backend --mode reinstall` will reinstall the backend canister clearing all existing state.

For more options and other commands see the [dfx CLI reference](https://internetcomputer.org/docs/current/references/cli-reference).

### Svelte frontend

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

