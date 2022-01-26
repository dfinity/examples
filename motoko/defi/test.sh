#!/bin/bash
echo "~~~ Running DeFi test script ~~~"

# Canister principals
ICP=$(dfx canister id ledger)
AKI=$(dfx canister id AkitaDIP20)
GLD=$(dfx canister id GoldenDIP20)

#dfx canister install defi_dapp --mode reinstall
#dfx canister call defi_dapp place_order '(principal '\"$(dfx canister id ledger)\"', 10, principal '\"$(dfx canister id GoldenDIP20)\"', 120)'
#dfx canister call defi_dapp place_order '(principal '\"$(dfx canister id ledger)\"', 10, principal '\"$(dfx canister id GoldenDIP20)\"', 130)'
#dfx canister call defi_dapp place_order '(principal '\"$(dfx canister id ledger)\"', 10, principal '\"$(dfx canister id GoldenDIP20)\"', 140)'

function place_order {
  dfx canister call defi_dapp place_order '(principal '\"$1\"', '$2', principal '\"$3\"', '$4')'
}

#Fill book with orders
place_order $ICP 10 $GLD 140
place_order $ICP 10 $GLD 130
place_order $ICP 10 $GLD 120

place_order $GLD 150 $ICP 10
place_order $GLD 160 $ICP 10
place_order $GLD 170 $ICP 10

