#!/usr/local/bin/ic-repl


identity default "~/.config/dfx/identity/default/identity.pem";
import AkitaDIP20="${AKITA_ID}";
import DEX="${DEX_PRINCIPLE}";
import LEDGER="${LEDGER}" as "../src/ledger/ledger.public.did";

// configurable values
// need to change assert values based on defined config values
// ic-repl does not support arithemtics
let dip20_fee=${DIP20_FEE};
let dip20_deposit=${DIP_DEPOSIT};
let icp_fee=${ICP_FEE};
let icp_deposit=${ICP_DEPOSIT};
// dip20 deposit test
let dip20_deposit_dex_balance=${DIP_DEPOSIT_DEX_BALANCE};
let dip20_deposit_dip_balance=${DIP_DEPOSIT_DIP_BALANCE};
// ICP deposit test
let icp_deposit_dex_balance=${ICP_DEPOSIT_DEX_BALANCE};
let icp_deposit_icp_balance=${ICP_DEPOSIT_ICP_BALANCE};
// DIP withdraw test
let dip20_withdraw=${DIP_WITHDAW};
let dip20_withdraw_dip_balance=${DIP_WITHDAW_DIP_BALANCE};


"Moving DIP20 tokens to DEX";
let _ =call AkitaDIP20.setFee(
  dip20_fee
);

let _ = call AkitaDIP20.balanceOf(
  default
);

let _ =call AkitaDIP20.approve(
  principal "${DEX_PRINCIPLE}", dip20_deposit
);

let _ = call AkitaDIP20.balanceOf(
  default
);

let _ = call DEX.deposit(
  principal "${AKITA_ID}"
);

let status = call DEX.balance(
  principal "${AKITA_ID}"
);

assert status ~= (dip20_deposit_dex_balance : nat);

let status = call AkitaDIP20.balanceOf(
  default
);

assert status ~= (dip20_deposit_dip_balance : nat);

"DIP20 TRANSFER OK!";

"Moving ICP tokens to DEX";
let deposit_addr =call DEX.deposit_address();
let status = call LEDGER.transfer(record { 
    amount = record { e8s = icp_deposit }; 
    to = deposit_addr;
    fee = record { e8s = icp_fee}; 
    memo = 1;}
);

let status = call DEX.deposit(
  principal "${LEDGER}"
);

assert status.Ok ~= (icp_deposit_dex_balance : nat);

// this test does not work. account(default) has wrong formatting
// tried: "encode (default)","encode (account(default))". no luck
//let status = call LEDGER.account_balance(record { 
//    account = account(default);
//});
//assert status.Ok ~= (icp_deposit_icp_balance : nat);

"ICP TRANSFER OK!";

"DIP WITHDRAW";

let _ = call DEX.withdraw(
  principal "${AKITA_ID}",dip20_withdraw
);

let status = call AkitaDIP20.balanceOf(
  default
);

assert status ~= (dip20_withdraw_dip_balance : nat);

"DIP WITHDRAW OK!";