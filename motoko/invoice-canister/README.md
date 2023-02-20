# Invoice Canister

This project provides an interface for creating and paying invoices in ICP and ICRC1 based tokens on the Internet Computer. It is a custodial solution, intended to provide a robust starting point for integrating payment flow supporting ICP and ICRC1 transactions in other canisters.

This project demonstrates integration of support for four different tokens, two of which are of the ICP standard and two of the ICRC1 standard. As this requires four token-ledger canisters, three of these are installed by the downloaded wasm and did files provided by the [Dfinity Rosetta-API repository](https://github.com/dfinity/ic/tree/master/rs/rosetta-api). These files can be found in the `src/token-ledger-canisters/` directory with an accompanying shell script for downloading them independently of this project. The ICRC1 token-ledger canister wasm and did is deployed twice, once for each of the two ICRC1 tokens integrated. The ICP ledger wasm and did is only deployed once as the other ICP based token has its ledger canister deployed by running the `dfx nns install` command. This to demonstrate the multiple ways of integrating token-ledger canisters.

In the event your project only requires support for the ICP ledger canister and a single ICRC1 token-ledger canister, it may be easier to use the `motoko-seller-client` `Invoice.mo` and its accompanying supporting modules as that project only integrates support for two tokens, one ICP and one ICRC1. Other than this and the mock ledgers used, a principal added for access control, and an API utility function for depositing funds to mock payment completion for an invoice, the code of those files are the same as this project. For more details see the [Design Doc](./docs/DesignDoc.md). 

## Getting Started - Development

Before running this project, it is necessary to check the system wide network configuration is set according to what the canisters installed by `dfx nns install` require.

Run `cat "$(dfx info networks-json-path)"` to confirm it matches:

```json
{
  "local": {
    "bind": "127.0.0.1:8080",
    "type": "ephemeral",
    "replica": {
      "subnet_type": "system"
    }
  }
}
```
If it does not, make a backup of the original file. Once the original has been backed up or if the `networks.json` file does not exist, use a text editor or `cat` command to set the `networks.json` to match the above. One completed, this project's startup script can be run. More details about using the `dfx nns` command can be found [here](https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md). 

To integrate support for the four token-ledger canisters this project uses, the [zx](https://github.com/google/zx) command line scripting library is used to start up a local replica correctly configured and specifically so for testing if running tests. This is the [clean-startup.mjs](./clean-startup.mjs) which contains documentation explaining how it restarts dfx, runs `dfx nns install`, adds an identity used for testing if needed, deploys the invoice and other token-ledger canisters, and finally, if testing, disbursing funds to that identity used in the E2E testing.

For convenience, two npm scripts have been added to initiate this script:  

`npm run deployAll`  
`npm run deployForTesting`  

As mentioned if deployed for testing, the Secp256k1 identity the `dfx nns install` uses as one its two initial deposit identities is added and switched to as the current user. If testing or not, the current identity is used as the minting account for all four token-ledger canisters. See the [clean-startup.mjs](./clean-startup.mjs) for more details. 

The script will check if the system wide networks configuration file is correctly set before running. If correctly configured, either of the two above commands can be used to start a local replica with all the deployed canisters ready. Note the command line arguments used with `dfx` in this script are first made a variable and can be logged to the console to be manually used as a `dfx` command, modified with custom fields if need be.  

The invoice canister can process invoices using transaction of any of the four token-ledger canisters. See the [Design Doc](./docs/DesignDoc.md), Motokodoc comments of the [Invoice API](./src/invoice/Invoice.mo), or introductory comments of the [clean-startup.mjs](./clean-startup.mjs) for more details.

## Integrating the Invoice Canister

To integrate the invoice canister in another project, review the [Design Doc](./docs/DesignDoc.md), [Invoice.mo](./src/invoice/Invoice.mo) and [SupportedToken.mo](./src/invoice/modules/SupportedToken.mo). Determine which tokens are to be supported, as configuring a single invoice canister to support adding new ICRC1 tokens after it has already been deployed requires extra initial configuration of the variant's references used to map the supported tokens. See "Future Proofing" in the [Design Doc](./docs/DesignDoc.md) for more details. Both the `Invoice.mo` and `SupportedToken.mo` files will need to be edited to reflect support for the tokens to be supported as outlined in the [Design Doc](./docs/DesignDoc.md) and explained [SupportedToken.mo](./src/invoice/modules/SupportedToken.mo). 

To summarize, an additional `SupportedToken` variant tag is needed for each token to be supported and their corresponding cases in each of the switches the `SupportedToken` variant is used must be updated to include that added variant tag: these switches are only found in the methods at the file scope of `SupportedToken.mo` and in the API methods of `Invoice.mo`. The API methods involve the actual calls that are made to the corresponding `ICP` ledger and `ICRC1` token-ledgers, which can instantiated with their canister id using the `ICP` and `ICRC1` supertype actors in `SupportedToken.mo`. An easy way to see where all of the edits are needed is modify the original `SupportedToken` variant (line 529 in `SupportedToken.mo`) to add or remove a tag, and see where the VSCode Motoko extension indicates the impacted switches are. Once `Invoice.mo` and `SupportedToken.mo` are finished being updated, add them along with `Types.mo` to the project. 

An example of integration with only two tokens, one for ICP and one for ICRC1 mapped to the variant tags `#ICP` and `#ICRC1` respectively, along with two corresponding mock ledgers that can be used, can be found in the `examples` directory and is the [motoko-seller-client](./examples/motoko-seller-client/) project.

## Testing

To test, you will need to install `moc` from the latest `motoko-<system>-<version>.tar.gz` release. https://github.com/dfinity/motoko/releases.

Then, install Vessel following the guide at https://github.com/dfinity/vessel.

You will also need to install `wasmtime`. For macOS, you can install with `brew install wasmtime`. For Linux, you can install with `sudo apt-get install wasmtime`.

To run unit tests, use `make test`.

To run the end-to-end JavaScript tests, use `make e2e`. 
