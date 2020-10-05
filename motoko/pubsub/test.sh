dfx canister create --all
dfx build
dfx canister install --all
dfx canister call sub init
dfx canister call sub getCount # should be 0
dfx canister call pub publish '(record { "topic" = "Apples"; "value" = 2 })'
sleep 2 # wait for update
dfx canister call sub getCount # should be 2
dfx canister call pub publish '(record { "topic" = "Bananas"; "value" = 3 })'
sleep 2
dfx canister call sub getCount # should still be 2
