# Example Seller Flow

This is an example of a seller flow integrating with an invoice canister in Motoko.  

For this example `Invoice.mo` is configured to support two tokens represented as the `#ICP` and `#ICRC1` `SupportedToken` variant tags. To perform their corresponding ledger calls, instead of requiring actual calls to be made, two mocks (from `MockTokenLedgerCanisters.mo`) are used in their place each with an additional `deposit_free_money` method that mocks minting tokens transferred to a given destination; this method is also made available in `Invoice.mo` and is used by the frontend to complete payment for an invoice (upon a user clicking the 'Confirm and Purchase' button in the `InvoicePayDialog`). As the seller needs the authorization to do this as well as create, get and verify invoices the seller canister id is imperatively set in the allowed creator's list when the invoice canister is deployed.

If your project only requires transactions involving ICP and a single ICRC1 ledger, it may be easier to use this copy of the `Invoice.mo` and it's two required modules (the supertype actors from `SupportedToken` can be used in `Invoice.mo` set as the `Ledger_ICP` and `Ledger_ICRC` fields which currently point to the mock ledgers). 

### Installing and running ###

Make sure you are in the `examples/motoko-sller-client/` directory and run the following commands:

`npm install`

To ensure the canister ids remain the same, restart dfx clean: 

`dfx stop`

`dfx start --clean --background`

`npm run deploy`

`npm run generate`

Open up Candid UI by running
```
open http://127.0.0.1:8080\?canisterId\=$(dfx canister id __Candid_UI)
```

Enter in the canister id for Seller (`dfx canister id seller`) and click on the `Go` button.

Do the same thing in another tab and enter in the canister id for Invoice Canister (`dfx canister id invoice`) and click on the `Go` button.

You can also use the links provided at the end of the `dfx deploy` output which include a link to vieiwing the front-end client in the browser.

## Frontend 

If you'd like to make further changes to the front-end you can also run:

`npm start` 

which will start a development server hosting the client available in your browser at the link in the console output (`Project is running at:`).

## Example workflow

* Create an invoice to buy a credential from the seller
* Use `deposit_free_money` from the invoice cansiter to complete payment to the invoice's `paymentAddress`
* Return to the seller and verify the invoice is paid
* Check to see that your credential has been awarded by calling the seller's `check_license_status`

