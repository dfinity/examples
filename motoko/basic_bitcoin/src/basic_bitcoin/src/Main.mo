import BitcoinWallet "BitcoinWallet";
import BitcoinApi "BitcoinApi";
import Types "Types";

actor BasicBitcoin {
  type GetUtxosResponse = Types.GetUtxosResponse;
  type MillisatoshiPerByte = Types.MillisatoshiPerByte;
  type SendRequest = Types.SendRequest;
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type Satoshi = Types.Satoshi;

  // The Bitcoin network to connect to.
  //
  // When developing locally this should be `Regtest`.
  // When deploying to the IC this should be `Testnet` or `Mainnet`.
  let NETWORK : Network = #Regtest;

  // The derivation path to use for ECDSA secp256k1.
  let DERIVATION_PATH : [[Nat8]] = [];

  /// Returns the balance of the given Bitcoin address.
  public func get_balance(address : BitcoinAddress) : async Satoshi {
    await BitcoinApi.get_balance(NETWORK, address)
  };

  /// Returns the UTXOs of the given Bitcoin address.
  public func get_utxos(address : BitcoinAddress) : async GetUtxosResponse {
    await BitcoinApi.get_utxos(NETWORK, address)
  };

  /// Returns the 100 fee percentiles measured in millisatoshi/byte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  public func get_current_fee_percentiles() : async [MillisatoshiPerByte] {
    await BitcoinApi.get_current_fee_percentiles(NETWORK)
  };

  /// Tried with https://tbtc.bitaps.com/raw/transaction/b5dd8f5c05d4e99a59291fce4965bdcd239b78aba1ce5d70e4b08c0b7c219400
  /// but results in `bitcoin_send_transaction failed: Can't deserialize transaction because it's malformed.`
  /// Sends a (signed) transaction to the Bitcoin network.
  public func send_transaction(transaction : [Nat8]) : async () {
    await BitcoinApi.send_transaction(NETWORK, transaction)
  };

  /// Returns the P2PKH address of this canister at a specific derivation path.
  public func get_p2pkh_address() : async BitcoinAddress {
    await BitcoinWallet.get_p2pkh_address(NETWORK, DERIVATION_PATH)
  };

  /// Sends the given amount of bitcoin from this canister to the given address.
  public func send(request : SendRequest) : async () {
    await BitcoinWallet.send(NETWORK, DERIVATION_PATH, request.destination_address, request.amount_in_satoshi)
  };
};
