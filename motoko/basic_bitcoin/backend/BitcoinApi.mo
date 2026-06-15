import Types "Types";
import Blob "mo:core/Blob";

module {
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type Satoshi = Types.Satoshi;
  type MillisatoshiPerByte = Types.MillisatoshiPerByte;

  // Actor type matching the official Bitcoin canister Candid interface.
  // See: https://github.com/dfinity/bitcoin-canister/blob/master/canister/candid.did
  //
  // The Bitcoin canister is deployed at two well-known principals:
  //   - Testnet/Regtest: g4xu7-jiaaa-aaaan-aaaaq-cai
  //   - Mainnet:         ghsi2-tqaaa-aaaan-aaaca-cai
  type BitcoinCanister = actor {
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

    bitcoin_get_block_headers : {
      network : Network;
      start_height : Nat32;
      end_height : ?Nat32;
    } -> async {
      tip_height : Nat32;
      block_headers : [Blob];
    };

    bitcoin_send_transaction : {
      network : Network;
      transaction : Blob;
    } -> async ();

    get_blockchain_info : () -> async {
      height : Nat32;
      block_hash : Blob;
      timestamp : Nat32;
      difficulty : Nat;
      utxos_length : Nat64;
    };
  };

  /// Returns the Bitcoin canister actor for the given network.
  func bitcoinCanister(network : Network) : BitcoinCanister {
    let id = switch network {
      case (#mainnet) "ghsi2-tqaaa-aaaan-aaaca-cai";
      case (#testnet or #regtest) "g4xu7-jiaaa-aaaan-aaaaq-cai";
    };
    actor (id) : BitcoinCanister;
  };

  /// Returns the balance of the given Bitcoin address.
  public func get_balance(network : Network, address : BitcoinAddress) : async Satoshi {
    await (with cycles = 100_000_000) bitcoinCanister(network).bitcoin_get_balance({
      address;
      network;
      min_confirmations = null;
    });
  };

  /// Returns the UTXOs of the given Bitcoin address.
  public func get_utxos(network : Network, address : BitcoinAddress) : async Types.GetUtxosResponse {
    await (with cycles = 10_000_000_000) bitcoinCanister(network).bitcoin_get_utxos({
      address;
      network;
      filter = null;
    });
  };

  /// Returns the 100 fee percentiles measured in millisatoshi/byte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  public func get_current_fee_percentiles(network : Network) : async [MillisatoshiPerByte] {
    await (with cycles = 100_000_000) bitcoinCanister(network).bitcoin_get_current_fee_percentiles({
      network;
    });
  };

  /// Returns Bitcoin block headers for the given height range.
  public func get_block_headers(network : Network, start_height : Nat32, end_height : ?Nat32) : async {
    tip_height : Nat32;
    block_headers : [Blob];
  } {
    await (with cycles = 100_000_000) bitcoinCanister(network).bitcoin_get_block_headers({
      network;
      start_height;
      end_height;
    });
  };

  /// Returns a summary of the current Bitcoin blockchain state.
  public func get_blockchain_info(network : Network) : async {
    height : Nat32;
    block_hash : Blob;
    timestamp : Nat32;
    difficulty : Nat;
    utxos_length : Nat64;
  } {
    await (with cycles = 100_000_000) bitcoinCanister(network).get_blockchain_info();
  };

  /// Sends a signed Bitcoin transaction to the network.
  public func send_transaction(network : Network, transaction : [Nat8]) : async () {
    let cost = 5_000_000_000 + transaction.size() * 20_000_000;
    await (with cycles = cost) bitcoinCanister(network).bitcoin_send_transaction({
      network;
      transaction = Blob.fromArray(transaction);
    });
  };
}
