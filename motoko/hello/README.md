# Hello Example

## Summary

This example demonstrates a dead simple dapp consisting of:

* a simple backend canister, `hello`, implementing the logic of the application in Motoko, and
* a simple frontend asset canister, `hello_assets` serving the assets of the dapp's web user interface.

The example is the template example created by running `dfx new` as described
https://smartcontracts.org/docs/quickstart/local-quickstart.html[here].

### Rust variant

A version of this example with a Rust implementation of canister `hello` can be found [here](../../rust/hello).

## Interface

Canister `hello` is defined as a Motoko actor with the Candid interface:

```
service : {
  greet: (text) -> (text);
}
```

The frontend displays a html text box and a button for suppling the argument and calling the function greet with that argument. The result of the call is displayed in a text box.


## Requirements

The example requires an installation of
* [DFINITY Canister SDK](https://sdk.dfinity.org).
* node.js (to build the web frontend).

(The Rust version of this example additionally requires a working Rust environment.)

## Setup

Check, you have stopped any local canister execution environment (i.e. `replica`) or other network process that would create a port conflict on 8000.


## Running Locally

Using two terminal windows, do the following steps:

1. Open the first terminal window.

1. Start a local canister execution environment

   ```text
   dfx start
   ```

   This command produces a lot of distracting diagnostic output which is best ignored by continuing in a second terminal.

1. Open the second terminal window.

1. Ensure that the required `+node+` modules are available in your project direc   tory, if needed, by running the following command:

   ```text
   npm install
   ```

1. Register, build and deploy the project.

   ```text
   dfx deploy
   ```

1. Call the hello canister's greet function:

   ```text
   dfx canister call hello greet '("everyone")'
   ```

1. Observe the following result.

   ```text
   ("Hello,a everyone!")
   ```

The previous steps use `dfx` to directly call the function on the `hello` (backend) canister.

To access the web user interface of the dapp, that is served by caniser `hello_assets`, do the following:

1. Determine the url of the `hello_assets` asset canister.

  ```text
  echo "http://localhost:8000/?canisterId=$(dfx canister id hello_assets)"
  ```

1. Navigate to the URL in your browser.

2. The browser should display a simple HTML page with a sample asset image file, an input field, and a button.

3. Enter the text `everone` and click the button to see the greeting returned by the backend `hello` canister.

## Troubleshooting

If the UI doesn't display properly, or displays the wrong contents,
you may need to clear your browser acache.

Alternatively, use a new in-private browser window to start with a clean cache.

## Links

For instructions on how to create this example from scratch as well as a more detailed walkthrough and some tips on frontend development using a development server, see:

- [Local Development](https://sdk.dfinity.org/docs/quickstart/quickstart-intro.html)

Other related links you might find useful are:

- [Motoko Programming Language Guide](https://sdk.dfinity.org/docs/language-guide/motoko.html)
- [Motoko Language Quick Reference](https://sdk.dfinity.org/docs/language-guide/language-manual.html)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.ic0.appa)
