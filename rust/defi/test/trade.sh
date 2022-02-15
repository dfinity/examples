set -x
set -e
trap 'catch' ERR
catch() {
  dfx identity use default
  echo "FAIL"
  exit 1
}
dfx identity use default
export WALLET=$(dfx identity get-wallet)
echo wallet "${WALLET}"
export AkitaDIP20=$(dfx canister id AkitaDIP20)
echo AkitaDIP20 "${AkitaDIP20}"
export GoldenDIP20=$(dfx canister id GoldenDIP20)
echo GoldenDIP20 "${GoldenDIP20}"
dfx canister --wallet "${WALLET}" call defi_dapp clear
dfx identity new user1 || true
dfx identity new user2 || true
dfx identity use user1
dfx identity get-principal
export USER1=$(dfx identity get-principal)
echo USER1 "${USER1}"
dfx canister call defi_dapp getBalance "(principal \"${AkitaDIP20}\")"
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 1: nat)"
dfx identity use user1
dfx canister call defi_dapp getBalance "(principal \"${AkitaDIP20}\")"
dfx identity use user2
dfx identity get-principal
export USER2=$(dfx identity get-principal)
echo USER2 "${USER2}"
dfx canister call defi_dapp getBalance "(principal \"${GoldenDIP20}\")"
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp credit "(principal \"${USER2}\", principal \"${GoldenDIP20}\", 100: nat)"
dfx identity use user2
dfx canister call defi_dapp getBalance "(principal \"${GoldenDIP20}\")"
dfx identity use user1
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 1: nat, principal \"${GoldenDIP20}\", 2: nat)"
dfx canister call defi_dapp getOrders
dfx identity use user2
dfx canister call defi_dapp placeOrder "(principal \"${GoldenDIP20}\" : principal, 4: nat, principal \"${AkitaDIP20}\", 2: nat)"
dfx canister call defi_dapp getOrders
dfx identity use user1
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 1: nat, principal \"${GoldenDIP20}\", 2: nat)"
dfx canister call defi_dapp getOrders
dfx canister call defi_dapp getAllBalances
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 1: nat)"
dfx identity use user1
dfx canister call defi_dapp getAllBalances
dfx canister call defi_dapp placeOrder "(principal \"${AkitaDIP20}\" : principal, 1: nat, principal \"${GoldenDIP20}\", 2: nat)"
echo "expect empty vec"
dfx canister call defi_dapp getOrders
dfx canister call defi_dapp getOrders | egrep "(vec ..)"
echo "expect 4 tlwi3"
dfx canister call defi_dapp getAllBalances
dfx canister call defi_dapp getAllBalances | grep "amount = 4"
dfx identity use user2
echo "expect 96 tlwi3 and 2 tfuft"
dfx canister call defi_dapp getAllBalances
dfx canister call defi_dapp getAllBalances | grep "amount = 96"
echo "testing imbalanced trades"
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp clear
dfx identity use user1
dfx identity get-principal
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 9: nat)"
dfx identity use user1
dfx identity use user2
dfx identity get-principal
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp credit "(principal \"${USER2}\", principal \"${GoldenDIP20}\", 2: nat)"
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
dfx canister --wallet "${WALLET}" call defi_dapp clear
dfx identity use user1
dfx identity get-principal
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp credit "(principal \"${USER1}\", principal \"${AkitaDIP20}\", 9: nat)"
dfx identity use user1
dfx identity use user2
dfx identity get-principal
dfx identity use default
dfx canister --wallet "${WALLET}" call defi_dapp credit "(principal \"${USER2}\", principal \"${GoldenDIP20}\", 2: nat)"
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
