# Example Seller Flow

This is an example of a seller flow, integrating with an invoice canister in Motoko. In this example, you can use the provided invoice canister mock to test your canister that integrates with the Invoice Canister.

To install: 

`dfx deploy`

To run:
Open up Candid UI by running
```
open http://localhost:8000\?canisterId\=$(dfx canister id __Candid_UI)
```

Enter in the canister id for Seller (`dfx canister id seller`) and click on the `Go` button.

Do the same thing in another tab and enter in the canister id for Invoice Canister (`dfx canister id invoice`) and click on the `Go` button.

## Example workflow

* Create an invoice to buy a credential from the seller
* Use the `deposit_free_money` from the mock invoice cansiter to pay the invoice `destination` account
* Return to the seller, and verify the invoice is paid
* Check to see that your credential has been awarded
