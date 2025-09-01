# Svelte template

This example is meant to give [Svelte](https://svelte.dev/) developers an easy on-ramp to get started with developing decentralized applications (Dapps in short) for ICP. Dapps, also known as smart contracts, are specialized software that runs on a blockchain.

This template contains a Svelte app under `src/frontend` that can be hosted onchain on ICP.

You can see a deployed version of this template here: https://zgvi5-hiaaa-aaaam-aaasq-cai.ic0.app/

## Deploying from ICP Ninja

When viewing this project in ICP Ninja, you can deploy it directly to the mainnet for free by clicking "Run" in the upper right corner. Open this project in ICP Ninja:

[![](https://icp.ninja/assets/open.svg)](https://icp.ninja/i?g=https://github.com/dfinity/examples/rust/backend_only)

## Build and deploy from the command-line

### 1. [Download and install the IC SDK.](https://internetcomputer.org/docs/building-apps/getting-started/install)

### 2. Download your project from ICP Ninja using the 'Download files' button on the upper left corner, or [clone the GitHub examples repository.](https://github.com/dfinity/examples/)

### 3. Navigate into the project's directory.

### 4. Deploy the project to your local environment:

```
dfx start --background --clean && dfx deploy
```

## Security considerations and best practices

If you base your application on this example, it is recommended that you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/building-apps/security/overview) for developing on ICP. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for creating frontends:
* [Use a well-audited authentication service and client-side IC libraries](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#use-a-well-audited-authentication-service-and-client-side-ic-libraries).
* [Define security headers, including a Content Security Policy (CSP)](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#define-security-headers-including-a-content-security-policy-csp).
* [Don’t load JavaScript (and other assets) from untrusted domains](https://internetcomputer.org/docs/current/references/security/web-app-development-security-best-practices#dont-load-javascript-and-other-assets-from-untrusted-domains).
