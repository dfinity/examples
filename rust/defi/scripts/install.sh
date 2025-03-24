set -e
dfx stop && dfx start --background --clean --host 127.0.0.1:8000


### === DEPLOY LOCAL LEDGER =====
dfx identity new minter --disable-encryption || true
dfx identity use minter
export MINT_ACC=$(dfx ledger account-id)

dfx identity use default
export LEDGER_ACC=$(dfx ledger account-id)

# Use private api for install
rm src/ledger/ledger.did || true
cp src/ledger/ledger.private.did src/ledger/ledger.did

dfx deploy ledger --argument '(record  {
    minting_account = "'${MINT_ACC}'";
    initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; };
    send_whitelist = vec {}
    })'
export LEDGER_ID=$(dfx canister id ledger)

# Replace with public api
rm src/ledger/ledger.did
cp src/ledger/ledger.public.did src/ledger/ledger.did

### === DEPLOY DIP TOKENS =====

dfx canister create AkitaDIP20
dfx canister create GoldenDIP20
dfx build AkitaDIP20
dfx build GoldenDIP20

export ROOT_PRINCIPAL="principal \"$(dfx identity get-principal)\""
dfx canister install GoldenDIP20 --argument="(\"https://dogbreedslist.com/wp-content/uploads/2019/08/Are-Golden-Retrievers-easy-to-train.png\", \"Golden Coin\", \"GLD\", 8, 10000000000000000, $ROOT_PRINCIPAL, 10000)"
dfx canister install AkitaDIP20 --argument="(\"https://akitagoose.com/wp-content/uploads/2021/12/IMG_0674.png\", \"Akita Coin\", \"AKI\", 8, 10000000000000000, $ROOT_PRINCIPAL, 10000)"

# set fees 
dfx canister call AkitaDIP20 setFeeTo "($ROOT_PRINCIPAL)"
dfx canister call AkitaDIP20 setFee "(420)" 
dfx canister call GoldenDIP20 setFeeTo "($ROOT_PRINCIPAL)"
dfx canister call GoldenDIP20 setFee "(420)" 

### === DEPLOY INTERNET IDENTITY =====

II_FETCH_ROOT_KEY=1 dfx deploy internet_identity --no-wallet --argument '(null)'

## === INSTALL FRONTEND / BACKEND ==== 

dfx deploy defi_dapp --argument "(opt principal \"$LEDGER_ID\")"

dfx generate defi_dapp
dfx build defi_dapp
dfx build AkitaDIP20
dfx build GoldenDIP20
dfx generate defi_dapp
dfx generate AkitaDIP20
dfx generate GoldenDIP20
dfx generate ledger

dfx canister create frontend
pushd src/frontend
npm install
npm run build
popd
dfx build frontend
dfx canister install frontend

echo "===== VISIT DEFI FRONTEND ====="
echo "http://localhost:8000?canisterId=$(dfx canister id frontend)"
echo "===== VISIT DEFI FRONTEND ====="
