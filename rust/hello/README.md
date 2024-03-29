---
keywords: [beginner, rust, hello]
---

# Hello, world!

[View this sample's code on GitHub](https://github.com/dfinity/examples/tree/master/rust/hello)

## Overview 
This sample demonstrates a simple dapp consisting of two canisters:

-   A simple backend canister, `hello`, implementing the logic of the application.

-   A simple frontend asset canister, `hello_assets`, serving the assets of the dapp’s web user interface.

It is the dapp equivalent of the ubiquitous 'Hello, world!' and can be seen running [here on the IC](https://6lqbm-ryaaa-aaaai-qibsa-cai.ic0.app/).

## Architecture

This sample is based on the default project created by running `dfx new` as described in the quick start documents.

The sample code is available from the [samples](https://github.com/dfinity/examples) repository in both [Motoko](https://github.com/dfinity/examples/tree/master/motoko/hello) and [Rust](https://github.com/dfinity/examples/tree/master/rust/hello).

Canister `hello`, whether implemented in Motoko or Rust, presents the same Candid interface:
```candid
service : {
  greet: (text) -> (text);
}
```

The frontend canister, `hello_assets`, displays an HTML page with a text box for the argument and a button for calling the function greet with that argument. The result of the call is displayed in a message box.

The frontend canister is a generic canister provided by `dfx` but the assets it serves to browsers are determined by the dfx project settings and project files.

The frontend canister and its assets are identical for both projects.

This example demonstrates a dead simple dapp consisting of two canister smart contracts:

- A simple backend canister, hello, implementing the logic of the application in Motoko.
- A simple frontend asset canister, hello_assets serving the assets of the dapp's web user interface.

This example is based on the default project created by running `dfx new hello`.

This example is based on the default project created by running `dfx new --type=rust hello`.

### Prerequisites 
This example requires an installation of:
- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).
- [x] Install `node.js` (to build the web frontend).

 ### Step 1: Open a terminal window.

If you haven't already, create a default project with the command:

```
dfx new --type=rust hello
cd hello
```

 ### Step 2: Start a local canister execution environment:

```
dfx start --background
```

 ### Step 3: Ensure that the required node modules are available in your project directory, if needed, by running the following command:

```
npm install
```

 ### Step 4: Register, build, and deploy the project with the command:

```
dfx deploy
npm start
```

 ### Step 5: Call the hello canister's greet function:

```
dfx canister call hello_backend greet everyone
```

 ### Step 6: Observe the following result:

```
("Hello, everyone!")
```

The previous steps use `dfx` to directly call the function on the hello (backend) canister. To access the web user interface of the dapp, that is served by canister hello_assets, do the following:

 ### Step 7: Determine the URL of the hello_frontend asset canister.

```
echo "http://localhost:8000/?canisterId=$(dfx canister id hello_frontend)"
```

 ### Step 8: Navigate to the URL in your browser.
The browser should display a simple HTML page with a sample asset image file, an input field, and a button.

 ### Step 9: Enter the text "everyone" and click the button to see the greeting returned by the backend hello canister.

### Troubleshooting
If the web page doesn't display properly or displays the wrong content, you may need to clear your browser cache.

Alternatively, open the URL in a fresh, in-private browser window to start with a clean cache.

### Resources
- [ic-cdk](https://docs.rs/ic-cdk/latest/ic_cdk/).
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros).
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.ic0.app/).


## Security considerations and security best practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [security best practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

