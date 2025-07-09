import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Array "mo:base/Array";
import Blob "mo:base/Blob";

import BitcoinApi "BitcoinApi";
import P2pkh "P2pkh";
import P2trKeyOnly "P2trKeyOnly";
import P2tr "P2tr";
import Types "Types";
import Utils "Utils";

actor class BasicBitcoin(network : Types.Network) {
  type GetUtxosResponse = Types.GetUtxosResponse;
  type MillisatoshiPerVByte = Types.MillisatoshiPerVByte;
  type SendRequest = Types.SendRequest;
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type Satoshi = Types.Satoshi;
  type TransactionId = Text;
  type EcdsaCanisterActor = Types.EcdsaCanisterActor;
  type SchnorrCanisterActor = Types.SchnorrCanisterActor;
  type P2trDerivationPaths = Types.P2trDerivationPaths;

  /// The Bitcoin network to connect to.
  ///
  /// When developing locally this should be `regtest`.
  /// When deploying to the IC this should be `testnet`.
  /// `mainnet` is currently unsupported.
  stable let NETWORK : Network = network;

  /// The derivation path to use for ECDSA secp256k1 or Schnorr BIP340/BIP341 key
  /// derivation.
  let DERIVATION_PATH : [[Nat8]] = [];

  // The ECDSA key name.
  let KEY_NAME : Text = switch NETWORK {
    // For local development, we use a special test key with dfx.
    case (#regtest) "dfx_test_key";
    // On the IC we're using a test ECDSA key.
    case _ "test_key_1";
  };

  // Threshold signing APIs instantiated with the management canister ID. Can be
  // replaced for cheaper testing.
  var ecdsa_canister_actor : EcdsaCanisterActor = actor ("aaaaa-aa");
  var schnorr_canister_actor : SchnorrCanisterActor = actor ("aaaaa-aa");

  /// Returns the balance of the given Bitcoin address.
  public func get_balance(address : BitcoinAddress) : async Satoshi {
    await BitcoinApi.get_balance(NETWORK, address);
  };

  /// Returns the UTXOs of the given Bitcoin address.
  public func get_utxos(address : BitcoinAddress) : async GetUtxosResponse {
    await BitcoinApi.get_utxos(NETWORK, address);
  };

  /// Returns the 100 fee percentiles measured in millisatoshi/vbyte.
  /// Percentiles are computed from the last 10,000 transactions (if available).
  public func get_current_fee_percentiles() : async [MillisatoshiPerVByte] {
    await BitcoinApi.get_current_fee_percentiles(NETWORK);
  };

  /// Returns the P2PKH address of this canister at a specific derivation path.
  public func get_p2pkh_address() : async BitcoinAddress {
    await P2pkh.get_address(ecdsa_canister_actor, NETWORK, KEY_NAME, p2pkhDerivationPath());
  };

  /// Sends the given amount of bitcoin from this canister to the given address.
  /// Returns the transaction ID.
  public func send_from_p2pkh_address(request : SendRequest) : async TransactionId {
    Utils.bytesToText(await P2pkh.send(ecdsa_canister_actor, NETWORK, p2pkhDerivationPath(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  public func get_p2tr_key_only_address() : async BitcoinAddress {
    await P2trKeyOnly.get_address_key_only(schnorr_canister_actor, NETWORK, KEY_NAME, p2trKeyOnlyDerivationPath());
  };

  public func send_from_p2tr_key_only_address(request : SendRequest) : async TransactionId {
    Utils.bytesToText(await P2trKeyOnly.send(schnorr_canister_actor, NETWORK, p2trKeyOnlyDerivationPath(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  public func get_p2tr_address() : async BitcoinAddress {
    await P2tr.get_address(schnorr_canister_actor, NETWORK, KEY_NAME, p2trDerivationPaths());
  };

  public func send_from_p2tr_address_key_path(request : SendRequest) : async TransactionId {
    Utils.bytesToText(await P2tr.send_key_path(schnorr_canister_actor, NETWORK, p2trDerivationPaths(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  public func send_from_p2tr_address_script_path(request : SendRequest) : async TransactionId {
    Utils.bytesToText(await P2tr.send_script_path(schnorr_canister_actor, NETWORK, p2trDerivationPaths(), KEY_NAME, request.destination_address, request.amount_in_satoshi));
  };

  func p2pkhDerivationPath() : [[Nat8]] {
    derivationPathWithSuffix("p2pkh");
  };

  func p2trKeyOnlyDerivationPath() : [[Nat8]] {
    derivationPathWithSuffix("p2tr_key_only");
  };

  func p2trDerivationPaths() : P2trDerivationPaths {
    {
      key_path_derivation_path = derivationPathWithSuffix("p2tr_internal_key");
      script_path_derivation_path = derivationPathWithSuffix("p2tr_script_key");
    };
  };

  func derivationPathWithSuffix(suffix : Blob) : [[Nat8]] {
    Array.flatten([DERIVATION_PATH, [Blob.toArray(suffix)]]);
  };
};
