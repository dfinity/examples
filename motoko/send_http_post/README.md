# HTTP: POST

[![Open in ICP Ninja](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/tree/master/motoko/send_http_post)

> 🥷 **Try it live — no local setup.** [ICP Ninja](https://icp.ninja) is a web-based IDE that builds and deploys this project to the mainnet for free, right in your browser. Click the badge above, or hit **Deploy** if you're already in Ninja. To build and run it locally instead, follow the steps below.

This example demonstrates how to use the ICP HTTPS outcalls feature to make a `POST` request from a Motoko canister. It sends a plain-text body to `postman-echo.com/post`, which echoes back the request as JSON, allowing you to verify the POST body and headers were sent correctly.

For a deeper understanding of HTTPS outcalls on the IC, see the [HTTPS outcalls documentation](https://docs.internetcomputer.org/concepts/https-outcalls).

## Build and deploy from the command line

### Prerequisites
- Node.js
- icp-cli: `npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm`
- ic-mops: `npm install -g ic-mops`

### Install
```bash
git clone https://github.com/dfinity/examples
cd examples/motoko/send_http_post
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
