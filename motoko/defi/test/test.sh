#!/bin/bash


TEST_DIR="$(dirname "$0")"

# ===== TEST EDGE CASES =====
# - withdraw more than user balance
# - deposit DIP two times to DEX
dfx stop
dfx start --background --clean &> /dev/null

# config
export AKITA_SUPPLY=100000000;
export ICP_SUPPLY=100000000;
export DIP20_FEE=100;
export DIP_DEPOSIT=10000;
export ICP_FEE=10000;
export ICP_DEPOSIT=1000000;

#setup environment
$TEST_DIR/setup.sh $AKITA_SUPPLY $ICP_SUPPLY

# environment vars for tests
export DEX_PRINCIPLE=$(dfx canister --no-wallet id defi_dapp)
export AKITA_ID=$(dfx canister --no-wallet id AkitaDIP20)
export DIP_DEPOSIT_DEX_BALANCE=$(($DIP_DEPOSIT))
export DIP_DEPOSIT_DIP_BALANCE=$(($AKITA_SUPPLY-2*$DIP20_FEE-$DIP_DEPOSIT))
export DIP_DEPOSIT2_DEX_BALANCE=$((2*$DIP_DEPOSIT))
export DIP_DEPOSIT2_DIP_BALANCE=$(($AKITA_SUPPLY-4*$DIP20_FEE-2*$DIP_DEPOSIT))

#run tests
$TEST_DIR/test-edge.sh

dfx stop

# ===== TEST MULTI USER DEPOSIT/WITHDRAW =====
# - deposit DIP for two users
dfx stop
dfx start --background --clean &> /dev/null

# config
export AKITA_SUPPLY=100000000;
export ICP_SUPPLY=100000000;
export DIP20_FEE=100;
export DIP_DEPOSIT=10000;
export ICP_FEE=10000;
export ICP_DEPOSIT=1000000;
export AKITA_BALANCE_USER1=2000000;

#setup environment
$TEST_DIR/setup.sh $AKITA_SUPPLY $ICP_SUPPLY

# environment vars for tests
export DEX_PRINCIPLE=$(dfx canister --no-wallet id defi_dapp)
export AKITA_ID=$(dfx canister --no-wallet id AkitaDIP20)
export DIP_DEPOSIT_DEX_BALANCE=$(($DIP_DEPOSIT))
export DIP_DEPOSIT_DIP_BALANCE=$(($AKITA_SUPPLY-$AKITA_BALANCE_USER1-3*$DIP20_FEE-$DIP_DEPOSIT))
export DIP_DEPOSIT_DIP_BALANCE1=$(($AKITA_BALANCE_USER1-2*$DIP20_FEE-$DIP_DEPOSIT))

#run tests
$TEST_DIR/test-multi-user.sh

dfx stop

# ===== TEST SINGLE USER DEPOSIT/WITHDRAW =====
# - icp/dip deposit
# - dip withdraw

dfx stop
dfx start --background --clean &> /dev/null

# config
export AKITA_SUPPLY=100000000;
export ICP_SUPPLY=100000000;
export DIP20_FEE=100;
export DIP_DEPOSIT=10000;
export ICP_FEE=10000;
export ICP_DEPOSIT=1000000;

# DIP deposit expected test results
export DIP_DEPOSIT_DEX_BALANCE=$(($DIP_DEPOSIT))
# needs to pay fee two times. approve and transfer 
export DIP_DEPOSIT_DIP_BALANCE=$(($AKITA_SUPPLY-2*$DIP20_FEE-$DIP_DEPOSIT))  
# ICP deposit expected test results
export ICP_DEPOSIT_DEX_BALANCE=$(($ICP_DEPOSIT-2*$ICP_FEE))
export ICP_DEPOSIT_ICP_BALANCE=$(($ICP_SUPPLY-2*$ICP_FEE-$ICP_DEPOSIT))
# DIP withdraw expected test results
export DIP_WITHDAW=1000;
export DIP_WITHDAW_DIP_BALANCE=$(($AKITA_SUPPLY-3*$DIP20_FEE+$DIP_WITHDAW-$DIP_DEPOSIT))  

#setup environment
$TEST_DIR/setup.sh $AKITA_SUPPLY $ICP_SUPPLY

# environment vars for tests
export DEX_PRINCIPLE=$(dfx canister --no-wallet id defi_dapp)
export AKITA_ID=$(dfx canister --no-wallet id AkitaDIP20)

#run tests
$TEST_DIR/test-single-user.sh

