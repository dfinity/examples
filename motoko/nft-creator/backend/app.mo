// backend/app.mo
// NFT Canister implementing ICRC-7 standard with minting functionality

// --- Standard Library Imports ---
import Principal "mo:base/Principal";
import Nat "mo:base/Nat";
import D "mo:base/Debug";

// --- Third-Party/External Imports ---
import Vec "mo:vector";
import ICRC7 "mo:icrc7-mo";
import ClassPlus "mo:class-plus";

// --- Local Imports ---
import DefaultConfig "defaultConfig";

// --- Actor Definition ---
shared (init_msg) persistent actor class NftCanister() : async (ICRC7.Service.Service) = this {

  // --- Initialization ---
  transient let initManager = ClassPlus.ClassPlusInitializationManager(
    init_msg.caller,
    Principal.fromActor(this),
    true,
  );

  var icrc7_migration_state = ICRC7.initialState();

  private func get_icrc7_environment() : ICRC7.Environment {
    {
      add_ledger_transaction = null;
      can_mint = null;
      can_burn = null;
      can_transfer = null;
      can_update = null;
    };
  };

  transient let icrc7 = ICRC7.Init<system>({
    manager = initManager;
    initialState = icrc7_migration_state;
    args = DefaultConfig.defaultConfig(init_msg.caller);
    pullEnvironment = ?get_icrc7_environment;
    onInitialize = null;
    onStorageChange = func(new_state : ICRC7.State) {
      icrc7_migration_state := new_state;
    };
  });

  // --- Query Calls ---

  public query func icrc7_symbol() : async Text {
    switch (icrc7().get_ledger_info().symbol) {
      case (?val) val;
      case (null) "";
    };
  };

  public query func icrc7_name() : async Text {
    switch (icrc7().get_ledger_info().name) {
      case (?val) val;
      case (null) "";
    };
  };

  public query func icrc7_description() : async ?Text {
    icrc7().get_ledger_info().description;
  };

  public query func icrc7_logo() : async ?Text {
    icrc7().get_ledger_info().logo;
  };

  public query func icrc7_max_memo_size() : async ?Nat {
    ?icrc7().get_ledger_info().max_memo_size;
  };

  public query func icrc7_tx_window() : async ?Nat {
    ?icrc7().get_ledger_info().tx_window;
  };

  public query func icrc7_permitted_drift() : async ?Nat {
    ?icrc7().get_ledger_info().permitted_drift;
  };

  public query func icrc7_total_supply() : async Nat {
    icrc7().get_stats().nft_count;
  };

  public query func icrc7_supply_cap() : async ?Nat {
    icrc7().get_ledger_info().supply_cap;
  };

  public query func icrc7_max_query_batch_size() : async ?Nat {
    icrc7().max_query_batch_size();
  };

  public query func icrc7_max_update_batch_size() : async ?Nat {
    icrc7().max_update_batch_size();
  };

  public query func icrc7_default_take_value() : async ?Nat {
    icrc7().default_take_value();
  };

  public query func icrc7_max_take_value() : async ?Nat {
    icrc7().max_take_value();
  };

  public query func icrc7_atomic_batch_transfers() : async ?Bool {
    icrc7().atomic_batch_transfers();
  };

  public query func icrc7_collection_metadata() : async [(Text, ICRC7.Value)] {
    let ledger_info = icrc7().collection_metadata();
    let results = Vec.new<(Text, ICRC7.Value)>();
    Vec.addFromIter(results, ledger_info.vals());
    Vec.toArray(results);
  };

  public query func icrc7_token_metadata(token_ids : [Nat]) : async [?[(Text, ICRC7.Value)]] {
    icrc7().token_metadata(token_ids);
  };

  public query func icrc7_owner_of(token_ids : ICRC7.Service.OwnerOfRequest) : async ICRC7.Service.OwnerOfResponse {
    switch (icrc7().get_token_owners(token_ids)) {
      case (#ok(val)) val;
      case (#err(err)) D.trap(err);
    };
  };

  public query func icrc7_balance_of(accounts : ICRC7.Service.BalanceOfRequest) : async ICRC7.Service.BalanceOfResponse {
    icrc7().balance_of(accounts);
  };

  public query func icrc7_tokens(prev : ?Nat, take : ?Nat) : async [Nat] {
    icrc7().get_tokens_paginated(prev, take);
  };

  public query func icrc7_tokens_of(account : ICRC7.Account, prev : ?Nat, take : ?Nat) : async [Nat] {
    icrc7().get_tokens_of_paginated(account, prev, take);
  };

  public query func icrc10_supported_standards() : async ICRC7.SupportedStandards {
    [
      { name = "ICRC-7"; url = "https://github.com/dfinity/ICRC/ICRCs/ICRC-7" },
      {
        name = "ICRC-10";
        url = "https://github.com/dfinity/ICRC/ICRCs/ICRC-10";
      },
    ];
  };

  public query func collectionHasBeenClaimed() : async Bool {
    hasBeenClaimed;
  };

  public query func getCollectionOwner() : async Principal {
    icrc7().get_collection_owner();
  };

  // --- Update Calls ---

  public shared (msg) func icrc7_transfer<system>(args : [ICRC7.Service.TransferArg]) : async [?ICRC7.Service.TransferResult] {
    icrc7().transfer<system>(msg.caller, args);
  };

  var hasBeenClaimed = false;

  public shared (msg) func claimCollection() : async () {
    if (hasBeenClaimed) {
      return;
    };
    ignore icrc7().update_ledger_info([#UpdateOwner(msg.caller)]);
    hasBeenClaimed := true;
  };

  // --- Custom NFT Minting Example ---

  var nextTokenId = 0;

  public shared (msg) func mint(to : ICRC7.Account) : async [ICRC7.SetNFTResult] {
    let setNftRequest : ICRC7.SetNFTItemRequest = {
      token_id = nextTokenId;
      metadata = #Map([("tokenUri", #Text(DefaultConfig.tokenURI))]);
      owner = ?to;
      override = false;
      memo = null;
      created_at_time = null;
    };

    switch (icrc7().set_nfts<system>(msg.caller, [setNftRequest], true)) {
      case (#ok(val)) {
        nextTokenId += 1;
        val;
      };
      case (#err(err)) D.trap(err);
    };
  };

};
