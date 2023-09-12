module CkBtcLedgerTypes {

  public type Actor = actor {
    get_transactions : (GetTransactionsRequest) -> async GetTransactionsResponse;
  };

  public type Memo = Blob;

  public type Account = { owner : Principal; subaccount : ?Blob };

  public type Mint = {
    amount : Nat;
    to : Account;
    memo : ?Memo;
    created_at_time : ?Nat64;
  };

  public type Burn = {
    amount : Nat;
    from : Account;
    memo : ?Memo;
    created_at_time : ?Nat64;
  };

  public type Transfer = {
    amount : Nat;
    from : Account;
    to : Account;
    memo : ?Memo;
    fee : ?Nat;
    created_at_time : ?Nat64;
  };

  public type Transaction = {
    kind : Text;
    mint : ?Mint;
    burn : ?Burn;
    transfer : ?Transfer;
    timestamp : Nat64;
  };

  public type GetTransactionsResponse = {
    log_length : Nat;
    first_index : Nat;
    transactions : [Transaction];
  };

  public type GetTransactionsRequest = {
    start : Nat;
    length : Nat;
  };
};
