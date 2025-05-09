module {
  public type DetailValue = {
    #I64 : Int64;
    #U64 : Nat64;
    #Vec : [DetailValue];
    #Slice : [Nat8];
    #Text : Text;
    #True;
    #False;
    #Float : Float;
    #Principal : Principal;
  };
  public type Event = {
    time : Nat64;
    operation : Text;
    details : [(Text, DetailValue)];
    caller : Principal;
  };
  public type GetBucketResponse = { witness : ?Witness; canister : Principal };
  public type GetNextCanistersResponse = {
    witness:?Witness;
    canisters:[Principal];
  };
  public type GetTransactionResponse = {
    #Delegate : (Principal, ?Witness);
    #Found : (?Event, ?Witness);
  };
  public type GetTransactionsArg = { page : ?Nat32; witness : Bool };
  public type GetTransactionsResponseBorrowed = {
    data : [Event];
    page : Nat32;
    witness : ?Witness;
  };
  public type GetUserTransactionsArg = {
    page : ?Nat32;
    user : Principal;
    witness : Bool;
  };
  public type IndefiniteEvent = {
    operation : Text;
    details : [(Text, DetailValue)];
    caller : Principal;
  };
  public type WithIdArg = { id : Nat64; witness : Bool };
  public type WithWitnessArg = { witness : Bool };
  public type Witness = { certificate : [Nat8]; tree : [Nat8] };
  public type Self = actor {
    cap_bypass_rate_limit : shared () -> async ();
    contract_id : shared query () -> async Principal;
    get_bucket_for : shared query WithIdArg -> async GetBucketResponse;
    get_next_canisters : shared query WithWitnessArg -> async GetNextCanistersResponse;
    get_transaction : shared query WithIdArg -> async GetTransactionResponse;
    get_transactions : shared query GetTransactionsArg -> async GetTransactionsResponseBorrowed;
    get_user_transactions : shared query GetUserTransactionsArg -> async GetTransactionsResponseBorrowed;
    insert : shared IndefiniteEvent -> async Nat64;
    migrate : shared [Event] -> async ();
    size : shared query () -> async Nat64;
    time : shared query () -> async Nat64;
  }
}
