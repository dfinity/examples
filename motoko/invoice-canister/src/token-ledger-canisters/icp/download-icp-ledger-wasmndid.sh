#For more details on how to use this
#see https://internetcomputer.org/docs/current/developer-docs/integrations/ledger/ledger-local-setup
#(latest IC_VERSION as of 3/6/2023)
IC_VERSION=dd3a710b03bd3ae10368a91b255571d012d1ec2f

curl -o ledger.wasm.gz https://download.dfinity.systems/ic/${IC_VERSION}/canisters/ledger-canister_notify-method.wasm.gz
gunzip ledger.wasm.gz
curl -o ledger.private.did https://raw.githubusercontent.com/dfinity/ic/${IC_VERSION}/rs/rosetta-api/ledger.did
curl -o ledger.public.did https://raw.githubusercontent.com/dfinity/ic/${IC_VERSION}/rs/rosetta-api/ledger_canister/ledger.did

