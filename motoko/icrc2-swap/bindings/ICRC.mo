// This is a generated Motoko binding.
// Please use `import service "ic:canister_id"` instead to call canisters on the IC if possible.

module {
  public type Account = { owner : Principal; subaccount : ?Subaccount };
  public type Allowance = { allowance : Tokens; expires_at : ?Timestamp };
  public type AllowanceArgs = { account : Account; spender : Account };
  public type ApproveArgs = {
    fee : ?Tokens;
    memo : ?Blob;
    from_subaccount : ?Subaccount;
    created_at_time : ?Timestamp;
    amount : Tokens;
    expected_allowance : ?Tokens;
    expires_at : ?Timestamp;
    spender : Account;
  };
  public type ApproveError = {
    #GenericError : { message : Text; error_code : Nat };
    #TemporarilyUnavailable;
    #Duplicate : { duplicate_of : BlockIndex };
    #BadFee : { expected_fee : Tokens };
    #AllowanceChanged : { current_allowance : Tokens };
    #CreatedInFuture : { ledger_time : Timestamp };
    #TooOld;
    #Expired : { ledger_time : Timestamp };
    #InsufficientFunds : { balance : Tokens };
  };
  public type ApproveResult = { #Ok : BlockIndex; #Err : ApproveError };
  public type BlockIndex = Nat;
  public type MetadataValue = {
    #Int : Int;
    #Nat : Nat;
    #Blob : Blob;
    #Text : Text;
  };
  public type StandardRecord = { url : Text; name : Text };
  public type Subaccount = Blob;
  public type Timestamp = Nat64;
  public type Tokens = Nat;
  public type TransferArg = {
    to : Account;
    fee : ?Tokens;
    memo : ?Blob;
    from_subaccount : ?Subaccount;
    created_at_time : ?Timestamp;
    amount : Tokens;
  };
  public type TransferError = {
    #GenericError : { message : Text; error_code : Nat };
    #TemporarilyUnavailable;
    #BadBurn : { min_burn_amount : Tokens };
    #Duplicate : { duplicate_of : BlockIndex };
    #BadFee : { expected_fee : Tokens };
    #CreatedInFuture : { ledger_time : Timestamp };
    #TooOld;
    #InsufficientFunds : { balance : Tokens };
  };
  public type TransferFromArgs = {
    to : Account;
    fee : ?Tokens;
    spender_subaccount : ?Subaccount;
    from : Account;
    memo : ?Blob;
    created_at_time : ?Timestamp;
    amount : Tokens;
  };
  public type TransferFromError = {
    #GenericError : { message : Text; error_code : Nat };
    #TemporarilyUnavailable;
    #InsufficientAllowance : { allowance : Tokens };
    #BadBurn : { min_burn_amount : Tokens };
    #Duplicate : { duplicate_of : BlockIndex };
    #BadFee : { expected_fee : Tokens };
    #CreatedInFuture : { ledger_time : Timestamp };
    #TooOld;
    #InsufficientFunds : { balance : Tokens };
  };
  public type TransferFromResult = {
    #Ok : BlockIndex;
    #Err : TransferFromError;
  };
  public type TransferResult = { #Ok : BlockIndex; #Err : TransferError };
  public type Self = actor {
    icrc1_balance_of : shared query Account -> async Tokens;
    icrc1_decimals : shared query () -> async Nat8;
    icrc1_fee : shared query () -> async Tokens;
    icrc1_metadata : shared query () -> async [(Text, MetadataValue)];
    icrc1_minting_account : shared query () -> async ?Account;
    icrc1_name : shared query () -> async Text;
    icrc1_supported_standards : shared query () -> async [StandardRecord];
    icrc1_symbol : shared query () -> async Text;
    icrc1_total_supply : shared query () -> async Tokens;
    icrc1_transfer : shared TransferArg -> async TransferResult;
    icrc2_allowance : shared query AllowanceArgs -> async Allowance;
    icrc2_approve : shared ApproveArgs -> async ApproveResult;
    icrc2_transfer_from : shared TransferFromArgs -> async TransferFromResult;
  }
}
