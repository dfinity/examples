## Build the canister wasm file
`cd rust/exchange_rate`
`wasm-pack build --out-dir ./wasm/ --out-name exchange_rate.wasm --release`
## Deploy the canister
`dfx deploy`

# Mainnet exchange_rate canister Id
mfwph-oqaaa-aaaam-qafsq-cai

Check the status of the canister: `dfx canister --network=ic status mfwph-oqaaa-aaaam-qafsq-cai`
