# Tokens Transfer

Tokens Transfer shows how a canister can interact with the ledger and make transfers from its account
to other accounts.

## Security Considerations and Security Best Practices

If you base your application on this example, we recommend you familiarize yourself with and adhere to the [Security Best Practices](https://internetcomputer.org/docs/current/references/security/) for developing on the Internet Computer. This example may not implement all the best practices.

For example, the following aspects are particularly relevant for this app, since it is a finance application that makes inter-canister calls to the ledger:
* [Be aware that state may change during inter-canister calls](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#be-aware-that-state-may-change-during-inter-canister-calls)
* [Only make inter-canister calls to trustworthy canisters](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#only-make-inter-canister-calls-to-trustworthy-canisters)
* [Don’t panic after await and don’t lock shared resources across await boundaries](https://internetcomputer.org/docs/current/references/security/rust-canister-development-security-best-practices#dont-panic-after-await-and-dont-lock-shared-resources-across-await-boundaries)

## Interface

1. `transfer`: takes in input the amount of tokens to transfer, the account (and optionally the subaccount) to which to transfer the tokens and returns either success or an error in case e.g. the tokens transfer canister doesn't have enough tokens to do the transfer. In case of success, a unique identifier of the transaction is returned. This identifier will be stored in the memo of the transaction in the Ledger.


## Initialization

The canister expects three arguments:
1. `ledger_canister_id`: the canister id of the ledger canister
2. `subaccount`: the optional subaccount of the canister account from which tokens will be withdrawn
3. `transaction_fee`: a constant representing the transaction fee of the ledger


## Test Locally

1. [build and deploy the Ledger canister](https://github.com/dfinity/ic/tree/master/rs/rosetta-api/ledger_canister#deploying-locally)
2. Add some tokens to your account (`dfx identity get-principal`) in the initialization parameters of the Ledger canister.
```bash
# MINTING_ACCOUNT_ID_HEX and ACCOUNT_ID_HEX are the hex representation
# of the minting account id and your account id respectively
read -r -d '' ARGS <<EOM
(record {
     minting_account="${MINTING_ACCOUNT_ID_HEX}";
     initial_values=vec { record { "${YOUR_ACCOUNT_ID_HEX}"; record { e8s=10_000_000_000 } }; };
     send_whitelist=vec {};
 }, )
EOM
dfx deploy --argument "${ARGS}" ledger
```
3. deploy the Tokens Transfer canister locally. Point to the ledger in the initialization parameters (`dfx canister id ledger`).
```bash
LEDGER_ID="$(dfx canister id ledger)"
read -r -d '' ARGS <<EOM
(record {
  ledger_canister_id=principal "${LEDGER_ID}";
  transaction_fee=record { e8s=10_000 };
  subaccount=null
}, )
EOM
dfx deploy --argument "${ARGS}" tokens_transfer
```
4. transfer some funds to the Tokens Transfer canister
```bash
# TOKENS_TRANSFER_ACCOUNT_ID_BYTES is the vec nat8 representation of the tokens transfer canister
TOKENS_TRANSFER_ACCOUNT_ID="$(dfx ledger account-id --of-canister tokens_transfer)"
TOKENS_TRANSFER_ACCOUNT_ID_BYTES="$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$TOKENS_TRANSFER_ACCOUNT_ID'")]) + "}")')" 
read -r -d '' ARGS <<EOM
(record {
  to=${TOKENS_TRANSFER_ACCOUNT_ID_BYTES};
  amount=record { e8s=100_000 };
  fee=record { e8s=10_000 };
  memo=0:nat64;
}, )
EOM
dfx canister call ledger transfer "${ARGS}"
```
5. transfer some of the tokens from the Tokens Transfer canister back to the original account
```bash
# YOUR_PRINCIPAL is the value returned by dfx identity get-principal
read -r -d '' ARGS <<EOM
(record {
  amount=record { e8s=5 };
  to_principal=principal "${YOUR_PRINCIPAL}"
},)
EOM
dfx canister call tokens_transfer transfer "${ARGS}"
```
