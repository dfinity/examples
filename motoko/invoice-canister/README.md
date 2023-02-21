# Invoice Canister

This project provides an interface for creating invoices to process payments in either ICP or ICRC1 based tokens on the Internet Computer. It is a custodial solution, intended to provide a robust point of departure for integrating payment flow of these and other tokens in an Internet Computer canister. 

The main project demonstrates support of four different tokens, two of which use the types of the ICP standard and two of which use the types of the ICRC1 standard. A simpler version of the same interface integrating support for only two tokens, one ICP and the other ICRC1, can be found in the [examples/motoko-seller-client](./examples/motoko-seller-client) subdirectory. That project uses two class based mock ledgers instead of the four deployed token-ledger canister this main project uses. 


## API Overview

Both projects share the same Invoice Canister API: 

```
add_allowed_creator()
remove_allowed_creator()
get_allowed_creators_list()

create_invoice()
verify_invoice()
recover_invoice_subaccount_balance()

transfer()
get_caller_address()
get_caller_balance()
to_other_address_format() 
```

Which allows authorized callers to create and verify invoices whose payments are processed by the Invoice Canister until the proceeds of those invoices successfully paid are transferred out of its custody. Specifically, the Invoice Canister creates a new payment address (that is, a subaccount of that deployed invoice canister) for each invoice created so that a buyer can complete purchase by sending the required amount to that address. Then an authorized call to `verify_invoice` will trigger the Invoice Canister to confirm that payment address's balance is equal or greater to that invoice's amount due, and if successfully confirmed transfer those proceeds to the address that is created (also a subaccount of that deployed invoice canister) for that invoice's creator. At this point the invoice creator can then transfer that amount out of the Invoice Canister's custody to a desired destination. 

Currently the only caller with authorization to add or remove principals to or from the allowed creators list is the caller who originally deployed the invoice canister, that is its installer. Other than this access of permission, the installer has no special privelege as far as what's coded into the Invoice Canister; in other words, other than the three API methods at the top of the above list, any calls made by the installer to any of the remaining API methods are equivalent to the installer being just another principal on the allowed creators list. This **does not** prevent an installer or canister controller from changing the code and transferring any funds held, but it is not possible for the installer to arbitrarily transfer any of the funds in the Invoice Canister's custody through any of the existing API methods. 

All allowed creators also have the same permission to add access control per invoice, by optionally including when creating an invoice two lists of principals that determine who can get or verify that invoice. Though it does not matter who verifies an invoice, if successfully verified as paid its proceeds are always sent to the address created for that invoice's creator. 

Any caller authorized to verify an invoice can also call on the Invoice Canister to recover the balance of an invoice's payment address. This should not be regarded as a refund mechanism, as invoices that are successfully verified as paid have their proceeds sent to the creator's address as part of the verification process. However if only partial payment has been made, or additional payment is sent after the invoice has already been verified, then it is possible to recover that balance; in either case the full amount of that invoice's payment address balance is transferred to the given destination. 

An invoice creator can call `get_caller_balance` to view the current balance of any proceeds that have not yet been transferred out. Calling `get_caller_address` will return the address associated with this balance. Calling `to_other_address_format` will return both the canister expected type and the text encoded form of an address or text given, or the default subaccount of the principal for the token type given. 

_This is only a summary description of the general functionality of the Invoice Canister, review the [Design Doc](./docs/DesignDoc.md) for more details in particular being aware of the security concerns such as regarding an invoice's data privacy. Additionally, there is extensive commentary in the [Invoice.mo](./src/invoice/Invoice.mo), [Types.mo](./src/invoice/modules/Types.mo) and [SupportedToken.mo](./src/invoice/modules/SupportedToken.mo) files._
## Getting Started - Development

As support for four tokens requires four token-ledger canisters, three of these are installed by the downloaded wasm and did files provided by the [Dfinity Rosetta-API repository](https://github.com/dfinity/ic/tree/master/rs/rosetta-api). These files can be found in the [src/token-ledger-canisters](./src//token-ledger-canisters/) directory with an accompanying shell script for downloading them independently of this project. The ICRC1 token-ledger canister wasm and did is deployed twice, once for each of the two ICRC1 tokens integrated. The ICP ledger wasm and did is only deployed once as the other ICP based token has its ledger canister deployed by running the `dfx nns install` command. This to demonstrate the multiple ways of integrating token-ledger canisters.

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
If it does not, make a backup of the original file. Once the original has been backed up or if the `networks.json` file does not exist, use a text editor or `cat` command to set the `networks.json` to match the above. More details about using the `dfx nns` command can be found [here](https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md). 

Once completed, this project's startup script can be run. This script uses the [zx](https://github.com/google/zx) command line scripting library to start up a local replica correctly configured with the four token-ledger canisters this project uses. This is the [clean-startup.mjs](./clean-startup.mjs) which contains documentation explaining how it restarts dfx, runs `dfx nns install`, adds an identity used for testing if needed, deploys the invoice and other token-ledger canisters, and finally, if testing, disbursing funds to that identity used in the E2E testing.

For convenience, two npm scripts have been added to initiate this script:  

`npm run deployAll`  
`npm run deployForTesting`  

If deployed for testing, the Secp256k1 identity the `dfx nns install` uses as one its two initial deposit identities is added and switched to as the current user. Whether testing or not, the current identity is used as the minting account for all four token-ledger canisters. See the [clean-startup.mjs](./clean-startup.mjs) for more details. 

This script will check if the system wide networks configuration file is correctly set before running. If correctly set, either of the two above commands can be used to start a local replica with all the deployed canisters ready. Note the command line arguments used with `dfx` in this script are first made a variable that can be logged to the console to be manually used as a `dfx` command and modified with custom fields if need be.  

_For more details be sure to check out the introductory comment of [clean-startup.mjs](./clean-startup.mjs)._ 
## Integrating the Invoice Canister

To integrate the invoice canister in another project, review the [Design Doc](./docs/DesignDoc.md), [Invoice.mo](./src/invoice/Invoice.mo) and [SupportedToken.mo](./src/invoice/modules/SupportedToken.mo). Determine which tokens are to be supported, as configuring a single invoice canister to support adding new ICRC1 tokens after it has already been deployed requires extra initial configuration of the variant's references used to map the supported tokens. See "Future Proofing" in the [Design Doc](./docs/DesignDoc.md) for more details. Both the `Invoice.mo` and `SupportedToken.mo` files will need to be edited to reflect support for the tokens to be supported as outlined in the [Design Doc](./docs/DesignDoc.md) and explained [SupportedToken.mo](./src/invoice/modules/SupportedToken.mo). 

To summarize, an additional `SupportedToken` variant tag is needed for each token to be supported and their corresponding cases in each of the switches the `SupportedToken` variant is used must be updated to include that added variant tag: these switches are only found in the methods at the file scope of `SupportedToken.mo` and in the API methods of `Invoice.mo`. The API methods involve the actual calls that are made to the corresponding `ICP` ledger and `ICRC1` token-ledgers, which can instantiated with their canister id using the `ICP` and `ICRC1` supertype actors in `SupportedToken.mo`. An easy way to see where all of the edits are needed is modify the original `SupportedToken` variant (line 529 in `SupportedToken.mo`) to add or remove a tag, and see where the VSCode Motoko extension indicates the impacted switches are. 

While updating the `SupportedToken` variant's references, also get the canister ids of the token-ledger canisters to be supported and set them as the canister ids used to instantiate the supertype actors representing these token-ledger canisters in `Invoice.mo`. Once `Invoice.mo` and `SupportedToken.mo` are finished being updated, add them with `Types.mo` to the project. The last two steps are updating `dfx.json` and adding the invoice canister to the canister in the project using it. Update `dfx.json` adding the `Invoice.mo` as the file reference of the Motoko canister to deploy as the invoice canister--it **does not need to be added as a dependency** to the other canister's entry, because as the invoice canister is now a class actor, it can only be imported by instantiation with its canister id. See how the `Invoice.mo` is "imported" in the [motoko-seller-client Seller canister](./examples/motoko-seller-client/src/backend/Seller.mo) for a concrete example of how to do this.

As stated earlier, the `motoko-seller-client` project is an example of integration with only two tokens, one for ICP and one for ICRC1 mapped to the variant tags `#ICP` and `#ICRC1` respectively, along with their two corresponding [class based mock ledgers](./examples/motoko-seller-client/src/backend/modules/MockTokenLedgerCanisters.mo) that can be used to develop more quickly returning all the same `Ok` and `Err` results, except for the two of the ICRC1 specification `#Generic Error` and `#TemporarilyUnavailable`. 

## Testing

To test, you will need to install `moc` from the latest `motoko-<system>-<version>.tar.gz` release. https://github.com/dfinity/motoko/releases.

Then, install Vessel following the guide at https://github.com/dfinity/vessel.

You will also need to install `wasmtime`. For macOS, you can install with `brew install wasmtime`. For Linux, you can install with `sudo apt-get install wasmtime`.

To run unit tests, use `make test`.

To run the end-to-end JavaScript tests, use `make e2e`. 

## Security Concerns

As stated earlier, there are a few points that must be considered when deploying the Invoice Canister in production. Unless further modification of the code base is performed to prevent or otherwise manage these, these are:

1) Funds held by the Invoice Canister are subject to the control of the installer and/or its current specified controller(s) and may be lost or otherwise unrecoverable. 
2) Details of stored invoices are not encrypted by default and could be physically inspected by a node provider. 
3) While measures have been implemented to reliably process transactions, there are certain conditions such as a malevolent token ledger-canister intentionally endlessly looping a call instead of returning or the Invoice Canister's message queue reaching capacity, which cannot always be anticipated and may effect it's expected operation. Particularly when deploying to mainnet, using a dedicated logger is strongly encouraged. 

_See the [Design Doc](./docs/DesignDoc.md) and [Security Best Practices] https://internetcomputer.org/docs/current/developer-docs/security/ for more details._
