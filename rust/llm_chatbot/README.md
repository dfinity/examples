# LLM Chatbot

This example demonstrates how an ICP smart contract can interact with a large language model (LLM) to generate text. The user can input a prompt and the smart contract will use the LLM to generate a response. Follow-up prompts continue the conversation with the full message history.

## How it works

The backend canister calls the `w36hm-eqaaa-aaaal-qr76a-cai` LLM canister on mainnet via the [`ic-llm`](https://crates.io/crates/ic-llm) crate. Locally, `icp deploy` deploys a copy of the LLM canister (backed by Ollama) and automatically injects its canister ID into the backend as the `PUBLIC_CANISTER_ID:llm` environment variable — `ic-llm` reads this at runtime so local calls go to the local LLM. On mainnet, the env var is absent and `ic-llm` falls back to the hardcoded mainnet principal.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Set up Ollama (local deployment only)

The LLM canister delegates inference to [Ollama](https://ollama.com/). Install it and then run:

```bash
ollama serve
```

In a separate terminal, pull the model (about 4 GiB, one-time download):

```bash
ollama pull llama3.1:8b
```

Once the model is loaded you can stop the `ollama run` command — `ollama serve` keeps it available.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/llm_chatbot
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

Open the frontend URL printed by `icp deploy` to use the chatbot in the browser.

For hot-reload frontend development:

```bash
npm run dev --prefix frontend
```

## Deploying to mainnet

```bash
icp deploy -e ic
```

No Ollama setup is needed — mainnet calls go directly to the LLM canister at `w36hm-eqaaa-aaaal-qr76a-cai`.

## Updating the Candid interface

```bash
icp build backend && candid-extractor target/wasm32-unknown-unknown/release/backend.wasm > backend/backend.did
```

## Security considerations and best practices

If you base your application on this example, familiarize yourself with the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all best practices.
