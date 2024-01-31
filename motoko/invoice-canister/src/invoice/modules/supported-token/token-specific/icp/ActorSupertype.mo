import Types "./Types"

/****Contains an actor supertype of an ICP ledger canister.***/
module Supertype {

  /****Actor supertype of an ICP ledger canister providing the methods  
    the Invoice Canister uses to process ICRC1 transactions.***/
  public type Actor = actor {
    transfer : shared Types.TransferArgs -> async Types.TransferResult;
    account_balance : shared query Types.AccountBalanceArgs -> async Types.Tokens;
  };
};
