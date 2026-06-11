import Types "Types";
import Blob "mo:core/Blob";

module {
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type Satoshi = Types.Satoshi;
  type MillisatoshiPerByte = Types.MillisatoshiPerByte;

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
    } -> async Types.GetUtxosResponse;

    bitcoin_get_current_fee_percentiles : {
      network : Network;
    } -> async [MillisatoshiPerByte];

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
    await (with cycles = 100_000_000) management_canister_actor.bitcoin_get_balance({
      address;
      network;
      min_confirmations = null;
    });
  };

  /// Returns the UTXOs of the given Bitcoin address.
  ///
  /// Relies on the `bitcoin_get_utxos` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
  public func get_utxos(network : Network, address : BitcoinAddress) : async Types.GetUtxosResponse {
    await (with cycles = 10_000_000_000) management_canister_actor.bitcoin_get_utxos({
      address;
      network;
      filter = null;
    });
  };

  /// Returns the 100 fee percentiles measured in millisatoshi/byte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  ///
  /// Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
  public func get_current_fee_percentiles(network : Network) : async [MillisatoshiPerByte] {
    await (with cycles = 100_000_000) management_canister_actor.bitcoin_get_current_fee_percentiles({
      network;
    });
  };

  /// Sends a (signed) transaction to the Bitcoin network.
  ///
  /// Relies on the `bitcoin_send_transaction` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
  public func send_transaction(network : Network, transaction : [Nat8]) : async () {
    let cost = 5_000_000_000 + transaction.size() * 20_000_000;
    await (with cycles = cost) management_canister_actor.bitcoin_send_transaction({
      network;
      transaction = Blob.fromArray(transaction);
    });
  };
}
