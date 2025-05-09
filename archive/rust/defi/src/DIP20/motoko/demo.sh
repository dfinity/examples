#!/bin/bash

# set -e

# clear
dfx stop
rm -rf .dfx

ALICE_HOME=$(mktemp -d)
BOB_HOME=$(mktemp -d)
DAN_HOME=$(mktemp -d)
FEE_HOME=$(mktemp -d)
HOME=$ALICE_HOME

ALICE_PUBLIC_KEY="principal \"$( \
    HOME=$ALICE_HOME dfx identity get-principal
)\""
BOB_PUBLIC_KEY="principal \"$( \
    HOME=$BOB_HOME dfx identity get-principal
)\""
DAN_PUBLIC_KEY="principal \"$( \
    HOME=$DAN_HOME dfx identity get-principal
)\""
FEE_PUBLIC_KEY="principal \"$( \
    HOME=$FEE_HOME dfx identity get-principal
)\""

echo Alice id = $ALICE_PUBLIC_KEY
echo Bob id = $BOB_PUBLIC_KEY
echo Dan id = $DAN_PUBLIC_KEY
echo Fee id = $FEE_PUBLIC_KEY

dfx start --clean --background
dfx canister --no-wallet create --all
dfx build

TOKENID=$(dfx canister --no-wallet id token)
TOKENID="principal \"$TOKENID\""

echo Token id: $TOKENID

echo
echo == Install token canister
echo

HOME=$ALICE_HOME
eval dfx canister --no-wallet install token --argument="'(\"Test Token Logo\", \"Test Token Name\", \"Test Token Symbol\", 3, 1000000, $ALICE_PUBLIC_KEY, 0)'"

echo
echo == Initial setting for token canister
echo

eval dfx canister --no-wallet call token setFeeTo "'($FEE_PUBLIC_KEY)'"
eval dfx canister --no-wallet call token setFee "'(100)'"

echo
echo == Initial token balances for Alice and Bob, Dan, FeeTo
echo

echo Alice = $( \
    eval dfx canister --no-wallet call token balanceOf "'($ALICE_PUBLIC_KEY)'" \
)
echo Bob = $( \
    eval dfx canister --no-wallet call token balanceOf "'($BOB_PUBLIC_KEY)'" \
)
echo Dan = $( \
    eval dfx canister --no-wallet call token balanceOf "'($DAN_PUBLIC_KEY)'" \
)
echo FeeTo = $( \
    eval dfx canister --no-wallet call token balanceOf "'($FEE_PUBLIC_KEY)'" \
)

echo
echo == Transfer 0 tokens from Alice to Bob, should Return false, as value is smaller than fee.
echo

eval dfx canister --no-wallet call token transfer "'($BOB_PUBLIC_KEY, 0)'"

echo
echo == Transfer 0 tokens from Alice to Alice, should Return false, as value is smaller than fee.
echo

eval dfx canister --no-wallet call token transfer "'($ALICE_PUBLIC_KEY, 0)'"

echo
echo == Transfer 0.1 tokens from Alice to Bob, should success, revieve 0, as value = fee.
echo

eval dfx canister --no-wallet call token transfer "'($BOB_PUBLIC_KEY, 100)'"

echo
echo == Transfer 0.1 tokens from Alice to Alice, should success, revieve 0, as value = fee.
echo

eval dfx canister --no-wallet call token transfer "'($ALICE_PUBLIC_KEY, 100)'"

echo
echo == Transfer 100 tokens from Alice to Alice, should success.
echo

eval dfx canister --no-wallet call token transfer "'($ALICE_PUBLIC_KEY, 100_000)'"

echo
echo == Transfer 2000 tokens from Alice to Alice, should Return false, as no enough balance.
echo

eval dfx canister --no-wallet call token transfer "'($ALICE_PUBLIC_KEY, 2_000_000)'"

echo
echo == Transfer 0 tokens from Bob to Bob, should Return false, as value is smaller than fee.
echo

HOME=$BOB_HOME
eval dfx canister --no-wallet call token transfer "'($BOB_PUBLIC_KEY, 0)'"

echo
echo == Transfer 42 tokens from Alice to Bob, should success.
echo

HOME=$ALICE_HOME
eval dfx canister --no-wallet call token transfer "'($BOB_PUBLIC_KEY, 42_000)'"

echo
echo == Alice grants Dan permission to spend 1 of her tokens, should success.
echo

eval dfx canister --no-wallet call token approve "'($DAN_PUBLIC_KEY, 1_000)'"

echo
echo == Alice grants Dan permission to spend 0 of her tokens, should success.
echo

eval dfx canister --no-wallet call token approve "'($DAN_PUBLIC_KEY, 0)'"

echo
echo == Bob grants Dan permission to spend 1 of her tokens, should success.
echo

HOME=$BOB_HOME
eval dfx canister --no-wallet call token approve "'($DAN_PUBLIC_KEY, 1_000)'"

echo
echo == Dan transfer 1 token from Bob to Alice, should success.
echo

HOME=$DAN_HOME
eval dfx canister --no-wallet call token transferFrom "'($BOB_PUBLIC_KEY, $ALICE_PUBLIC_KEY, 1_000)'"


echo
echo == Transfer 40.9 tokens from Bob to Alice, should success.
echo

HOME=$BOB_HOME
eval dfx canister --no-wallet call token transfer "'($ALICE_PUBLIC_KEY, 40_900)'"

echo
echo == token balances for Alice, Bob, Dan and FeeTo.
echo

echo Alice = $( \
    eval dfx canister --no-wallet call token balanceOf "'($ALICE_PUBLIC_KEY)'" \
)
echo Bob = $( \
    eval dfx canister --no-wallet call token balanceOf "'($BOB_PUBLIC_KEY)'" \
)
echo Dan = $( \
    eval dfx canister --no-wallet call token balanceOf "'($DAN_PUBLIC_KEY)'" \
)
echo FeeTo = $( \
    eval dfx canister --no-wallet call token balanceOf "'($FEE_PUBLIC_KEY)'" \
)

echo
echo == Alice grants Dan permission to spend 50 of her tokens, should success.
echo

HOME=$ALICE_HOME
eval dfx canister --no-wallet call token approve "'($DAN_PUBLIC_KEY, 50_000)'"

echo
echo == Alices allowances 
echo

echo Alices allowance for Dan = $( \
    eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $DAN_PUBLIC_KEY)'" \
)
echo Alices allowance for Bob = $( \
    eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $BOB_PUBLIC_KEY)'" \
)

echo
echo == Dan transfers 40 tokens from Alice to Bob, should success.
echo

HOME=$DAN_HOME
eval dfx canister --no-wallet call token transferFrom "'($ALICE_PUBLIC_KEY, $BOB_PUBLIC_KEY, 40_000)'"

echo
echo == Alice transfer 1 tokens To Dan
echo

HOME=$ALICE_HOME
eval dfx canister --no-wallet call token transfer "'($DAN_PUBLIC_KEY, 1_000)'"

echo
echo == Dan transfers 40 tokens from Alice to Bob, should Return false, as allowance remain 10, smaller than 40.
echo

HOME=$DAN_HOME
eval dfx canister --no-wallet call token transferFrom "'($ALICE_PUBLIC_KEY, $BOB_PUBLIC_KEY, 40_000)'"

echo
echo == Token balance for Alice and Bob and Dan
echo

echo Alice = $( \
    eval dfx canister --no-wallet call token balanceOf "'($ALICE_PUBLIC_KEY)'" \
)
echo Bob = $( \
    eval dfx canister --no-wallet call token balanceOf "'($BOB_PUBLIC_KEY)'" \
)
echo Dan = $( \
    eval dfx canister --no-wallet call token balanceOf "'($DAN_PUBLIC_KEY)'" \
)
echo Fee = $( \
    eval dfx canister --no-wallet call token balanceOf "'($FEE_PUBLIC_KEY)'" \
)

echo
echo == Alice allowances
echo

echo Alices allowance for Bob = $( \
    eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $BOB_PUBLIC_KEY)'" \
)
echo Alices allowance for Dan = $( \
    eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $DAN_PUBLIC_KEY)'" \
)


echo
echo == Alice grants Bob permission to spend 100 of her tokens
echo

HOME=$ALICE_HOME
eval dfx canister --no-wallet call token approve "'($BOB_PUBLIC_KEY, 100_000)'"

echo
echo == Alice allowances
echo

echo Alices allowance for Bob = $( \
    eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $BOB_PUBLIC_KEY)'" \
)
echo Alices allowance for Dan = $( \
    eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $DAN_PUBLIC_KEY)'" \
)

echo
echo == Bob transfers 99 tokens from Alice to Dan
echo

HOME=$BOB_HOME
eval dfx canister --no-wallet call token transferFrom "'($ALICE_PUBLIC_KEY, $DAN_PUBLIC_KEY, 99_000)'"

echo
echo == Balances
echo

echo Alice = $( \
    eval dfx canister --no-wallet call token balanceOf "'($ALICE_PUBLIC_KEY)'" \
)
echo Bob = $( \
    eval dfx canister --no-wallet call token balanceOf "'($BOB_PUBLIC_KEY)'" \
)
echo Dan = $( \
    eval dfx canister --no-wallet call token balanceOf "'($DAN_PUBLIC_KEY)'" \
)
echo Fee = $( \
    eval dfx canister --no-wallet call token balanceOf "'($FEE_PUBLIC_KEY)'" \
)

echo
echo == Alice allowances
echo

echo Alices allowance for Bob = $( eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $BOB_PUBLIC_KEY)'" )
echo Alices allowance for Dan = $( eval dfx canister --no-wallet call token allowance "'($ALICE_PUBLIC_KEY, $DAN_PUBLIC_KEY)'" )

echo
echo == Dan grants Bob permission to spend 100 of this tokens, should success.
echo

HOME=$DAN_HOME
eval dfx canister --no-wallet call token approve "'($BOB_PUBLIC_KEY, 100_000)'"

echo
echo == Dan grants Bob permission to spend 50 of this tokens
echo

eval dfx canister --no-wallet call token approve "'($BOB_PUBLIC_KEY, 50_000)'"

echo
echo == Dan allowances
echo

echo Dan allowance for Bob = $( \
    eval dfx canister --no-wallet call token allowance "'($DAN_PUBLIC_KEY, $BOB_PUBLIC_KEY)'" \
)
echo Dan allowance for Alice = $( \
    eval dfx canister --no-wallet call token allowance "'($DAN_PUBLIC_KEY, $ALICE_PUBLIC_KEY)'" \
)

echo
echo == Dan change Bobs permission to spend 40 of this tokens instead of 50
echo

eval dfx canister --no-wallet call token approve "'($BOB_PUBLIC_KEY, 40_000)'"

echo
echo == Dan allowances
echo

echo Dan allowance for Bob = $( \
    eval dfx canister --no-wallet call token allowance "'($DAN_PUBLIC_KEY, $BOB_PUBLIC_KEY)'" \
)
echo Dan allowance for Alice = $( \
    eval dfx canister --no-wallet call token allowance "'($DAN_PUBLIC_KEY, $ALICE_PUBLIC_KEY)'" \
)

echo
echo == logo
echo
eval dfx canister --no-wallet call token logo

echo
echo == name
echo
eval dfx canister --no-wallet call token name

echo
echo == symbol
echo
eval dfx canister --no-wallet call token symbol

echo
echo == decimals
echo
eval dfx canister --no-wallet call token decimals

echo
echo == totalSupply
echo
eval dfx canister --no-wallet call token totalSupply

echo
echo == getMetadata
echo
eval dfx canister --no-wallet call token getMetadata

echo
echo == historySize
echo
eval dfx canister --no-wallet call token historySize

echo
echo == getTransaction
echo
eval dfx canister --no-wallet call token getTransaction "'(1)'"

echo
echo == getTransactions
echo
eval dfx canister --no-wallet call token getTransactions "'(0,100)'" 

echo
echo == getUserTransactionAmount
echo
eval dfx canister --no-wallet  call token getUserTransactionAmount "'($ALICE_PUBLIC_KEY)'" 

echo
echo == getUserTransactions
echo
eval dfx canister --no-wallet call token getUserTransactions "'($ALICE_PUBLIC_KEY, 0, 1000)'"

echo
echo == getTokenInfo
echo
eval dfx canister --no-wallet  call token getTokenInfo

echo
echo == getHolders
echo
eval dfx canister --no-wallet  call token getHolders "'(0,100)'"

echo
echo == getAllowanceSize
echo
eval dfx canister --no-wallet  call token getAllowanceSize

echo
echo == getUserApprovals
echo
eval dfx canister --no-wallet  call token getUserApprovals "'($ALICE_PUBLIC_KEY)'"

echo
echo == get alice getUserTransactions
echo
eval dfx canister --no-wallet  call token getUserTransactions "'($ALICE_PUBLIC_KEY, 0, 1000)'"

echo
echo == get bob History
echo
eval dfx canister --no-wallet  call token getUserTransactions "'($BOB_PUBLIC_KEY, 0, 1000)'"

echo
echo == get dan History
echo
eval dfx canister --no-wallet  call token getUserTransactions "'($DAN_PUBLIC_KEY, 0, 1000)'"

echo
echo == get fee History
echo
eval dfx canister --no-wallet  call token getUserTransactions "'($FEE_PUBLIC_KEY, 0, 1000)'"


echo
echo == Upgrade token
echo
HOME=$ALICE_HOME
eval dfx canister --no-wallet install token --argument="'(\"test\", \"Test Token\", \"TT\", 2, 100, $ALICE_PUBLIC_KEY)'" -m=upgrade

echo
echo == all History
echo
eval dfx canister --no-wallet call token getTransactions "'(0, 1000)'"

echo
echo == getTokenInfo
echo
dfx canister --no-wallet call token getTokenInfo

echo
echo == get alice History
echo
eval dfx canister --no-wallet  call token getUserTransactions "'($ALICE_PUBLIC_KEY, 0, 1000)'"

dfx stop
