# HTTP: POST

<!--
[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/send_http_post)
-->

This example demonstrates how to use the ICP HTTPS outcalls feature to make a `POST` request from a Rust canister. It sends a plain-text body to `postman-echo.com/post`, which echoes back the request as JSON, allowing you to verify the POST body and headers were sent correctly.

For a deeper understanding of HTTPS outcalls on the IC, see the [HTTPS outcalls documentation](https://docs.internetcomputer.org/building-apps/network-features/using-http/https-outcalls/post).

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/rust/send_http_post
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
