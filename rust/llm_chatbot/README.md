# LLM Chatbot

This example demonstrates how an ICP canister can interact with a large language model (LLM) to generate text. The user can input a prompt and the canister will use the LLM to generate a response. Follow-up prompts continue the conversation with the full message history.

## How it works

The backend canister calls the [LLM canister](https://forum.dfinity.org/t/introducing-the-llm-canister-deploy-ai-agents-with-a-few-lines-of-code/41424)'s `v1_chat` endpoint directly (see `backend/lib.rs`), without a helper crate. Locally, `icp deploy` deploys a copy of the LLM canister (backed by Ollama) and automatically injects its canister ID into the backend as the `PUBLIC_CANISTER_ID:llm` environment variable — the backend reads this at runtime so local calls go to the local LLM. On mainnet, the env var is absent and the backend falls back to the hardcoded mainnet principal `w36hm-eqaaa-aaaal-qr76a-cai`.

## Build and deploy from the command line

### Prerequisites

- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Set up Ollama (local deployment only)

The LLM canister delegates inference to [Ollama](https://ollama.com/). Install it and then run:

```bash
ollama serve
```

In a separate terminal, download the model (about 4 GiB, one-time) and load it
into memory:

```bash
ollama run llama3.1:8b "hi"
```

`ollama run` pulls the model if needed and warms it in memory. This matters: the
LLM canister's HTTP outcall to Ollama has a ~30 s deadline, and a cold model
load alone can take longer than that — so the *first* call after `ollama serve`
starts may time out (`SysFatal: Timeout expired`) if the model isn't warm yet.
Warming it first avoids this; `ollama serve` then keeps it loaded. If you do hit
a timeout on the first call, simply retry — the model stays resident afterwards.

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/llm_chatbot
```

### Deploy

```bash
icp network start -d
icp deploy
```

Open the frontend URL printed by `icp deploy` to use the chatbot in the browser. Make sure Ollama is running with the model warmed (see above) so the first message does not time out.

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
