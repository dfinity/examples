module ICRC {
  public type BlockIndex = Nat;
  public type Subaccount = Blob;
  // Number of nanoseconds since the UNIX epoch in UTC timezone.
  public type Timestamp = Nat64;
  // Number of nanoseconds between two [Timestamp]s.
  public type Tokens = Nat;
  public type TxIndex = Nat;

  public type Account = {
    owner : Principal;
    subaccount : ?Subaccount;
  };

  public type TransferArg = {
    from_subaccount : ?Subaccount;
    to : Account;
    amount : Tokens;
    fee : ?Tokens;
    memo : ?Blob;
    created_at_time : ?Timestamp;
  };

  public type TransferError = {
    #BadFee : { expected_fee : Tokens };
    #BadBurn : { min_burn_amount : Tokens };
    #InsufficientFunds : { balance : Tokens };
    #TooOld;
    #CreatedInFuture : { ledger_time : Nat64 };
    #TemporarilyUnavailable;
    #Duplicate : { duplicate_of : BlockIndex };
    #GenericError : { error_code : Nat; message : Text };
  };

  public type TransferResult = {
    #Ok : BlockIndex;
    #Err : TransferError;
  };

  // The value returned from the [icrc1_metadata] endpoint.
  public type MetadataValue = {
    #Nat : Nat;
    #Int : Int;
    #Text : Text;
    #Blob : Blob;
  };

  public type ApproveArgs = {
    from_subaccount : ?Subaccount;
    spender : Account;
    amount : Tokens;
    expected_allowance : ?Tokens;
    expires_at : ?Timestamp;
    fee : ?Tokens;
    memo : ?Blob;
    created_at_time : ?Timestamp;
  };

  public type ApproveError = {
    #BadFee : { expected_fee : Tokens };
    #InsufficientFunds : { balance : Tokens };
    #AllowanceChanged : { current_allowance : Tokens };
    #Expired : { ledger_time : Nat64 };
    #TooOld;
    #CreatedInFuture : { ledger_time : Nat64 };
    #Duplicate : { duplicate_of : BlockIndex };
    #TemporarilyUnavailable;
    #GenericError : { error_code : Nat; message : Text };
  };

  public type ApproveResult = {
    #Ok : BlockIndex;
    #Err : ApproveError;
  };

  public type AllowanceArgs = {
    account : Account;
    spender : Account;
  };

  public type Allowance = {
    allowance : Tokens;
    expires_at : ?Timestamp;
  };

  public type TransferFromArgs = {
    spender_subaccount : ?Subaccount;
    from : Account;
    to : Account;
    amount : Tokens;
    fee : ?Tokens;
    memo : ?Blob;
    created_at_time : ?Timestamp;
  };

  public type TransferFromResult = {
    #Ok : BlockIndex;
    #Err : TransferFromError;
  };

  public type TransferFromError = {
    #BadFee : { expected_fee : Tokens };
    #BadBurn : { min_burn_amount : Tokens };
    #InsufficientFunds : { balance : Tokens };
    #InsufficientAllowance : { allowance : Tokens };
    #TooOld;
    #CreatedInFuture : { ledger_time : Nat64 };
    #Duplicate : { duplicate_of : BlockIndex };
    #TemporarilyUnavailable;
    #GenericError : { error_code : Nat; message : Text };
  };

  public type Actor = actor {
    icrc1_name : shared query () -> async Text;
    icrc1_symbol : shared query () -> async Text;
    icrc1_decimals : shared query () -> async Nat8;
    icrc1_metadata : shared query () -> async [(Text, MetadataValue)];
    icrc1_total_supply : shared query () -> async Tokens;
    icrc1_fee : shared query () -> async Tokens;
    icrc1_minting_account : shared query () -> async ?Account;
    icrc1_balance_of : shared query (Account) -> async Tokens;
    icrc1_transfer : shared (TransferArg) -> async TransferResult;
    icrc1_supported_standards : shared query () -> async [{
      name : Text;
      url : Text;
    }];
    icrc2_approve : shared (ApproveArgs) -> async ApproveResult;
    icrc2_allowance : shared query (AllowanceArgs) -> async Allowance;
    icrc2_transfer_from : shared (TransferFromArgs) -> async TransferFromResult;
  };
};
