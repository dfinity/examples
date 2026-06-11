import Types "Types";
import Blob "mo:core/Blob";

module {
  type Cycles = Types.Cycles;
  type Satoshi = Types.Satoshi;
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type GetUtxosResponse = Types.GetUtxosResponse;
  type MillisatoshiPerVByte = Types.MillisatoshiPerVByte;

  // The fees for the various Bitcoin endpoints.
  let GET_BALANCE_COST_CYCLES : Cycles = 100_000_000;
  let GET_UTXOS_COST_CYCLES : Cycles = 10_000_000_000;
  let GET_CURRENT_FEE_PERCENTILES_COST_CYCLES : Cycles = 100_000_000;
  let SEND_TRANSACTION_BASE_COST_CYCLES : Cycles = 5_000_000_000;
  let SEND_TRANSACTION_COST_CYCLES_PER_BYTE : Cycles = 20_000_000;

  // Use an inline actor type with the full Network variant (including #regtest).
  // mo:ic@4.0.0 defines BitcoinNetwork as { #mainnet; #testnet } only — passing
  // #regtest through it would be a type error, and the local Bitcoin subnet
  // rejects calls made with the wrong network mode.
  type ManagementCanisterActor = actor {
    bitcoin_get_balance : {
      address : BitcoinAddress;
      network : Network;
      min_confirmations : ?Nat32;
    } -> async Satoshi;

    bitcoin_get_utxos : {
      address : BitcoinAddress;
      network : Network;
      filter : ?Types.UtxosFilter;
    } -> async {
      utxos : [Types.Utxo];
      tip_block_hash : Blob;   // Blob from mgmt canister; converted to [Nat8] below
      tip_height : Nat32;
      next_page : ?Blob;       // Blob from mgmt canister; converted to [Nat8] below
    };

    bitcoin_get_current_fee_percentiles : {
      network : Network;
    } -> async [MillisatoshiPerVByte];

    bitcoin_send_transaction : {
      network : Network;
      transaction : Blob;
    } -> async ();
  };

  let management_canister_actor : ManagementCanisterActor = actor ("aaaaa-aa");

  /// Returns the balance of the given Bitcoin address.
  ///
  /// Relies on the `bitcoin_get_balance` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
  public func get_balance(network : Network, address : BitcoinAddress) : async Satoshi {
    await (with cycles = GET_BALANCE_COST_CYCLES) management_canister_actor.bitcoin_get_balance({
      address;
      network;
      min_confirmations = null;
    });
  };

  /// Returns the UTXOs of the given Bitcoin address.
  ///
  /// NOTE: Relies on the `bitcoin_get_utxos` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
  public func get_utxos(network : Network, address : BitcoinAddress) : async GetUtxosResponse {
    let result = await (with cycles = GET_UTXOS_COST_CYCLES) management_canister_actor.bitcoin_get_utxos({
      address;
      network;
      filter = null;
    });
    {
      utxos = result.utxos;
      tip_block_hash = result.tip_block_hash.toArray();
      tip_height = result.tip_height;
      next_page = switch (result.next_page) {
        case null null;
        case (?p) ?p.toArray();
      };
    };
  };

  /// Returns the 100 fee percentiles measured in millisatoshi/vbyte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  ///
  /// Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
  public func get_current_fee_percentiles(network : Network) : async [MillisatoshiPerVByte] {
    await (with cycles = GET_CURRENT_FEE_PERCENTILES_COST_CYCLES) management_canister_actor.bitcoin_get_current_fee_percentiles({
      network;
    });
  };

  /// Sends a (signed) transaction to the Bitcoin network.
  ///
  /// Relies on the `bitcoin_send_transaction` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
  public func send_transaction(network : Network, transaction : [Nat8]) : async () {
    await (with cycles = SEND_TRANSACTION_BASE_COST_CYCLES + transaction.size() * SEND_TRANSACTION_COST_CYCLES_PER_BYTE) management_canister_actor.bitcoin_send_transaction({
      network;
      transaction = Blob.fromArray(transaction);
    });
  };
}
