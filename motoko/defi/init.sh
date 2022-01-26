#!/bin/bash
echo "~~~ Running DeFi init script ~~~"

# Canister principals
DEX=$(dfx canister id defi_dapp)
ICP=$(dfx canister id ledger)
AKI=$(dfx canister id AkitaDIP20)
GLD=$(dfx canister id GoldenDIP20)

# Initialisation.
echo "Initialisation..."
echo " - set allowance on DIP20 tokens"
dfx canister --no-wallet call AkitaDIP20 approve  '(principal '\"$DEX\"',10000000)'
dfx canister --no-wallet call GoldenDIP20 approve '(principal '\"$DEX\"',10000000)'
echo " - get ICP deposit address"
ICP_DEPOSIT_ADDR=$(dfx canister call defi_dapp deposit_address)
echo " - deposit some ICP in the DEX"
dfx canister call ledger transfer "(record { amount = record { e8s = 1000000 }; to = $ICP_DEPOSIT_ADDR; fee = record { e8s = 10000}; memo = 1;})"
dfx canister call defi_dapp deposit '(principal '\"$AKI\"')'
dfx canister call defi_dapp deposit '(principal '\"$GLD\"')'
echo " - transfer ICP to DEX"
dfx canister call defi_dapp deposit '(principal '\"$ICP\"')'
