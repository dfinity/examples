import Types "Types";
import Array "mo:core/Array";
import Blob "mo:core/Blob";
import { ic } "mo:ic";

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

  // Maps our local Network type (which includes #regtest) to the mo:ic BitcoinNetwork
  // type (which only has #mainnet and #testnet). Regtest is treated as testnet for
  // API calls since the local replica's bitcoin integration is equivalent.
  func toIcNetwork(network : Network) : { #mainnet; #testnet } {
    switch network {
      case (#mainnet) #mainnet;
      case (#testnet) #testnet;
      case (#regtest) #testnet;
    };
  };

  /// Returns the balance of the given Bitcoin address.
  ///
  /// Relies on the `bitcoin_get_balance` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
  public func get_balance(network : Network, address : BitcoinAddress) : async Satoshi {
    await (with cycles = GET_BALANCE_COST_CYCLES) ic.bitcoin_get_balance({
        address;
        network = toIcNetwork(network);
        min_confirmations = null;
    })
  };

  /// Returns the UTXOs of the given Bitcoin address.
  ///
  /// NOTE: Relies on the `bitcoin_get_utxos` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
  public func get_utxos(network : Network, address : BitcoinAddress) : async GetUtxosResponse {
    let result = await (with cycles = GET_UTXOS_COST_CYCLES) ic.bitcoin_get_utxos({
        address;
        network = toIcNetwork(network);
        filter = null;
    });
    // Convert mo:ic result types to our local types.
    // Blobs are converted to [Nat8] arrays, and Outpoint is mapped to OutPoint.
    {
      utxos = result.utxos.map(func(u : { height : Nat32; value : Satoshi; outpoint : { txid : Blob; vout : Nat32 } }) : Types.Utxo {
        {
          outpoint = { txid = u.outpoint.txid; vout = u.outpoint.vout };
          value = u.value;
          height = u.height;
        }
      });
      tip_block_hash = result.tip_block_hash.toArray();
      tip_height = result.tip_height;
      next_page = switch (result.next_page) {
        case null null;
        case (?p) ?p.toArray();
      };
    }
  };

  /// Returns the 100 fee percentiles measured in millisatoshi/vbyte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  ///
  /// Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
  public func get_current_fee_percentiles(network : Network) : async [MillisatoshiPerVByte] {
    await (with cycles = GET_CURRENT_FEE_PERCENTILES_COST_CYCLES) ic.bitcoin_get_current_fee_percentiles({
        network = toIcNetwork(network);
    })
  };

  /// Sends a (signed) transaction to the Bitcoin network.
  ///
  /// Relies on the `bitcoin_send_transaction` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
  public func send_transaction(network : Network, transaction : [Nat8]) : async () {
    await (with cycles = SEND_TRANSACTION_BASE_COST_CYCLES + transaction.size() * SEND_TRANSACTION_COST_CYCLES_PER_BYTE) ic.bitcoin_send_transaction({
        network = toIcNetwork(network);
        transaction = Blob.fromArray(transaction);
    })
  };
}
