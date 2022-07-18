set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}
dfx identity use default
export AkitaDIP20=$(dfx canister id AkitaDIP20)
echo AkitaDIP20 "${AkitaDIP20}"
export GoldenDIP20=$(dfx canister id GoldenDIP20)
echo GoldenDIP20 "${GoldenDIP20}"
dfx canister call defi_dapp clear
dfx identity new user1 --disable-encryption || true
dfx identity new user2 --disable-encryption || true
dfx identity use user1
dfx identity get-principal
export USER1=$(dfx identity get-principal)
echo USER1 "${USER1}"
dfx canister call defi_dapp getBalance "(principal \"${AkitaDIP20}\")"
dfx identity use default
dfx canister call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 1: nat)"
dfx identity use user1
dfx canister call defi_dapp getBalance "(principal \"${AkitaDIP20}\")"
dfx identity use user2
dfx identity get-principal
export USER2=$(dfx identity get-principal)
echo USER2 "${USER2}"
dfx canister call defi_dapp getBalance "(principal \"${GoldenDIP20}\")"
dfx identity use default
dfx canister call defi_dapp credit "(principal \"${USER2}\", principal \"${GoldenDIP20}\", 100: nat)"
dfx identity use user2
dfx canister call defi_dapp getBalance "(principal \"${GoldenDIP20}\")"
dfx identity use user1
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 1: nat, principal \"${GoldenDIP20}\", 2: nat)"
dfx canister call defi_dapp getOrders
dfx identity use user2
echo "order will partially execute"
dfx canister call defi_dapp placeOrder "(principal \"${GoldenDIP20}\" : principal, 4: nat, principal \"${AkitaDIP20}\", 2: nat)"
dfx canister call defi_dapp getOrders
echo "expect user2 98 GoldenDIP20 1 AkitaDIP20, user1 2 GoldenDIP20"
dfx canister call defi_dapp getAllBalances
dfx identity use user1
echo "order will fail: not enough balance"
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 1: nat, principal \"${GoldenDIP20}\", 2: nat)"
dfx canister call defi_dapp getOrders
dfx canister call defi_dapp getAllBalances
dfx identity use default
dfx canister call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 1: nat)"
dfx identity use user1
dfx canister call defi_dapp getAllBalances
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 1: nat, principal \"${GoldenDIP20}\", 2: nat)"
dfx canister call defi_dapp getOrders
echo "expect empty vec"
dfx canister call defi_dapp getOrders | egrep "(vec ..)"
dfx canister call defi_dapp getAllBalances
echo "expect user1 4 GoldenDIP20"
dfx canister call defi_dapp getAllBalances | grep -B1 -A2 $GoldenDIP20 | grep -A2 $USER1 | grep "amount = 4"
echo "expect user2 96 GoldenDIP20"
dfx canister call defi_dapp getAllBalances | grep -B1 -A2 $GoldenDIP20 | grep -A2 $USER2 | grep "amount = 96"
echo "expect user2 2 AkitaDIP20"
dfx canister call defi_dapp getAllBalances | grep -B1 -A2 $AkitaDIP20 | grep -A2 $USER2 | grep "amount = 2"
echo "testing imbalanced trades"
dfx identity use default
dfx canister call defi_dapp clear
dfx identity use user1
dfx identity get-principal
dfx identity use default
dfx canister call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 9: nat)"
dfx identity use user1
dfx identity use user2
dfx identity get-principal
dfx identity use default
dfx canister call defi_dapp credit "(principal \"${USER2}\", principal \"${GoldenDIP20}\", 2: nat)"
dfx identity use user2
dfx canister call defi_dapp getAllBalances
dfx canister call defi_dapp placeOrder "(principal \"${GoldenDIP20}\" : principal, 2: nat, principal \"${AkitaDIP20}\", 1: nat)"
dfx canister call defi_dapp getOrders
dfx identity use user1
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 9: nat, principal \"${GoldenDIP20}\", 3: nat)"
dfx canister call defi_dapp getOrders
dfx canister call defi_dapp getAllBalances
echo "Check that it partially executed"
dfx canister call defi_dapp getAllBalances | grep "amount = 5"
echo "testing imbalanced trades with non-natural number trade amounts"
dfx identity use default
dfx canister call defi_dapp clear
dfx identity use user1
dfx identity get-principal
dfx identity use default
dfx canister call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 9: nat)"
dfx identity use user1
dfx identity use user2
dfx identity get-principal
dfx identity use default
dfx canister call defi_dapp credit "(principal \"${USER2}\", principal \"${GoldenDIP20}\", 2: nat)"
dfx identity use user2
dfx canister call defi_dapp getAllBalances
dfx canister call defi_dapp placeOrder "(principal \"${GoldenDIP20}\" : principal, 2: nat, principal \"${AkitaDIP20}\", 1: nat)"
dfx canister call defi_dapp getOrders
dfx identity use user1
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 9: nat, principal \"${GoldenDIP20}\", 4: nat)"
dfx canister call defi_dapp getOrders
dfx canister call defi_dapp getAllBalances
echo "Check that it did not execute"
dfx canister call defi_dapp getAllBalances | grep "amount = 9"
dfx identity use default
echo "PASS"
