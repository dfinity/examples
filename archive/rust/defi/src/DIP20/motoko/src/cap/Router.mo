// This is a generated Motoko binding.
// Please use `import service "ic:canister_id"` instead to call canisters on the IC if possible.

module {
  public type GetIndexCanistersResponse = {
    witness : ?Witness;
    canisters : [Principal];
  };
  public type GetTokenContractRootBucketArg = {
    witness : Bool;
    canister : Principal;
  };
  public type GetTokenContractRootBucketResponse = {
    witness : ?Witness;
    canister : ?Principal;
  };
  public type GetUserRootBucketsArg = { user : Principal; witness : Bool };
  public type GetUserRootBucketsResponse = {
    witness : ?Witness;
    contracts : [Principal];
  };
  public type WithWitnessArg = { witness : Bool };
  public type Witness = { certificate : [Nat8]; tree : [Nat8] };
  public type Self = actor {
    deploy_plug_bucket : shared (Principal, Nat64) -> async ();
    get_index_canisters : shared query WithWitnessArg -> async GetIndexCanistersResponse;
    get_token_contract_root_bucket : shared query GetTokenContractRootBucketArg -> async GetTokenContractRootBucketResponse;
    get_user_root_buckets : shared query GetUserRootBucketsArg -> async GetUserRootBucketsResponse;
    insert_new_users : shared (Principal, [Principal]) -> async ();
    install_bucket_code : shared Principal -> async ();
    trigger_upgrade : shared () -> async ();
  }
}
