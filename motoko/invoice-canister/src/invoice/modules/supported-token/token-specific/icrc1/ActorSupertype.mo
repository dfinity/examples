import Types "./Types"

/****Contains the actor supertype of an ICRC1 token-ledger canister.***/
module Supertype {

  /****Actor supertype of an ICRC1 token-ledger canister providing  
    the methods the Invoice Canister uses to process ICRC1 transactions.***/
  public type Actor = actor {
    icrc1_transfer : shared Types.TransferArgs -> async Types.TransferResult;
    icrc1_balance_of : shared query Types.BalanceArgs -> async Types.Tokens;
  };
};
