//! A demo of a very bare-bones Bitcoin "wallet".
//!
//! The wallet here showcases how Bitcoin addresses can be computed
//! and how Bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't a P2TR key spend with unspendable
//!   Merkle script tree.
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.

import Array "mo:base/Array";
import Nat8 "mo:base/Nat8";
import Blob "mo:base/Blob";

import Script "mo:bitcoin/bitcoin/Script";
import Transaction "mo:bitcoin/bitcoin/Transaction";

import P2tr "P2tr";
import SchnorrApi "SchnorrApi";
import Types "Types";

module {
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type Satoshi = Types.Satoshi;
  type Utxo = Types.Utxo;
  type MillisatoshiPerVByte = Types.MillisatoshiPerVByte;
  type Transaction = Transaction.Transaction;
  type Script = Script.Script;
  type SchnorrCanisterActor = Types.SchnorrCanisterActor;

  /// Sends a transaction to the network that transfers the given amount to the
  /// given destination, where the source of the funds is the canister itself
  /// at the given derivation path.
  public func send(schnorr_canister_actor : SchnorrCanisterActor, network : Network, derivation_path : [[Nat8]], key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
    let own_address = await P2tr.get_address_key_only(schnorr_canister_actor, network, key_name, derivation_path, null);
    let untweaked_sec1_public_key = await SchnorrApi.schnorr_public_key(schnorr_canister_actor, key_name, Array.map(derivation_path, Blob.fromArray));
    assert untweaked_sec1_public_key.size() == 33;
    let untweaked_bip340_public_key_bytes = Array.subArray(Blob.toArray(untweaked_sec1_public_key), 1, 32);
    let aux =
    #bip341({
      merkle_root_hash = Blob.fromArray(P2tr.unspendableMerkleRoot(untweaked_bip340_public_key_bytes));
    });
    await P2tr.send_key_spend_generic(schnorr_canister_actor, own_address, untweaked_bip340_public_key_bytes, network, derivation_path, key_name, ?aux, dst_address, amount);
  };
};
