### Console Output from Running `node clean-startup.mjs --deployForTesting`

##### This is similar to `npm run deployForTesting`, except with verbose output and no declarations generation.

Provided for reference.

```
λ: ~/examples/motoko/invoice-canister$ node clean-startup.mjs --deployForTesting
spinning up a local replica for testing the invoice canister...
restarting dfx clean in background

$ dfx stop
Using shared network 'local' defined in /home/xyz/.config/dfx/networks.json

$ dfx info networks-json-path
/home/xyz/.config/dfx/networks.json
adding the dfx nns ICP ledger's funded secp256k1 identity if not already added to switch current identity to

$ dfx identity list
anonymous
default
nns-funded-secp256k1

$ dfx identity use nns-funded-secp256k1
Using identity: "nns-funded-secp256k1".

$ dfx identity get-principal
hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe
running dfx nns install

$ dfx nns install
Found local replica running on port 39953
Checking out the environment...
Found local replica running on port 39953
Installing the core backend wasm canisters...
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/registry-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/governance-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/governance-canister_test.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/ledger-canister_notify-method.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/root-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/cycles-minting-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/lifeline.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/genesis-token-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/identity-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/nns-ui-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/sns-wasm-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/ic-ckbtc-minter.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/sns-root-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/sns-governance-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/sns-swap-canister.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/ic-icrc1-ledger.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/ic-icrc1-archive.wasm
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/ic-icrc1-index.wasm
ic-nns-init --url http://127.0.0.1:39953/ --wasm-dir /home/xyz/.cache/dfinity/versions/0.12.1/wasms --initialize-ledger-with-test-accounts 5b315d2f6702cb3a27d826161797d7b2c2e131cd312aece51d4d5574d1247087 --initialize-ledger-with-test-accounts 2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138 --sns-subnet skket-3bree-end22-vdowh-wcpcr-sfv63-pi7nk-nvihe-gpvrh-lbz7h-cae
[ic-nns-init] The content of the registry will be initialized with an empty content. This is most likely not what you want. Use --initial-registry or --registry-local-store-dir to specify initial content.
[ic-nns-init] Initial neuron CSV or PB path not specified, initializing with test neurons
[ic-nns-init] Initializing with test ledger account: 5b315d2f6702cb3a27d826161797d7b2c2e131cd312aece51d4d5574d1247087
[ic-nns-init] Initializing with test ledger account: 2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138
[ic-nns-init] Initialized governance.
NNS canisters created after 2.1 s
Compiling Wasm for registry-canister in task on thread: ThreadId(10)
looking up registry-canister at REGISTRY_CANISTER_WASM_PATH
looking up governance-canister at GOVERNANCE_CANISTER_TEST_WASM_PATH
looking up ledger-canister at LEDGER_CANISTER_NOTIFY_METHOD_WASM_PATH
Compiling Wasm for governance-canister in task on thread: ThreadId(11)
Compiling Wasm for ledger-canister in task on thread: ThreadId(12)
Using pre-built binary for ledger-canister (size = 1387710)
Using pre-built binary for governance-canister (size = 1562995)
Done compiling the wasm for ledger-canister
Using pre-built binary for registry-canister (size = 2479668)
Attempting to install wasm into canister with ID: ryjl3-tyaaa-aaaaa-aaaba-cai
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: ryjl3-tyaaa-aaaaa-aaaba-cai
  wasm_module: <1387710 bytes>
  arg: <790 bytes>
  compute_allocation: None
  memory_allocation: Some("4_294_967_296")
  query_allocation: None
}

Done compiling the wasm for registry-canister
Attempting to install wasm into canister with ID: rwlgt-iiaaa-aaaaa-aaaaa-cai
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: rwlgt-iiaaa-aaaaa-aaaaa-cai
  wasm_module: <2479668 bytes>
  arg: <69 bytes>
  compute_allocation: None
  memory_allocation: Some("4_294_967_296")
  query_allocation: None
}

Done compiling the wasm for governance-canister
Attempting to install wasm into canister with ID: rrkah-fqaaa-aaaaa-aaaaq-cai
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: rrkah-fqaaa-aaaaa-aaaaq-cai
  wasm_module: <1562995 bytes>
  arg: <361 bytes>
  compute_allocation: None
  memory_allocation: Some("4_294_967_296")
  query_allocation: None
}

Successfully installed wasm into canister with ID: ryjl3-tyaaa-aaaaa-aaaba-cai
Installed ryjl3-tyaaa-aaaaa-aaaba-cai with ledger-canister
Successfully installed wasm into canister with ID: rwlgt-iiaaa-aaaaa-aaaaa-cai
Installed rwlgt-iiaaa-aaaaa-aaaaa-cai with registry-canister
Successfully installed wasm into canister with ID: rrkah-fqaaa-aaaaa-aaaaq-cai
Installed rrkah-fqaaa-aaaaa-aaaaq-cai with governance-canister
Compiling Wasm for root-canister in task on thread: ThreadId(12)
Compiling Wasm for cycles-minting-canister in task on thread: ThreadId(11)
Attempting to install wasm into canister with ID: rno2w-sqaaa-aaaaa-aaacq-cai
looking up lifeline at LIFELINE_WASM_PATH
looking up root-canister at ROOT_CANISTER_WASM_PATH
looking up cycles-minting-canister at CYCLES_MINTING_CANISTER_WASM_PATH
Using pre-built binary for lifeline (size = 139468)
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: rno2w-sqaaa-aaaaa-aaacq-cai
  wasm_module: <139468 bytes>
  arg: <0 bytes>
  compute_allocation: None
  memory_allocation: Some("1_073_741_824")
  query_allocation: None
}

Using pre-built binary for root-canister (size = 684147)
Compiling Wasm for genesis-token-canister in task on thread: ThreadId(10)
Done compiling the wasm for root-canister
Compiling Wasm for sns-wasm-canister in task on thread: ThreadId(12)
looking up genesis-token-canister at GENESIS_TOKEN_CANISTER_WASM_PATH
looking up sns-wasm-canister at SNS_WASM_CANISTER_WASM_PATH
Using pre-built binary for cycles-minting-canister (size = 856649)
Attempting to install wasm into canister with ID: r7inp-6aaaa-aaaaa-aaabq-cai
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: r7inp-6aaaa-aaaaa-aaabq-cai
  wasm_module: <684147 bytes>
  arg: <9 bytes>
  compute_allocation: None
  memory_allocation: Some("1_073_741_824")
  query_allocation: None
}

Using pre-built binary for genesis-token-canister (size = 466311)
Using pre-built binary for sns-wasm-canister (size = 1145567)
Done compiling the wasm for cycles-minting-canister
Attempting to install wasm into canister with ID: rkp4c-7iaaa-aaaaa-aaaca-cai
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: rkp4c-7iaaa-aaaaa-aaaca-cai
  wasm_module: <856649 bytes>
  arg: <136 bytes>
  compute_allocation: None
  memory_allocation: Some("1_073_741_824")
  query_allocation: None
}

Done compiling the wasm for genesis-token-canister
Attempting to install wasm into canister with ID: renrk-eyaaa-aaaaa-aaada-cai
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: renrk-eyaaa-aaaaa-aaada-cai
  wasm_module: <466311 bytes>
  arg: <8 bytes>
  compute_allocation: None
  memory_allocation: Some("1_073_741_824")
  query_allocation: None
}

Done compiling the wasm for sns-wasm-canister
Attempting to install wasm into canister with ID: qaa6y-5yaaa-aaaaa-aaafa-cai
Install args: InstallCodeArgs {
  mode: Reinstall
  canister_id: qaa6y-5yaaa-aaaaa-aaafa-cai
  wasm_module: <1145567 bytes>
  arg: <63 bytes>
  compute_allocation: None
  memory_allocation: Some("1_073_741_824")
  query_allocation: None
}

Successfully installed wasm into canister with ID: rkp4c-7iaaa-aaaaa-aaaca-cai
Installed rkp4c-7iaaa-aaaaa-aaaca-cai with cycles-minting-canister
Successfully installed wasm into canister with ID: r7inp-6aaaa-aaaaa-aaabq-cai
Installed r7inp-6aaaa-aaaaa-aaabq-cai with root-canister
Successfully installed wasm into canister with ID: rno2w-sqaaa-aaaaa-aaacq-cai
Installed rno2w-sqaaa-aaaaa-aaacq-cai with the lifeline handler
Successfully installed wasm into canister with ID: qaa6y-5yaaa-aaaaa-aaafa-cai
Installed qaa6y-5yaaa-aaaaa-aaafa-cai with sns-wasm-canister
Successfully installed wasm into canister with ID: renrk-eyaaa-aaaaa-aaada-cai
Installed renrk-eyaaa-aaaaa-aaada-cai with genesis-token-canister
NNS canisters installed after 13.6 s
NNS canisters set up after 15.7 s
[ic-nns-init] All NNS canisters have been set up on the replica with http://127.0.0.1:39953/
Uploading NNS configuration data...
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/internet_identity_dev.wasm
Installing code for canister internet_identity, with canister ID qhbym-qaaaa-aaaaa-aaafq-cai
Installed internet_identity at qhbym-qaaaa-aaaaa-aaafq-cai
Already downloaded: /home/xyz/.cache/dfinity/versions/0.12.1/wasms/nns-dapp_local.wasm
Installing code for canister nns-dapp, with canister ID qsgjb-riaaa-aaaaa-aaaga-cai
Installed nns-dapp at qsgjb-riaaa-aaaaa-aaaga-cai
Configuring the NNS...

######################################
# NNS CANISTER INSTALLATION COMPLETE #
######################################

Backend canisters:
nns-registry          rwlgt-iiaaa-aaaaa-aaaaa-cai
nns-governance        rrkah-fqaaa-aaaaa-aaaaq-cai
nns-ledger            ryjl3-tyaaa-aaaaa-aaaba-cai
nns-root              r7inp-6aaaa-aaaaa-aaabq-cai
nns-cycles-minting    rkp4c-7iaaa-aaaaa-aaaca-cai
nns-lifeline          rno2w-sqaaa-aaaaa-aaacq-cai
nns-genesis-token     renrk-eyaaa-aaaaa-aaada-cai
nns-identity          rdmx6-jaaaa-aaaaa-aaadq-cai
nns-ui                qoctq-giaaa-aaaaa-aaaea-cai
nns-sns-wasm          qaa6y-5yaaa-aaaaa-aaafa-cai
nns-ic-ckbtc-minter   qjdve-lqaaa-aaaaa-aaaeq-cai


Frontend canisters:
internet_identity     http://qhbym-qaaaa-aaaaa-aaafq-cai.localhost:8080/
nns-dapp              http://qsgjb-riaaa-aaaaa-aaaga-cai.localhost:8080/

deploying invoice canister

$ dfx deploy invoice
Creating a wallet canister on the local network.
The wallet canister on the "local" network for user "nns-funded-secp256k1" is "qvhpv-4qaaa-aaaaa-aaagq-cai"
Deploying: invoice
Creating canisters...
Creating canister invoice...
invoice canister created with canister id: q4eej-kyaaa-aaaaa-aaaha-cai
Building canisters...
Shrink WASM module size.
Installing canisters...
Creating UI canister on the local network.
The UI canister on the "local" network is "q3fc5-haaaa-aaaaa-aaahq-cai"
Installing code for canister invoice, with canister ID q4eej-kyaaa-aaaaa-aaaha-cai
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    invoice: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=q4eej-kyaaa-aaaaa-aaaha-cai

$ dfx canister id invoice
q4eej-kyaaa-aaaaa-aaaha-cai
deploying ICP ledger from downloaded wasm and swapping its public and private did files

$ dfx ledger account-id
2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138

$ dfx deploy icp_ledger_canister  --argument '(record { minting_account = "2b8fbde99de881f695f279d2a892b1137bfe81a42d7694e064b1be58701e1138";  initial_values = vec {};  send_whitelist = vec {} } )'
Deploying: icp_ledger_canister
Creating canisters...
Creating canister icp_ledger_canister...
icp_ledger_canister canister created with canister id: sgymv-uiaaa-aaaaa-aaaia-cai
Building canisters...
Executing ''
Installing canisters...
Installing code for canister icp_ledger_canister, with canister ID sgymv-uiaaa-aaaaa-aaaia-cai
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    icp_ledger_canister: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=sgymv-uiaaa-aaaaa-aaaia-cai
    invoice: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=q4eej-kyaaa-aaaaa-aaaha-cai
deploying ICRC1 token canister with name Internet Computer Random Curency One Example Token with hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe as minting principal

$ dfx deploy icrc1_token_ledger_canister_ex1 --argument '(  record {  token_symbol = "_1ICRC1EX"; token_name =  "Internet Computer Random Curency One Example Token"; minting_account = record { owner = principal"hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe" }; transfer_fee = 10_000; metadata = vec {}; initial_balances = vec { };  archive_options = record { num_blocks_to_archive = 2000; trigger_threshold = 1000; controller_id = principal"hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe"; }; } )'
Deploying: icrc1_token_ledger_canister_ex1
Creating canisters...
Creating canister icrc1_token_ledger_canister_ex1...
icrc1_token_ledger_canister_ex1 canister created with canister id: sbzkb-zqaaa-aaaaa-aaaiq-cai
Building canisters...
Executing ''
Installing canisters...
Installing code for canister icrc1_token_ledger_canister_ex1, with canister ID sbzkb-zqaaa-aaaaa-aaaiq-cai
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    icp_ledger_canister: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=sgymv-uiaaa-aaaaa-aaaia-cai
    icrc1_token_ledger_canister_ex1: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=sbzkb-zqaaa-aaaaa-aaaiq-cai
    invoice: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=q4eej-kyaaa-aaaaa-aaaha-cai
deploying ICRC1 token canister with name Two Internet Computer Random Curency One Example Token with hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe as minting principal

$ dfx deploy icrc1_token_ledger_canister_ex2 --argument '(  record {  token_symbol = "_2ICRC1EX"; token_name =  "Two Internet Computer Random Curency One Example Token"; minting_account = record { owner = principal"hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe" }; transfer_fee = 10_000; metadata = vec {}; initial_balances = vec { };  archive_options = record { num_blocks_to_archive = 2000; trigger_threshold = 1000; controller_id = principal"hpikg-6exdt-jn33w-ndty3-fc7jc-tl2lr-buih3-cs3y7-tftkp-sfp62-gqe"; }; } )'
Deploying: icrc1_token_ledger_canister_ex2
Creating canisters...
Creating canister icrc1_token_ledger_canister_ex2...
icrc1_token_ledger_canister_ex2 canister created with canister id: si2b5-pyaaa-aaaaa-aaaja-cai
Building canisters...
Executing ''
Installing canisters...
Installing code for canister icrc1_token_ledger_canister_ex2, with canister ID si2b5-pyaaa-aaaaa-aaaja-cai
Deployed canisters.
URLs:
  Backend canister via Candid interface:
    icp_ledger_canister: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=sgymv-uiaaa-aaaaa-aaaia-cai
    icrc1_token_ledger_canister_ex1: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=sbzkb-zqaaa-aaaaa-aaaiq-cai
    icrc1_token_ledger_canister_ex2: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=si2b5-pyaaa-aaaaa-aaaja-cai
    invoice: http://127.0.0.1:8080/?canisterId=q3fc5-haaaa-aaaaa-aaahq-cai&id=q4eej-kyaaa-aaaaa-aaaha-cai
disbursing funds to principal subaccounts of invoice canister for E2E testing

$ dfx ledger transfer 5bea0a832af66531a1c2dda9e4f027f6c31dd943af039ff509ae492841b8b980 --memo 123 --e8s 100000000000000
Transfer sent at BlockHeight: 5

$ dfx canister call icp_ledger_canister transfer '( record {  memo = 1; fee = record {  e8s = 0  }; amount = record {  e8s = 100000000000000  }; to = blob "[\ea\0a\83*\f6e1\a1\c2\dd\a9\e4\f0\27\f6\c3\1d\d9C\af\03\9f\f5\09\aeI(A\b8\b9\80" } )'
(variant { Ok = 0 : nat64 })

$ dfx canister call icrc1_token_ledger_canister_ex1 icrc1_transfer '( record {  to = record {   owner = principal"q4eej-kyaaa-aaaaa-aaaha-cai";   subaccount = opt blob"\00\00\00\007;lQ\e0\9aG&\1e\fb\bfI\c2\95\bdS\88\08\de2\f8\bc\eb\a3bZl\60";  }; amount = 100000000000; } )'
(variant { Ok = 0 : nat })

$ dfx canister call icrc1_token_ledger_canister_ex2 icrc1_transfer '( record {  to = record {   owner = principal"q4eej-kyaaa-aaaaa-aaaha-cai";   subaccount = opt blob"\00\00\00\007;lQ\e0\9aG&\1e\fb\bfI\c2\95\bdS\88\08\de2\f8\bc\eb\a3bZl\60";  }; amount = 100000000000; } )'
(variant { Ok = 0 : nat })

all canisters deployed and ready to be called...
```