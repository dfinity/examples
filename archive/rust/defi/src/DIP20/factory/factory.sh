#!/bin/bash

dfxDir="/home/dan/.config/dfx"
candidDir="/home/dan/dev/psy/ic-token/factory/"

factoryId=$(dfx canister id factory)
AlicePem="${dfxDir}/identity/Alice/identity.pem"
BobPem="${dfxDir}/identity/Bob/identity.pem"
CharliePem="${dfxDir}/identity/Charlie/identity.pem"
AlicePrincipalId=$(dfx identity use Alice 2>/dev/null;dfx identity get-principal)
BobPrincipalId=$(dfx identity use Bob 2>/dev/null;dfx identity get-principal)
CharliePrincipalId=$(dfx identity use Charlie 2>/dev/null;dfx identity get-principal)
factoryCandidFile="${candidDir}/factory.did"
icxPrologueFactory="--candid=${factoryCandidFile}"

dfx identity use default 2>/dev/null

declare -A nameToPrincipal=( ["Alice"]="$AlicePrincipalId" ["Bob"]="$BobPrincipalId" ["Charlie"]="$CharliePrincipalId")
declare -A nameToPem=( ["Alice"]="$AlicePem" ["Bob"]="$BobPem" ["Charlie"]="$CharliePem")

create(){
    fromPem="${nameToPem[$1]}"
    amount=$2
    icx --pem=$fromPem update $factoryId create "(\"No logo\", \"Boss token\", \"BS\", 18, 100, principal \"$BobPrincipalId\", vec{principal \"$AlicePrincipalId\"}, 200, 1, variant {DIP20Motoko})" $icxPrologueFactory
}

help()
{
    printf "\n\nPrincipal ids\n"
    printf "Alice: ${AlicePrincipalId}\n"
    printf "Bob: ${BobPrincipalId}\n"
    printf "Charlie: ${CharliePrincipalId}\n"

    printf "\n\nAccount ids\n"
    printf "Alice: ${AliceAccountId}\n"
    printf "Bob: ${BobAccountId}\n"
    printf "Charlie: ${CharlieAccountId}\n\n\n"
}
