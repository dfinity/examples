# Invoice Canister

This project provides a simple interface for creating and paying invoices in various tokens on the Internet Computer. It is a custodial solution, intended to be a simple, drop-in payments solution for any canister. To read more about the design of the canister, see the [Design Doc](./docs/DesignDoc.md).

## Integrating with the Invoice Canister

To simply add the Invoice Canister to your project, copy the source code from the `src/invoice` directory to your project. For the sake of this example, we'll say the directory you place them in is also `src/invoice`. Do the same with `src/ledger`.

Then, add the following to your `dfx.json`:

```json
"canisters": [
    // ...
    "ledger": {
        "type": "custom",
        "candid": "src/ledger/ledger.did",
        "wasm": "src/ledger/ledger.wasm",
        "remote": {
            "candid": "src/ledger/ledger.did",
            "id": {
                "ic": "ryjl3-tyaaa-aaaaa-aaaba-cai"
            }
        }
    },
    "invoice": {
        "dependencies": [
            "ledger"
        ],
        "main": "src/invoice/main.mo",
        "type": "motoko"
    },
]
```

If you have a canister that will make calls to the invoice canister, you can now add it as a dependency and make calls to it from your own canister. The typical payment workflow will be as follows:

- create an invoice (`create_invoice`)
- make a payment to the invoice destination
- verify invoice (`verify_invoice`)

At this stage, the funds will be consolidated in the invoice creator's account. You can verify those funds in via

`get_balance` -> returns the balance in e8's

and you can transfer those funds to another account via `transfer`.

Once an invoice has been verified, you can look it up again anytime to check the status and the amount paid.

## Constraints
In order to keep the size of the state predictable we set constraints on the size of various fields in invoice creation arguments. The following table lists the constraints.

| Field         | Max Size |
|---------------|----------|
| `Meta`        | 32_000   |
| `description` | 256      |
| `canGet`      | 256      |
| `canVerify`   | 256      |

Given these constraints, we can estimate that the canister can safely hold 30,000 invoices. This is a reasonable upper bound for the number of invoices that can be stored in the canister until we add in the ability to archive and scale the provider automatically.


For security, the canister will only allow the invoice creator to read the status of an invoice or to verify it. If your flow requires a different principal, of say the customer, to make those requests, you can specify that in the `Permissions` configuration at the time the invoice is created.

## Getting Started - Development

Make sure you have followed the DFX installation instructions from https://smartcontracts.org.

Run the `install-local.sh` script to install the ICP ledger and and the invoice canister on your device. You can make calls using the `dfx` sdk, or you can see test cases running through the flows under the `test` directory.

## Testing

To test, you will need to install `moc` from the latest `motoko-<system>-<version>.tar.gz` release. https://github.com/dfinity/motoko/releases.

Then, install Vessel following the guide at https://github.com/dfinity/vessel.

You will also need to install `wasmtime`. For macOS, you can install with `brew install wasmtime`. For Linux, you can install with `sudo apt-get install wasmtime`.

To run unit tests, use `make test`.

To run the end-to-end JavaScript tests, first install fresh with with `./install-local.sh`. Then, run `npm test`.

## Caveats

There are several issues you may want to consider when using the invoice canister.

1. The controller of the canister can claim funds held by any account. 
  * mitigation - you can blackhole the invoice canister or use an allowlist to prevent unauthorized accounts from creating invoices and discourage users from holding a balance in the canister.
2. Funds can get stuck in invoice accounts
  * If someone continues to transfer funds into a ledger subaccount for an invoice after the invoice has been verified, the funds will get stuck in the invoice account.
  * mitigation - a new method may be added to the canister to allow the invoice creator to sweep the funds.
3. Uncertified queries
  * There is some degree of risk in allowing the `get_invoice`, `get_account_identifier`, and `get_balance` queries to be left as queries
  * mitigation - you can remove the query keyword from the canister
