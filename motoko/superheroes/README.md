# CRUD example

This example demonstrates how to build a CRUD application on ICP using Motoko and React.

This is a Motoko example that does not currently have a Rust variant.

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Clone the example dapp project: `git clone https://github.com/dfinity/examples`

Begin by opening a terminal window.

## Step 1: Setup the project environment

Navigate into the folder containing the project's files and start a local instance of the Internet Computer with the commands:

```bash
cd examples/motoko/superheroes
dfx start --background
```

## Step 2: Deploy the canisters

```bash
dfx deploy
```

## Step 3: Take note of the URL at which the canister is accessible

```bash
echo "http://127.0.0.1:4943/?canisterId=$(dfx canister id www)"
```

## Step 4: Open the aforementioned URL in your web browser.

## Security considerations and best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Use HTTP asset certification and avoid serving your dApp through raw.ic0.app](https://internetcomputer.org/docs/current/developer-docs/security/security-best-practices/overview), since this app serves a frontend.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this app uses query calls.

