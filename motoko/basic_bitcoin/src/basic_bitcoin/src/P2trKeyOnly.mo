//! A demo of a very bare-bones Bitcoin "wallet".
//!
//! The wallet here showcases how Bitcoin addresses can be computed
//! and how Bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't P2TR raw key spend.
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.

import Debug "mo:base/Debug";
import Array "mo:base/Array";
import Nat8 "mo:base/Nat8";
import Iter "mo:base/Iter";
import Blob "mo:base/Blob";
import Option "mo:base/Option";

import Hash "mo:bitcoin/Hash";
import { tweakFromKeyAndHash; tweakPublicKey; makeScriptFromP2trKeyAddress } "mo:bitcoin/bitcoin/P2tr";
import Script "mo:bitcoin/bitcoin/Script";
import Transaction "mo:bitcoin/bitcoin/Transaction";
import TxInput "mo:bitcoin/bitcoin/TxInput";

import BitcoinApi "BitcoinApi";
import P2tr "P2tr";
import SchnorrApi "SchnorrApi";
import Types "Types";
import Utils "Utils";

module {
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type Satoshi = Types.Satoshi;
  type Utxo = Types.Utxo;
  type MillisatoshiPerVByte = Types.MillisatoshiPerVByte;
  type Transaction = Transaction.Transaction;
  type Script = Script.Script;
  type SchnorrCanisterActor = Types.SchnorrCanisterActor;

  public func get_address(schnorr_canister_actor : SchnorrCanisterActor, network : Network, key_name : Text, derivation_path : [[Nat8]]) : async BitcoinAddress {
    // Fetch the public key of the given derivation path.
    let sec1_public_key = await SchnorrApi.schnorr_public_key(schnorr_canister_actor, key_name, Array.map(derivation_path, Blob.fromArray));
    assert sec1_public_key.size() == 33;

    let bip340_public_key_bytes = Array.subArray(Blob.toArray(sec1_public_key), 1, 32);

    let merkleRoot = unspendableMerkleRoot(bip340_public_key_bytes);
    let tweak = Utils.get_ok(tweakFromKeyAndHash(bip340_public_key_bytes, merkleRoot));
    let tweaked_public_key = Utils.get_ok(tweakPublicKey(bip340_public_key_bytes, tweak)).bip340_public_key;

    P2tr.tweaked_public_key_to_p2tr_address(network, tweaked_public_key);
  };

  /// Sends a transaction to the network that transfers the given amount to the
  /// given destination, where the source of the funds is the canister itself
  /// at the given derivation path.
  public func send(schnorr_canister_actor : SchnorrCanisterActor, network : Network, derivation_path : [[Nat8]], key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
    // Get fee percentiles from previous transactions to estimate our own fee.
    let fee_percentiles = await BitcoinApi.get_current_fee_percentiles(network);

    let fee_per_vbyte : MillisatoshiPerVByte = if (fee_percentiles.size() == 0) {
      // There are no fee percentiles. This case can only happen on a regtest
      // network where there are no non-coinbase transactions. In this case,
      // we use a default of 1000 millisatoshis/vbyte (i.e. 2 satoshi/byte)
      2000;
    } else {
      // Choose the 50th percentile for sending fees.
      fee_percentiles[50];
    };

    // Fetch our public key, P2TR raw key spend address, and UTXOs.
    let own_address = await get_address(schnorr_canister_actor, network, key_name, derivation_path);
    let own_bip340_public_key = switch (Utils.get_ok(makeScriptFromP2trKeyAddress(own_address))[1]) {
      case (#data(own_bip340_public_key)) own_bip340_public_key;
      case (_) Debug.trap("bug: expected data script");
    };

    Debug.print("Fetching UTXOs...");
    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = (await BitcoinApi.get_utxos(network, own_address)).utxos;

    // Build the transaction that sends `amount` to the destination address.
    let tx_bytes = await P2tr.build_transaction(schnorr_canister_actor, own_address, own_utxos, dst_address, amount, fee_per_vbyte);
    let transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(tx_bytes)));

    let tx_in_outpoints = Array.map<TxInput.TxInput, Types.OutPoint>(transaction.txInputs, func(txin) { txin.prevOutput });

    let amounts = Array.mapFilter<Utxo, Satoshi>(
      own_utxos,
      func(utxo) {
        if (Option.isSome(Array.find<Types.OutPoint>(tx_in_outpoints, func(tx_in_outpoint) { tx_in_outpoint == utxo.outpoint }))) {
          ?utxo.value;
        } else {
          null;
        };
      },
    );

    let aux =
    #bip341({
      merkle_root_hash = Blob.fromArray(unspendableMerkleRoot(own_bip340_public_key));
    });

    // Sign the transaction.
    let signed_transaction_bytes = await P2tr.sign_key_spend_transaction(schnorr_canister_actor, own_address, transaction, amounts, key_name, Array.map(derivation_path, Blob.fromArray), ?aux, SchnorrApi.sign_with_schnorr);
    let signed_transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(signed_transaction_bytes)));

    Debug.print("Sending transaction...");
    await BitcoinApi.send_transaction(network, signed_transaction_bytes);

    signed_transaction.txid();
  };

  func unspendableMerkleRoot(untweaked_bip340_public_key : [Nat8]) : [Nat8] {
    Hash.taggedHash(untweaked_bip340_public_key, "TapTweak");
  };
};
