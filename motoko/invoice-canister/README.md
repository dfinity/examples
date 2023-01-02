# Invoice Canister

This project provides a simple interface for creating and paying invoices in various tokens on the Internet Computer. It is a custodial solution, intended to be a simple, drop-in payments solution for any canister. To read more about the design of the canister, see the [Design Doc](./docs/DesignDoc.md).

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app:
* [Inter-Canister Calls and Rollbacks](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices/#inter-canister-calls-and-rollbacks), since issues around inter-canister calls can e.g. lead to time-of-check time-of-use or double spending security bugs.
* [Certify query responses if they are relevant for security](https://internetcomputer.org/docs/current/references/security/general-security-best-practices#certify-query-responses-if-they-are-relevant-for-security), since this is essential when e.g. displaying important financial data in the frontend that may be used by users to decide on future transactions.
* [Use a decentralized governance system like SNS to make a canister have a decentralized controller](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#use-a-decentralized-governance-system-like-sns-to-make-a-canister-have-a-decentralized-controller), since decentralizing control is a fundamental aspect of decentralized finance applications like the invoice canister.

## Integrating with the Invoice Canister

To simply add the Invoice Canister to your project, copy the source code from the `src/invoice` directory to your project. For the sake of this example, we'll say the directory you place them in is also `src/invoice`. 

Then, add the following to your `dfx.json`:

```diff
"canisters": [
    // ...
+    "invoice": {
+        "main": "src/invoice/main.mo",
+        "type": "motoko"
+    },
]
```

At this stage, the invoice canister processes invoices with ICP transactions which requires an ICP ledger. You can download and deploy a local ICP ledger following [Ledger Local Setup](https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/ledger-local-setup/) instructions from the Developer Docs or you can alternatively use the `dfx nns` command. 

This repository and the rest of this guide uses the `dfx nns` command to locally install an ICP ledger. 

In your project's root directory, run `dfx nns install` to automatically download and deploy a local ICP ledger. When that is complete run `dfx nns import` which will add the automatically configured declaration for this installed ledger named as `nns-ledger`. Note running `dfx nns import` will also add declarations for the other nns related canisters, which you can remove from `dfx.json` if you are not using them in your project. 

Then edit `dfx.json` to add the `nns-ledger` as a dependency to the invoice canister's declaration: 

```diff
"canisters": [
    // ... 
    // running dfx nns import will automatically add the following declaration 
    "nns-ledger": {
        "build": "",
        "candid": "candid/nns-ledger.did",
        "remote": {
            "id": {
                "ic": "ryjl3-tyaaa-aaaaa-aaaba-cai",
                "local": "ryjl3-tyaaa-aaaaa-aaaba-cai"
            }
        },
        "type": "custom",
        "wasm": ""
    },
    "invoice": {
+        "dependencies": [
+            "nns-ledger"
+        ],
        "main": "src/invoice/main.mo",
        "type": "motoko"
    },
]
```

The last step is checking if your system wide dfx networks configuration file is set correctly.

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

If not, modify it so it does as this the network configuration required by `dfx nns` installed canisters. More details about using the `dfx nns` command can be found [here](https://github.com/dfinity/sdk/blob/master/docs/cli-reference/dfx-nns.md). 

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

Additionally invoices must be created with a minimum billable amount due that is currently set as twice the transfer fee (for ICP transactions this equals 20,000 e8s). This is to at least be able to cover the cost of transferring the proceeds of a paid invoice, when successfully verified, from that invoice's subaccount to the subaccount associated with that invoice creator's principal. 

For security, the canister will only allow the invoice creator to read the status of an invoice or to verify it. If your flow requires a different principal, of say the customer, to make those requests, you can specify that in the `Permissions` configuration at the time the invoice is created.

## Getting Started - Development

Make sure you have followed the DFX installation instructions from https://smartcontracts.org.

This project uses the `dfx nns` command to install a local ICP ledger so verify your `networks.json` is configured accordingly. 

Run `node install-local.mjs` which uses the `zx` library to run a Javascript based bash script to install the ICP ledger and and the invoice canister on your device. You can make calls using the `dfx` sdk, or you can see test cases running through the flows under the `test` directory.

## Testing

To test, you will need to install `moc` from the latest `motoko-<system>-<version>.tar.gz` release. https://github.com/dfinity/motoko/releases.

Then, install Vessel following the guide at https://github.com/dfinity/vessel.

You will also need to install `wasmtime`. For macOS, you can install with `brew install wasmtime`. For Linux, you can install with `sudo apt-get install wasmtime`.

To run unit tests, use `make test`.

To run the end-to-end JavaScript tests, use `make e2e`. 

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
