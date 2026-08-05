# LLM Chatbot

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/motoko/llm_chatbot)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

This example demonstrates how an ICP canister can interact with a large language model (LLM) to generate text. The user can input a prompt and the canister will use the LLM to generate a response. Follow-up prompts continue the conversation with the full message history.

## How it works

The backend canister calls the [LLM canister](https://forum.dfinity.org/t/introducing-the-llm-canister-deploy-ai-agents-with-a-few-lines-of-code/41424)'s `v1_chat` endpoint directly (see `backend/app.mo`), without a helper library. It reaches the canister through the typed import `import LLM "canister:llm"` — no LLM actor type is hand-written. The import is typed against the LLM canister's committed Candid interface (`candid/llm.did`), and the `--actor-env-alias` flag in `mops.toml` binds it to the `PUBLIC_CANISTER_ID:llm` environment variable. Locally, `icp deploy` deploys a copy of the LLM canister (backed by Ollama) and injects this variable automatically. On mainnet the shared LLM canister already exists, so `icp.yaml` sets the variable to its principal (`w36hm-eqaaa-aaaal-qr76a-cai`) for the `ic` environment. The principal is resolved at canister install/upgrade, so the same Wasm runs in both environments.

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
cd examples/motoko/llm_chatbot
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

The `backend/backend.did` file defines the backend canister's public interface. The frontend TypeScript bindings are auto-generated from this file during the frontend build. If you modify the backend's public API, regenerate the `.did` file:

```bash
mops generate candid backend
```

`candid/llm.did` is the **LLM canister's own interface**, not the backend's — the `canister:llm` import is typed against it. The `candid/` directory holds the interfaces of external canisters this project calls (as opposed to `backend/backend.did`, which is this project's own interface). These are not produced by `mops generate candid` — each is the Candid interface of an external canister. To refresh one (e.g. after bumping the LLM release), get it straight from the canister with either of:

**From mainnet** — the live shared LLM canister. One command, no files to handle:

```bash
icp canister metadata w36hm-eqaaa-aaaal-qr76a-cai candid:service -e ic > candid/llm.did
```

**From the pinned Wasm** — matches exactly what deploys locally. The Wasm is the pre-built artifact pinned in this project's `icp.yaml` (the `llm` canister's `build.steps[].url`); download it, then extract its interface (the LLM Wasm is not gzipped):

```bash
curl -sSL https://github.com/dfinity/llm/releases/download/v0.3.1/llm-canister.wasm -o llm-canister.wasm
ic-wasm llm-canister.wasm metadata candid:service > candid/llm.did
```

Both give the same interface as long as `icp.yaml` pins the release that is live on mainnet.

## Security considerations and best practices

If you base your application on this example, familiarize yourself with the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for developing on ICP. This example may not implement all best practices.
