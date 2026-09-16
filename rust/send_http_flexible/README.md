# HTTP: flexible outcall

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/rust/send_http_flexible)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

This example demonstrates a **flexible** HTTPS outcall, made through the management canister's `flexible_http_request` method. A committee of nodes each make the request independently, and the canister receives their individual responses instead of one the subnet reached consensus on. Reconciling them is the canister's job.

That is useful when the data changes faster than the nodes could ever agree on it, which is where a replicated outcall fails consensus. This example asks `postman-echo.com` for the current time, then tallies the distinct answers it got back and reports the most common first.

For a deeper understanding of HTTPS outcalls on the IC, see the [HTTPS outcalls documentation](https://docs.internetcomputer.org/concepts/https-outcalls/).

## Build and deploy from the command line

### Prerequisites
- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/send_http_flexible
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## Security considerations and best practices

A flexible outcall gives up the integrity guarantee that consensus provides, so how the responses are reconciled is part of the security of your canister. Handle any count between `min_responses` and `max_responses`, check each response's status separately, and pick a reconciliation rule that a minority of nodes cannot skew.

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
