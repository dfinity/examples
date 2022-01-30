#!/bin/bash
echo "~~~ Running DeFi test script ~~~"

# Canister principals
DEX=$(dfx canister id defi_dapp)
ICP=$(dfx canister id ledger)
AKI=$(dfx canister id AkitaDIP20)
GLD=$(dfx canister id GoldenDIP20)

#dfx canister call defi_dapp placeOrder '(principal '\"$(dfx canister id ledger)\"', 10, principal '\"$(dfx canister id GoldenDIP20)\"', 130)'
#dfx canister call defi_dapp placeOrder '(principal '\"$(dfx canister id ledger)\"', 10, principal '\"$(dfx canister id AkitaDIP20)\"', 130)'

function place_order {
  dfx canister call defi_dapp placeOrder '(principal '\"$1\"', '$2', principal '\"$3\"', '$4')'
}

#echo "Cancelling all standing orders..."
#dfx canister call defi_dapp cancel_all_orders
# => re-install instead: dfx canister install defi_dapp --mode reinstall

# Fill book with orders
# Bid
place_order $ICP 3000 $GLD 10000
place_order $ICP 2000 $GLD 10000
place_order $ICP 1000 $GLD 10000
# Ask
place_order $GLD 10000 $ICP 4000
place_order $GLD 10000 $ICP 5000
place_order $GLD 10000 $ICP 6000

# trigger transaction
place_order $ICP 2000 $GLD 2500

# partial order execution while selling (filling two orders and half of the last one).
place_order $GLD 25000 $ICP 2500

# partial order execution while buying
place_order $ICP 12000 $GLD 20000
