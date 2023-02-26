/****ICRC1 token-ledger canister related types.**  
  Comments copied in part from https://github.com/dfinity/ic/blob/master/rs/rosetta-api/icrc1/ledger/icrc1.did   
  _**Note** both the ICRC1 adapter and actor supertype are dependent on these type declarations._  */
module ICRC1 {

  /** Address type of an ICRC1 token-ledger canister.  */
  public type Account = { owner : Principal; subaccount : ?Subaccount };

  /** Subaccount is an arbitrary 32-byte byte array that enables one principal to have 
    multiple unique ICRC1 addresses.  */
  public type Subaccount = Blob;

  /** An arbitrary blob no longer than 32 bytes associated with a transaction.  
    The caller can set it in a `icrc1_transfer` call as a correlation identifier.  */
  public type Memo = Blob;

  /**  Number of nanoseconds from the UNIX epoch in UTC timezone.  */
  public type Timestamp = Nat64;

  /** Sequence number of transactions processed by the ICRC1 token-ledger canister.  */
  public type TxIndex = Nat;

  /** Amount of tokens, measured 10^(decimal number) defined by the ICRC1 token-ledger canister interface.  */
  public type Tokens = Nat;

  /** Arguments for the `icrc1_balance_of` call.  */
  public type BalanceArgs = Account;

  /** Arguments for the `icrc1_transfer` call.  */
  public type TransferArgs = {
    from_subaccount : ?Subaccount;
    to : Account;
    amount : Tokens;
    fee : ?Tokens;
    memo : ?Memo;
    created_at_time : ?Timestamp;
  };

  /** ICRC1 token-ledger specific Result type, note the o and e of Ok and Err are capitalized.  */
  public type Result<T, E> = { #Ok : T; #Err : E };

  /** Result type returned from a `icrc1_transfer call.  */
  public type TransferResult = Result<Tokens, TransferError>;

  /** Set of err result types returned from an unsuccessful `transfer` call due duplicated transfer requests.  */
  public type DeduplicationError = {
    #TooOld;
    #Duplicate : { duplicate_of : TxIndex };
    #CreatedInFuture : { ledger_time : Timestamp };
  };

  /** Set of err result types returned from an unsuccessful `transfer` call due to usual problems.  */
  public type CommonError = {
    #InsufficientFunds : { balance : Tokens };
    #BadFee : { expected_fee : Tokens };
    #TemporarilyUnavailable;
    #GenericError : { error_code : Nat; message : Text };
  };

  /** Error Err type returned from an unsuccessful `transfer` call.  */
  public type TransferError = DeduplicationError or CommonError or {
    // In case the invoice canister would be used to directly manage a token-ledger canister.
    #BadBurn : { min_burn_amount : Tokens };
  };
};
