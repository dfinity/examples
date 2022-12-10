import ExperimentalCycles "mo:base/ExperimentalCycles";

import Types "Types";

module {
  type Cycles = Types.Cycles;
  type Satoshi = Types.Satoshi;
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type GetUtxosResponse = Types.GetUtxosResponse;
  type MillisatoshiPerByte = Types.MillisatoshiPerByte;
  type GetBalanceRequest = Types.GetBalanceRequest;
  type GetUtxosRequest = Types.GetUtxosRequest;
  type GetCurrentFeePercentilesRequest = Types.GetCurrentFeePercentilesRequest;
  type SendTransactionRequest = Types.SendTransactionRequest;

  // The fees for the various Bitcoin endpoints.
  let GET_BALANCE_COST_CYCLES : Cycles = 100_000_000;
  let GET_UTXOS_COST_CYCLES : Cycles = 10_000_000_000;
  let GET_CURRENT_FEE_PERCENTILES_COST_CYCLES : Cycles = 100_000_000;
  let SEND_TRANSACTION_BASE_COST_CYCLES : Cycles = 5_000_000_000;
  let SEND_TRANSACTION_COST_CYCLES_PER_BYTE : Cycles = 20_000_000;

  /// Actor definition to handle interactions with the management canister.
  type ManagementCanisterActor = actor {
      bitcoin_get_balance : GetBalanceRequest -> async Satoshi;
      bitcoin_get_utxos : GetUtxosRequest -> async GetUtxosResponse;
      bitcoin_get_current_fee_percentiles : GetCurrentFeePercentilesRequest -> async [MillisatoshiPerByte];
      bitcoin_send_transaction : SendTransactionRequest -> async ();
  };

  let management_canister_actor : ManagementCanisterActor = actor("aaaaa-aa");

  /// Returns the balance of the given Bitcoin address.
  ///
  /// Relies on the `bitcoin_get_balance` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_balance
  public func get_balance(network : Network, address : BitcoinAddress) : async Satoshi {
    ExperimentalCycles.add(GET_BALANCE_COST_CYCLES);
    await management_canister_actor.bitcoin_get_balance({
        address;
        network;
        min_confirmations = null;
    })
  };

  /// Returns the UTXOs of the given Bitcoin address.
  ///
  /// NOTE: Pagination is ignored in this example. If an address has many thousands
  /// of UTXOs, then subsequent calls to `bitcoin_get_utxos` are required.
  ///
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_utxos
  public func get_utxos(network : Network, address : BitcoinAddress) : async GetUtxosResponse {
    ExperimentalCycles.add(GET_UTXOS_COST_CYCLES);
    await management_canister_actor.bitcoin_get_utxos({
        address;
        network;
        filter = null;
    })
  };

  /// Returns the 100 fee percentiles measured in millisatoshi/byte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  ///
  /// Relies on the `bitcoin_get_current_fee_percentiles` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_get_current_fee_percentiles
  public func get_current_fee_percentiles(network : Network) : async [MillisatoshiPerByte] {
    ExperimentalCycles.add(GET_CURRENT_FEE_PERCENTILES_COST_CYCLES);
    await management_canister_actor.bitcoin_get_current_fee_percentiles({
        network;
    })
  };

  /// Sends a (signed) transaction to the Bitcoin network.
  ///
  /// Relies on the `bitcoin_send_transaction` endpoint.
  /// See https://internetcomputer.org/docs/current/references/ic-interface-spec/#ic-bitcoin_send_transaction
  public func send_transaction(network : Network, transaction : [Nat8]) : async () {
    let transaction_fee =
        SEND_TRANSACTION_BASE_COST_CYCLES + transaction.size() * SEND_TRANSACTION_COST_CYCLES_PER_BYTE;

    ExperimentalCycles.add(transaction_fee);
    await management_canister_actor.bitcoin_send_transaction({
        network;
        transaction;
    })
  };
}
