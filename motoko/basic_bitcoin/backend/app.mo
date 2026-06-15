import Array "mo:core/Array";
import Blob "mo:core/Blob";

import BitcoinApi "BitcoinApi";
import P2pkh "P2pkh";
import P2trKeyOnly "P2trKeyOnly";
import P2tr "P2tr";
import Types "Types";
import Utils "Utils";

actor class BasicBitcoin(network : Types.Network) {

  /// The Bitcoin network to connect to.
  /// Passed as an init arg and determined by the environment:
  /// `regtest` (local), `testnet` (staging), or `mainnet` (production).
  let NETWORK : Types.Network = network;

  /// The derivation path to use for ECDSA secp256k1 or Schnorr BIP340/BIP341 key
  /// derivation.
  transient let DERIVATION_PATH : [[Nat8]] = [];

  // The ECDSA/Schnorr key name depends on which Bitcoin network this canister targets:
  //   - "key_1"       — Bitcoin mainnet on ICP mainnet
  //   - "test_key_1"  — Bitcoin testnet4 on ICP mainnet (staging) OR local regtest
  transient let KEY_NAME : Text = switch NETWORK {
    case (#mainnet) "key_1";
    case (#testnet or #regtest) "test_key_1";
  };

  /// Returns the balance of the given Bitcoin address.
  public func get_balance(address : Types.BitcoinAddress) : async Types.Satoshi {
    await BitcoinApi.get_balance(NETWORK, address);
  };

  /// Returns the UTXOs of the given Bitcoin address.
  public func get_utxos(address : Types.BitcoinAddress) : async Types.GetUtxosResponse {
    await BitcoinApi.get_utxos(NETWORK, address);
  };

  /// Returns the 100 fee percentiles measured in millisatoshi per byte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  public func get_current_fee_percentiles() : async [Types.MillisatoshiPerByte] {
    await BitcoinApi.get_current_fee_percentiles(NETWORK);
  };

  /// Returns the P2PKH address of this canister at a specific derivation path.
  public func get_p2pkh_address() : async Types.BitcoinAddress {
    await P2pkh.get_address(NETWORK, KEY_NAME, p2pkhDerivationPath());
  };

  /// Sends the given amount of bitcoin from this canister to the given address.
  /// Returns the transaction ID.
  public func send_from_p2pkh_address(request : Types.SendRequest) : async Text {
    Utils.bytesToText(await P2pkh.send(NETWORK, p2pkhDerivationPath(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  public func get_p2tr_key_only_address() : async Types.BitcoinAddress {
    await P2trKeyOnly.get_address_key_only(NETWORK, KEY_NAME, p2trKeyOnlyDerivationPath());
  };

  public func send_from_p2tr_key_only_address(request : Types.SendRequest) : async Text {
    Utils.bytesToText(await P2trKeyOnly.send(NETWORK, p2trKeyOnlyDerivationPath(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  public func get_p2tr_address() : async Types.BitcoinAddress {
    await P2tr.get_address(NETWORK, KEY_NAME, p2trDerivationPaths());
  };

  public func send_from_p2tr_address_key_path(request : Types.SendRequest) : async Text {
    Utils.bytesToText(await P2tr.send_key_path(NETWORK, p2trDerivationPaths(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  public func send_from_p2tr_address_script_path(request : Types.SendRequest) : async Text {
    Utils.bytesToText(await P2tr.send_script_path(NETWORK, p2trDerivationPaths(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  /// Returns Bitcoin block headers starting at `start_height`.
  /// Optionally limit to `end_height` (inclusive).
  public func get_block_headers(start_height : Nat32, end_height : ?Nat32) : async {
    tip_height : Nat32;
    block_headers : [Blob];
  } {
    await BitcoinApi.get_block_headers(NETWORK, start_height, end_height);
  };

  /// Returns a summary of the current Bitcoin blockchain state: tip height,
  /// tip block hash, timestamp, difficulty, and UTXO set size.
  public func get_blockchain_info() : async {
    height : Nat32;
    block_hash : Blob;
    timestamp : Nat32;
    difficulty : Nat;
    utxos_length : Nat64;
  } {
    await BitcoinApi.get_blockchain_info(NETWORK);
  };

  func p2pkhDerivationPath() : [[Nat8]] {
    derivationPathWithSuffix("p2pkh");
  };

  func p2trKeyOnlyDerivationPath() : [[Nat8]] {
    derivationPathWithSuffix("p2tr_key_only");
  };

  func p2trDerivationPaths() : Types.P2trDerivationPaths {
    {
      key_path_derivation_path = derivationPathWithSuffix("p2tr_internal_key");
      script_path_derivation_path = derivationPathWithSuffix("p2tr_script_key");
    };
  };

  func derivationPathWithSuffix(suffix : Blob) : [[Nat8]] {
    [DERIVATION_PATH, [suffix.toArray()]].flatten();
  };
};
