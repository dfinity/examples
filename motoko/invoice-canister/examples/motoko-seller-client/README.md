# Example Seller Flow

This is an example of a seller flow integrating with an invoice canister in Motoko.  

An invoice canister is used by a seller canister to manage and process the sales handled by a frontend canister. The frontend canister hosts the storefront of the seller. All three are deployed to a local replica. As the frontend canister is dependent on the declarations of the seller and invoice canisters, and the seller canister is dependent on the invoice canister, it's important that they are deployed and have declarations generated in the correct order the first time the project is run (see below). Note that the invoice canister's declarations are only required in the frontend to simulate payment with the deposit free money method, which would typically be handled independently by an interested buyer. 

In this example `Invoice.mo` is configured to support two tokens represented as the `#ICP` and `#ICRC1` `SupportedToken` variant tags. To perform their corresponding ledger calls, instead of requiring actual calls to be made, two mocks (from [MockTokenLedgerCanisters.mo](./src/backend/modules/MockTokenLedgerCanisters.mo)) are used in their place each with an additional `deposit_free_money` method that simulates minting tokens transferred to a given destination address. As just mentioned, these methods are also made accessible in `Invoice.mo` and available in the frontend so that completing payment for an invoice can be made upon a user clicking the 'Confirm and Purchase' button in the `InvoicePayDialog` UI. As the seller canister needs the authorization to do this as well as create, get and verify invoices, the seller canister id is hard-coded and imperatively set in the allowed creator's list of the invoice canister. 

Note that `SupportedToken.mo` now also contains all the associated and dependent modules used by the `SupportedToken` type in case it is easier to use a single file format when developing for a separate project. Additionally, all the in-body comments have been omitted in both this and the `Invoice.mo` files of this project. Other than the changes listed in this and the preceding paragraph, the Invoice Canister code is the same as that of the original project. If your project only requires transactions involving ICP and a single ICRC1 ledger, it may be easier to use this copy of the `Invoice.mo` and it's two required modules (the supertype actors from `SupportedToken` can be used in `Invoice.mo` set as the `Ledger_ICP` and `Ledger_ICRC` fields which currently point to the mock ledgers). 

### Installing and running ###

Make sure you are in the `examples/motoko-sller-client/` directory and run the following commands:

`npm install`

To ensure the canister ids remain the same when they are deployed in the same order, restart dfx clean: 

`dfx stop`

`dfx start --clean --background`  

First deploy the invoice canister:  

`dfx deploy invoice`  

Then deploy the seller canister:  

`dfx deploy seller`  

Now the declarations used by the frontend can be generated with the npm script (which will run `dfx generate invoice` and `dfx generate seller`):  

`npm run generate`

Lastly, the frontend can now be deployed:  

`dfx deploy frontend`  

Once this has completed, the project should be up and running. To inspect you can open up the Candid UI by running:  
```
open http://127.0.0.1:8080\?canisterId\=$(dfx canister id __Candid_UI)
```

Enter in the canister id for Seller (`dfx canister id seller`) and click on the `Go` button. Do the same thing in another tab, entering in the canister id for the Invoice Canister (`dfx canister id invoice`) and clicking on the `Go` button. You can now use the Candid UI to try out the example workflow below. You can also use the link provided at the end of the `dfx deploy frontend` command's output which links to viewing the front-end client in the browser to demonstrate the workflow from a client's perspective.  

_Note that this project is setup to use the same port (8080) as the system wide dfx `networks.json` configuration used for the main project. If using a `networks.json` with a different port, the `devServer` entry in `webpack.config.js` will need to be updated (at line 116)._  

## Frontend 

If you'd like to make further changes to the front-end you can also run:

`npm start` 

which will start a development server hosting the client available in your browser at the link in the console output (`Project is running at:`).

## Example workflow

* Create an invoice to buy a credential from the seller using either ICP or ICRC1 tokens.  
* Use `deposit_free_money` from the invoice canister to complete payment to the invoice's `paymentAddress`.  
* Return to the seller and verify the invoice is paid.  
* Check to see that your credential has been awarded by calling the seller's `check_license_status`.  

