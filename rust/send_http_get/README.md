# HTTP: GET

This example demonstrates how to use the Internet Computer's HTTPS outcalls feature to make a `GET` request from a Rust canister. It sends a request to `postman-echo.com` and returns the echoed JSON response, showing how to pass query parameters and headers through HTTP outcalls.

For a deeper understanding of HTTPS outcalls on the IC, see the [HTTPS outcalls documentation](https://docs.internetcomputer.org/concepts/https-outcalls/).

<!--
[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/send_http_get)
-->

## Build and deploy from the command line

### Prerequisites
- [Node.js](https://nodejs.org/) v18+
- [icp-cli](https://cli.internetcomputer.org/): `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`

### Install

```bash
git clone https://github.com/dfinity/examples
cd examples/rust/send_http_get
```

### Deploy and test

```bash
icp network start -d
icp deploy
bash test.sh
icp network stop
```

## Security considerations and best practices

Refer to the [security best practices](https://docs.internetcomputer.org/guides/security/overview) for information on security and best practices for your ICP app.
