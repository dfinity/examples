# Continue building locally

Projects deployed through ICP Ninja are temporary; they will only be live for 30 minutes before they are removed. To continue building locally, follow these steps.

### 1. Install developer tools

Install [Node.js](https://nodejs.org/en/download/) and [icp-cli](https://cli.icp.build):

```bash
npm install -g @icp-sdk/icp-cli @icp-sdk/ic-wasm
```

Then navigate into your project's directory that you downloaded from ICP Ninja.

### 2. Deploy locally

Start the local network and deploy the project:

```bash
icp network start -d
icp deploy
```

The local canister URL will be shown in the terminal output. Open it in your web browser.

## Additional examples

Additional code examples and sample applications can be found in the [DFINITY examples repo](https://github.com/dfinity/examples).
