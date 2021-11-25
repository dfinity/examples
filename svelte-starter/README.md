<p align="left" >
  <img width="240"  src="./assets/logo.png">
</p>

# Svelte Dapp template

This repository is meant to give Svelte developers an easy on-ramp to get started with developing decentralized applications (Dapps in short) for the Internet Computer blockchain.

Dapps, also known as smart contracts are specialized software that run on a blockchain.

### What is the Internet Computer?

The Internet Computer is a novel blockchain that has the unique capability to serve web content while not requiring the end users to use a browser extension, such as Metamask.

Coupled with super fast execution the Internet Computer provides the worlds first truly user friendly Web 3.0 experience.

### What are canisters?

Dapps on the Internet Computer live in canisters, which are special smart contracts that run WebAssembly, and can respond to regular HTTP requests, among other capabilities.

This repository uses Svelte for the frontend, and can upload it to an `asset` type canister that can host your frontend on the Internet Computer.

## Install dependencies

Make sure you have [node.js](https://nodejs.org/) installed.

### How to get this template

To clone this template without downloading the entire repository, run the following command:

```
npx degit olaszakos/examples/svelte-starter svelte-starter
```

### DFX

Install `dfx` by running

```
sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"
```

## Start the local replica

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

Make sure you switch back to the project root directory.

To build and deploy the project run

```
dfx deploy
```

When the process completes you'll have a frontend canister running locally. To find the frontend canister's ID, run

```
dfx canister id frontend
```

It will output something similar to `rno2w-sqaaa-aaaaa-aaacq-cai`. Copy this ID and open it in the browser using `http://localhost:8000?canisterId=<canister ID>`, eg. `http://localhost:8000?canisterId=rno2w-sqaaa-aaaaa-aaacq-cai`.

## Local development

You can serve the frontend in development mode like you normally develop a svelte app using the command

```
npm run dev
```

from the project root directory, it is not necessary to deploy it to the frontend canister during development.

## Deploying to the IC

`TODO`
