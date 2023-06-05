# Invoice canister

## Overview

This project provides an interface for creating invoices to process payments in either ICP or ICRC1 based tokens on the Internet Computer. It is a custodial solution, intended to provide a robust point of departure for integrating payment flow of these and other tokens in an Internet Computer canister. 

The main project demonstrates support of four different tokens, two of which use the types of the ICP standard and two of which use the types of the ICRC1 standard. A simpler version of the same interface integrating support for only two tokens, one ICP and the other ICRC1, can be found in the [examples/motoko-seller-client] subdirectory. 

That example project uses two class based mock ledgers instead of the four deployed token-ledger canisters this main project uses as well as featuring the deployed invoice canister functioning in another canister to process purchases.  Additionally, all the distinct module files associated with the `SupportedToken` that are found in the main project's `/src/invoice/modules/supported-token/` directory are compacted into a single `SupportedToken.mo` module file, and the example project's code base has in-body comments omitted. Otherwise the codebase is the same. 

Be aware mainnet ICP Ledger now supports the ICRC1 standard, and if deploying an invoice canister integrating with the mainnet ICP Ledger that token can be done with either the generic ICP or ICRC1 type of `SupportedToken.mo`. However it is advised to use ICRC1 as the ICRC1 standard is the basis for future tokenization standards on the Internet Computer. This project includes both ICP and ICRC1 for demonstration and reference purposes. 

## API Overview

Both projects share the same invoice canister API: 

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

This API allows authorized callers to create and verify invoices whose payments are processed by the invoice canister until the proceeds of those invoices successfully paid are transferred out of its custody. Specifically, the invoice canister creates a new payment address (that is, a subaccount of that deployed invoice canister) for each invoice created so that a buyer can complete the purchase by sending the required amount to that address. Then an authorized call to `verify_invoice()` will trigger the invoice canister to confirm that payment address's balance is equal or greater to that invoice's amount due, and if successfully confirmed transfer those proceeds to the address that is created (also a subaccount of that deployed invoice canister) for that invoice's creator. At this point the invoice creator can then transfer that amount out of the invoice canister's custody to a desired destination. 

Currently the only caller with authorization to add or remove principals to or from the allowed creators list is the caller who originally deployed the invoice canister, that is its installer. Other than this access of permission, the installer has no special privilege as far as what's coded into the invoice canister; in other words, other than the three API methods at the top of the above list, any calls made by the installer to any of the remaining API methods are equivalent to the installer being just another principal on the allowed creators list. This **does not** prevent an installer or canister controller from changing the code and transferring any funds held, but it is not possible for the installer to arbitrarily transfer any of the funds in the invoice canister's custody through any of the existing API methods. 

All allowed creators also have the same permission to add access control per invoice, by optionally including when creating an invoice two lists of principals that determine who can get or verify that invoice. Though it does not matter who verifies an invoice, if successfully verified as paid its proceeds are always sent to the address created for that invoice's creator. 

Any caller authorized to verify an invoice can also call on the invoice canister to recover the balance of an invoice's payment address. This should not be regarded as a refund mechanism, as invoices that are successfully verified as paid have their proceeds sent to the creator's address as part of the verification process. However if only partial payment has been made, or additional payment is sent after the invoice has already been verified, then it is possible to recover that balance; in either case the full amount of that invoice's payment address balance is transferred to the given destination. 

An invoice creator can call `get_caller_balance()` to view the current balance of any proceeds that have not yet been transferred out. Calling `get_caller_address()` will return the address associated with this balance. Calling `to_other_address_format()` will return both the canister expected type and the text encoded form of an address or text given, or the default subaccount of the principal for the token type given. 

This is only a summary description of the general functionality of the Invoice Canister, review the [design doc](https://github.com/dfinity/examples/blob/master/motoko/invoice-canister/docs/DesignDoc.md) for more details in particular being aware of the security concerns such as regarding an invoice's data privacy. Additionally, there is extensive commentary in this [invoice.did](https://github.com/dfinity/examples/blob/master/motoko/invoice-canister/invoice.did), or the [Invoice.mo](https://github.com/dfinity/examples/blob/master/motoko/invoice-canister/src/invoice/Invoice.mo), [Types.mo](https://github.com/dfinity/examples/blob/master/motoko/invoice-canister/src/invoice/modules/Types.mo) and the associated modules of the [SupportedToken.mo](https://github.com/dfinity/examples/blob/master/motoko/invoice-canister/src/invoice/modules/supported-token/SupportedToken.mo) files.


## Tutorial

## Prerequisites
This example requires an installation of:

- [x] Install the [IC SDK](https://internetcomputer.org/docs/current/developer-docs/setup/install/index.mdx).

Begin by opening a terminal window.

### Step 1: Navigate into the folder containing the project's files install the required packages:

`cd examples/motoko/invoice-canister`
`npm install`

At this point the project is ready to be deployed to a local replica. As support for four tokens requires four token-ledger canisters, three of these are installed by the downloaded Wasm and did files provided by the [DFINITY Rosetta-API repository](https://github.com/dfinity/ic/tree/master/rs/rosetta-api). These files can be found in the `src/token-ledger-canisters` directory with an accompanying shell script for downloading them independently of this project. The ICRC1 token-ledger canister Wasm and did is deployed twice, once for each of the two ICRC1 tokens integrated. The ICP ledger wasm and did is only deployed once as the other ICP based token has its ledger canister deployed by running the `dfx nns install` command. This to demonstrate the multiple ways of integrating token-ledger canisters into your own projects.

### Step 2: Before running this project, it is necessary to check the system wide network configuration is set according to what the canisters installed by `dfx nns install` require.

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

If it does not, make a backup of the original file. Once the original has been backed up or if the `networks.json` file does not exist, use a text editor such as [nano](https://www.nano-editor.org/download.php) or the `cat` command to set the `networks.json` to match the above. More details about using the `dfx nns` command can be found [here](https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md). Note that when using `dfx nns install` for the first time, `dfx nns import` is also used afterward to correctly install the associated did files and configure `dfx.json`. As this project has already been configured, this step is skipped in the startup script. 

Once the system-wide `networks.json` is set as above, this project's startup script can be run. This script uses the [zx](https://github.com/google/zx) command line scripting library to start up a local replica correctly configured with the four token-ledger canisters this project uses. This is the [clean-startup.mjs](./clean-startup.mjs) which contains documentation explaining how it restarts dfx, runs `dfx nns install`, adds an identity used for testing if needed, deploys the invoice and other token-ledger canisters, and finally, if testing, disbursing funds to that identity used in the E2E testing.

### Step 3: For convenience, two npm scripts have been added to initiate this script:  

`npm run deployAll`  
`npm run deployForTesting` 

If deployed for testing, the Secp256k1 identity the `dfx nns install` command uses as one of its two initial deposit identities is added and switched to as the current user. Whether testing or not, the current identity is used as the minting account for all four token-ledger canisters. See the `clean-startup.mjs` for more details. To see an example of the console output of running this script for testing, the `./docs/clean-startup-console-output` file that can be reviewed in the docs folder. 

This script will check if the system wide networks configuration file is correctly set before running. If correctly set, either of the two above commands can be used to start a local replica with all the deployed canisters ready. Note the command line arguments used with `dfx` in this script are first made a variable that can be logged to the console to be manually used as a `dfx` command and modified with custom values if need be.  

For more details be sure to check out the introductory comment of [clean-startup.mjs](./clean-startup.mjs) or review the [clean-startup-console-output](./docs/clean-startup-console-output.md) example.

### Step 4: Integrating the invoice canister

To integrate the invoice canister in another project, review the [design doc](./docs/DesignDoc.md), [Invoice.mo](./src/invoice/Invoice.mo) and [SupportedToken.mo](./src/invoice/modules/supported-token/SupportedToken.mo). 

To summarize, both the `Invoice.mo` and `SupportedToken.mo` files need to be edited according to which tokens are to be supported. While this project uses four tokens, it may be easier to start from the `motoko-seller-client` example as only two tokens (one for ICP and one for ICRC1) are integrated in that project. To add support for an additional token, it can take as little as 15 minutes using this project's code as a reference to copy.

### Step 5: The preliminary step is to decide which tokens are to be supported by the operations of the invoice canister. 

For each token, choose the label or text to represent the token as a new tag in the `SupportedToken` variant found in [SupportedToken.mo](./src/invoice/modules/supported-token/SupportedToken.mo) at line 49. For example to add support for the ICRC1 ckTESTBTC token: 

```diff
public type SupportedToken<T1, T2> = {
  // Other supported token tag entries.
  // T1 is used for any tokens using the ICP specification, while T2 for any of the ICRC1 specification.
  // So adding a tag for ICRC1 ckTESTBTC token would look like:
+  #ICRC1_ckTESTBTC : T2; 
};
```

Note that the generic type `T1` is used for ICP (or any token using the associated types of the ICP specification), while `T2` is used for any token that is based on the ICRC1 standard. Each token to be supported will need its own tag entry, and here the tag added for ckTESTBTC is prefixed with "ICRC1_" to keep things clearer. 

Observe also that once this variant's declaration is modified by adding (or removing) a tag, the Motoko VSCode extension will automatically indicate all the other places in the code that need to be edited (or if not using VSCode the `moc` compiler warnings). This consists of all the switches that will need an additional case for the tag that is added. These switches are only found in the methods of `SupportedToken.mo` and in some of the API methods of `Invoice.mo` (all relevant methods are listed here). 

The methods to update in `SupportedToken.mo` are: 
``` 
getTokenVerbose()
unwrapTokenAmount()
wrapAsTokenAmount()  
encodeAddress()  
encodeAddressOrUnitErr()  
getAddressOrUnitErr()  
getInvoiceSubaccountAddress()  
getEncodedInvoiceSubaccountAddress()  
getCreatorSubaccountAddress()  
getTransferArgsFromInvoiceSubaccount()  
getTransferArgsFromCreatorSubaccount()  
rewrapTransferResults()  
getDefaultSubaccountAddress() 
```

### Step 6: For all these methods except `getTokenVerbose()`, adding the additional case in its switch can be done by simply copying the example given in this project's implementation corresponding to whether the token to support is ICP or ICRC1. 

Be sure to update each reference of the tag. For the `#ICRC1_ckTESTBTC` tag that was added above, updating the switch in `getEncodedInvoiceSubaccountAddress()` would look like:

```diff
public func getEncodedInvoiceSubaccountAddress({
  token : UnitType;
  id : Text;
  creator : Principal;
  canisterId : Principal;
}) : Text {
  switch token {
    // Other cases.
+   case (#ICRC1_ckTESTBTC) {
+      ICRC1_Adapter.encodeAddress(ICRC1_Adapter.computeInvoiceSubaccountAddress(id, creator, canisterId));
+    };
  };
};
```

or for `rewrapTransferResults()`:

```diff
public func rewrapTransferResults(sttransferResult : TransferResult) : Result.Result<TransferSuccess, TransferErr> {
  switch (sttransferResult) {
    // Other cases.
+    case (#ICRC1_ckBTC transferResult) {
+      switch transferResult {
+        case (#Ok txIndex) #ok(#ICRC1_ckBTC(txIndex));
+        case (#Err transferErr) #err(#ICRC1_ckBTC(transferErr));
+    };
  };
};
```


Each method requires a similar simple update of its switch. As mentioned before `getTokenVerbose()` requires unique attention--in particular the correct transfer fee must be defined. As the `TokenVerbose` is part of the invoice record returned to the caller, correctly defining the other fields is also strongly encouraged (particularly the URL so further inquiry of the token-ledger canister can be easily made if needed). For the example of adding support for the ckTESTBTC token, this could look like:

```diff
public func getTokenVerbose<T1, T2>(supportedToken : SupportedToken<T1, T2>) : TokenVerbose {
  switch supportedToken {
    // Other cases.
+    case (#ICRC1_ckTESTBTC T1) {
+      return {
+        symbol = "ckTESTBTC";
+        name = "Chain key testnet Bitcoin";
+        decimals = 8 : Int;
+        fee = 10;
+        meta = ?{
+          Issuer = "NNS - For Testing Purposes Only";
+          Url = "https://dashboard.internetcomputer.org/canister/mc6ru-gyaaa-aaaar-qaaaq-cai";
+        };
+      };
+    };
  };
};
```

Note that the URL here to the dashboard for the ckTESTBTC ledger could be another URL all together; the dashboard URL is used since it is a standard and official web interface. For instance the correct values of fee, decimals, symbol or title for other ICRC1 token-ledger canisters could be found by visiting its dashboard URL. Once each of these methods has its switch updated to include the case for the tag for the token to support, this can be confirmed with the Motoko VSCode extension as it should report no warnings in any of these switches if done correctly.  

### Step 7: The next step is updating `Invoice.mo` to add the declaration for the token ledger-canister's actor type and updating the relevant API methods. 

As these API methods (except for `to_other_address_format()`) involve the actual calls that are made to the corresponding `ICP` ledger and `ICRC1` token-ledgers, their actor type can be instantiated with their canister id reusing the `ICP` and `ICRC1` actor supertype declarations found in either the ICP and ICRC1 subdirectories of [supported-token](./src/invoice/modules/supported-token/). For the ckTESTBTC token example, this would look like:

```diff
import SupportedToken "./modules/supported-token/SupportedToken";
// Could also import directly like:
// import Actor_Supertype_ICRC1 "./modules/supported-token/token-specific/icrc1/ActorSupertype.mo";

shared ({ caller = installer_ }) actor class Invoice() = this {

  // Other private state and function declarations.   

  // The canister id is being directly used in the actor's constructor, 
  // but it could be added as a separate declaration, passed as a deployment argument,
  // or even be passed in a method to be called later to dynamically create this actor type. 
+ let Ledger_ICRC1_ckTESTBTC : SupportedToken.Actor_Supertype_ICRC1 = actor ("mc6ru-gyaaa-aaaar-qaaaq-cai");

  // Rest of invoice canister's code.
};
```

### Step 8: Once the token-ledger canister's actor's declaration is added, the next step is to update the corresponding switches in the four API methods that make the actual calls to this actor's canister. 

These API methods are:

```
get_caller_balance()
transfer()
verify_invoice()
recovery_invoice_subaccount_balance()
```

It should be noted `verify_invoice()` and `recover_invoice_subaccount_balance()` each have two switch statements to be updated while `get_caller_balance()` and `transfer()` only have a single switch statement to be updated.

Like the methods from `SupportedToken`, updating each switch with an added case for the new tag can be copied from this project's code, just using the correct tag and actor declaration being used. For the example of adding support for the ckTESTBTC token, updating the `get_caller_balance()` method would look like:

```diff
public shared ({ caller }) func get_caller_balance(
  { token } : Types.GetCallerBalanceArgs,
) : async Types.GetCallerBalanceResult {
  // Beginning method code omitted.
  try {
    switch subaccountAddress {
      // Other cases.
+     case (#ICRC1_ckTESTBTC account) {
+       let balance = #ICRC1_ckTESTBTC(await Ledger_ICRC1_ckTESTBTC.icrc1_balance_of(account));
+       #ok({ balance });
+     };
    };
  } catch e {
  // Rest of method's code omitted.
};
```
Updating these methods' switch statements after adding the constructed actor type(s) declaration, **are the only** other changes needed in these two files. As mentioned, once this is done correctly it can also be confirmed with the Motoko VSCode extension (or `moc` compiler) not indicating any warnings or errors about these switches. 

At this point integrating support for an additional token is complete.

As stated earlier, the `motoko-seller-client` project is an example of integration with only two tokens, one for ICP and one for ICRC1 mapped to the variant tags `#ICP` and `#ICRC1` respectively, along with their two corresponding [class based mock ledgers](./examples/motoko-seller-client/src/backend/modules/MockTokenLedgerCanisters.mo) that can be used to develop more quickly. They should return all the same `Ok` and `Err` results, except for the two of the ICRC1 specification `#Generic Error` and `#TemporarilyUnavailable`. 

### Step 9: Once the above files are configured correctly, the next step is adding these files to the project's file structure (if this has not already been done, be sure to also include the `Types.mo` module) and updating `dfx.json` to include the invoice canister. 

Add the invoice canister to the canisters list as a Motoko type canister with its main pointing to the `Invoice.mo` file (in this example snippet named "invoice", with `Invoice.mo` in the project's `src` directory, and without a declarations output field):

```diff
{
  "canisters": {
  // Other canisters. 
+    "invoice": {
+      "main": "src/Invoice.mo",
+      "type": "motoko",
+
+    },
}
```

As the invoice canister `Invoice.mo` is now a class actor, **it is not necessary** to add its canister entry in `dfx.json` as a dependency in the other canister's entry that uses it. Instead an invoice canister actor reference can be instantiated with the actor class constructor passing in the canister id of the deployed invoice canister. The type reference of this actor class can be included in two ways.

One way is demonstrated in the [motoko-seller-client Seller canister](./examples/motoko-seller-client/src/backend/Seller.mo) which imports the type directly as the `Invoice.mo` file. The other way to do this, which can be used when creating an invoice canister actor in a separate project, is to copy the `Types.mo` file into that project and using the `InvoiceCanisterAPI` type declaration (at the bottom of `Types.mo`) as the type reference for constructing the invoice canister actor in that project's canister's code.

For example if `CanisterRequiringPaymentProcessing` was the name of a canister in another project that uses the Invoice Canister this way, after copying all the types from `Type.mo` into its `Types.mo` file and getting the deployed invoice canister id, this would look like:

```diff
// Other imports.
import Types "./Types.mo"

actor CanisterRequiringPaymentProcessing {
   // Other state, type and method declarations.
+  let invoiceCanister : Types.InvoiceCanisterAPI = fromActor("<invoice canister id>");

  // Now the API methods can be called such as
  // await invoiceCanister.create_invoice(<CreateInvoiceArgs>);
  // rest of canister's code...
};
```

### Step 10: After deploying the invoice canister, its canister id can be found with the [`dfx canister id` command](https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-canister.md#dfx-canister-id).

This completes covering all the needed steps for integrating the invoice canister in another project. While much of this was reiterating what was already written elsewhere in this project, for more details be sure to review [Design Doc](./docs/DesignDoc.md), [Invoice.mo](./src/invoice/Invoice.mo), [Types.mo](./src/modules/Types.mo) and [SupportedToken.mo](./src/invoice/modules/supported-token/SupportedToken.mo) as well as [clean-startup.mjs](./clean-startup.mjs) for further explanation of how to deploy the invoice canister and token ledger-canisters however may be needed.

## Testing

- #### Step 1: To test, you will need to install `moc` from the latest `motoko-<system>-<version>.tar.gz` release. https://github.com/dfinity/motoko/releases.

- #### Step 2: Then, install Vessel following the guide at https://github.com/dfinity/vessel.

- #### Step 3: You will also need to install `wasmtime`. For macOS, you can install with `brew install wasmtime`. For Linux, you can install with `sudo apt-get install wasmtime`.

- #### Step 4: To run unit tests, use `make test`.

- #### Step 5: To run the end-to-end JavaScript tests, use `make e2e`. 

For understanding how the Invoice Canister works, reviewing the E2E test suite [recover_invoice_subaccount_balance.test.js](./test/e2e/src/tests/recover_invoice_subaccount_balance.test.js) is useful as it demonstrates almost all the functionality of the Invoice Canister. Additionally, an example of all the unit and E2E tests' output can be found in the [Testing Glossary](./docs/TestingGlossay.md).

## Disclaimer: security considerations 

This and the `motoko-seller-client` projects are **educational examples** demonstrating how invoice based payment processing on the Internet Computer can work. They are not intended to be used in a production environment, with sensitive data or real world financial value. As stated earlier, there are known security issues that must be considered when deploying an invoice canister to mainnet:

- Funds held by the invoice canister are subject to the control of the installer and/or its current specified controller(s) and may be lost or otherwise unrecoverable.  
- Details of stored invoice records are not encrypted by default and could be physically inspected by a node provider.  
- While measures have been implemented to reliably process transactions, there are certain conditions such as an inter-canister call intentionally looping instead of returning; or the invoice canister's message queue reaching capacity while there are still incoming calls that could then be dropped, which cannot always be anticipated and may affect its expected operation.  
- This project uses a local replica configured to be a system subnet and therefore requires no cycles to process computation, which is not typical of mainnet canisters particularly those on a fiduciary subnet. When deploying an invoice canister to mainnet, keep track of its cycles balance is critical for its continued operation.
- While all the API calls have been made update to automatically return them as certified by the consensus of a subnet, if any of the three API methods that can be made query calls are converted into query calls, their results will not have this certification by default. To deliver query call results that can be certified, they must be returned as [CertifiedData](https://internetcomputer.org/docs/current/references/motoko-ref/certifieddata/).  

Additionally, the encoding and decoding of ICRC1 accounts may require being updated if the specification is finalized different than the current implementation of the [AccountTextConverter](./src/invoice/modules/supported-token/token-specific/icrc1/AccountTextConverter.mo) used for ICRC1 accounts in this project.  

When getting ready to deploy for production, thoroughly review the guides:
* [Running in production](https://internetcomputer.org/docs/current/developer-docs/production/).
* [Security best practices](https://internetcomputer.org/docs/current/developer-docs/security/).
* [How to audit an Internet Computer canister](https://www.joachim-breitner.de/blog/788-How_to_audit_an_Internet_Computer_canister).

and proceed with enough caution and preparation. See the [Design Doc](./docs/DesignDoc.md) for more details.


