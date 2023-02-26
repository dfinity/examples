/****ICP ledger canister related types.**  
  Comments are copied from https://github.com/dfinity/ic/blob/master/rs/rosetta-api/icp_ledger/ledger.did  
  _**Note** both the ICP adapter and actor supertype are dependent on these type declarations._  */
module ICP {

  /** AccountIdentifier is a 32-byte array.  
   The first 4 bytes is big-endian encoding of a CRC32 checksum of the last 28 bytes. */
  public type AccountIdentifier = Blob;

  /** Subaccount is an arbitrary 32-byte byte array.  
    Ledger uses subaccounts to compute the source address, which enables one  
    principal to control multiple ledger accounts.   */
  public type Subaccount = Blob;

  /** Amount of tokens, measured in 10^-8 of a token.  */
  public type Tokens = { e8s : Nat64 };

  /**  Number of nanoseconds from the UNIX epoch in UTC timezone.  */
  public type Timestamp = { timestamp_nanos : Nat64 };

  /** An arbitrary number associated with a transaction.  
    The caller can set it in a `transfer` call as a correlation identifier.  */
  public type Memo = Nat64;

  /** Sequence number of a block produced by the ledger.  */
  public type BlockIndex = Nat64;

  /** Arguments for the `account_balance` call.  */
  public type AccountBalanceArgs = { account : AccountIdentifier };

  /** Arguments for the `transfer` call.  */
  public type TransferArgs = {
    memo : Memo;
    amount : Tokens;
    fee : Tokens;
    from_subaccount : ?Subaccount;
    to : AccountIdentifier;
    created_at_time : ?Timestamp;
  };

  /** ICP Ledger specific Result type, note the o and e of Ok and Err are capitalized.  */
  public type Result<T, E> = { #Ok : T; #Err : E };

  /** Result type returned from a `transfer` call.  */
  public type TransferResult = Result<BlockIndex, TransferError>;

  /** Error Err type returned from an unsuccessful `transfer` call.  */
  public type TransferError = {
    #BadFee : { expected_fee : Tokens };
    #InsufficientFunds : { balance : Tokens };
    #TxTooOld : { allowed_window_nanos : Nat64 };
    #TxCreatedInFuture;
    #TxDuplicate : { duplicate_of : BlockIndex };
  };
};
