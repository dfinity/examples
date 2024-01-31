set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}
# create new demo identities
dfx identity new user1 --disable-encryption || true
dfx identity use user1
export USER1_PRINCIPAL=$(dfx identity get-principal)
export USER1_ACC=$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$(dfx ledger account-id)'")]) + "}")')
dfx identity new user2 --disable-encryption || true
dfx identity use user2
export USER2_PRINCIPAL=$(dfx identity get-principal)
export USER2_ACC=$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$(dfx ledger account-id)'")]) + "}")')
# transfer dip tokens to user
dfx identity use default
dfx canister call defi_dapp clear
dfx canister call AkitaDIP20 transfer  '(principal '\"$USER1_PRINCIPAL\"',10000000)'
dfx canister call AkitaDIP20 transfer  '(principal '\"$USER2_PRINCIPAL\"',10000000)'
dfx canister call GoldenDIP20 transfer  '(principal '\"$USER1_PRINCIPAL\"',10000000)'
dfx canister call GoldenDIP20 transfer  '(principal '\"$USER2_PRINCIPAL\"',10000000)'
# transfer ICP tokens to users
dfx canister call ledger transfer "(record { amount = record { e8s = 100000 }; to = $USER1_ACC; fee = record { e8s = 10000}; memo = 1;})"
dfx canister call ledger transfer "(record { amount = record { e8s = 100000 }; to = $USER2_ACC; fee = record { e8s = 10000}; memo = 1;})"
# get canister IDs
export DEX_PRINCIPLE=$(dfx canister id defi_dapp)
export AKITA_ID=$(dfx canister id AkitaDIP20)
export GOLDEN_ID=$(dfx canister id GoldenDIP20)
export LEDGER_ID=$(dfx canister id ledger)
# setup DIP20 balances on DEX for user1
dfx identity use user1
dfx canister call AkitaDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',1000000)'
dfx canister call GoldenDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',1000000)'
dfx canister call defi_dapp deposit '(principal '\"$AKITA_ID\"')'
dfx canister call defi_dapp deposit '(principal '\"$GOLDEN_ID\"')'
# setup ICP balances on DEX for user1
export ICP_DEPOSIT_ADDR_USER1=$(dfx canister call defi_dapp getDepositAddress | tr -d '\n' | sed 's/,)/)/')
dfx canister call ledger transfer "(record { amount = record { e8s = 50000 }; to = $ICP_DEPOSIT_ADDR_USER1; fee = record { e8s = 10000}; memo = 1;})"
dfx canister call defi_dapp deposit '(principal '\"$LEDGER_ID\"')'
# show balances user1
dfx canister call defi_dapp getBalances
# setup DIP20 balances on DEX for user2
dfx identity use user2
dfx canister call AkitaDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',1000000)'
dfx canister call GoldenDIP20 approve  '(principal '\"$DEX_PRINCIPLE\"',1000000)'
dfx canister call defi_dapp deposit '(principal '\"$AKITA_ID\"')'
dfx canister call defi_dapp deposit '(principal '\"$GOLDEN_ID\"')'
# setup ICP balances on DEX for user2
export ICP_DEPOSIT_ADDR_USER2=$(dfx canister call defi_dapp getDepositAddress | tr -d '\n' | sed 's/,)/)/')
dfx canister call ledger transfer "(record { amount = record { e8s = 50000 }; to = $ICP_DEPOSIT_ADDR_USER2; fee = record { e8s = 10000}; memo = 1;})"
dfx canister call defi_dapp deposit '(principal '\"$LEDGER_ID\"')'
# show balances user2
dfx canister call defi_dapp getBalances
# user 1 sell 3 ICP for 200 Golden to user2
dfx identity use user1
dfx canister call defi_dapp placeOrder '(principal '\"$LEDGER_ID\"', 3, principal '\"$GOLDEN_ID\"', 200)'
dfx identity use user2
dfx canister call defi_dapp placeOrder '(principal '\"$GOLDEN_ID\"', 200, principal '\"$LEDGER_ID\"', 3)'
dfx canister call defi_dapp getBalances
dfx canister call defi_dapp getBalances | grep 999_800
dfx identity use user1
dfx canister call defi_dapp getBalances
dfx canister call defi_dapp getBalances | grep 39_997
dfx identity use default
echo "PASS"
