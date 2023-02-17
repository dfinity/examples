#For more details on how to use this
#see https://github.com/dfinity/ic/tree/master/rs/rosetta-api/icrc1/ledger
#(latest IC_VERSION as of 3/6/2023)
IC_VERSION=b43543ce7365acd1720294e701e8e8361fa30c8f

curl -o ledger.wasm.gz https://download.dfinity.systems/ic/${IC_VERSION}/canisters/ic-icrc1-ledger.wasm.gz
gunzip ledger.wasm.gz
curl -o icrc1.did https://raw.githubusercontent.com/dfinity/ic/${IC_VERSION}/rs/rosetta-api/icrc1/ledger/icrc1.did
