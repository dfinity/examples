//! A demo of a very bare-bones Bitcoin "wallet".
//!
//! The wallet here showcases how Bitcoin addresses can be computed
//! and how Bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't a P2TR key spend with unspendable
//!   Merkle script tree root
//!   (see [BIP341](https://github.com/bitcoin/bips/blob/master/bip-0341.mediawiki#cite_note-23)).
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.

import Nat8 "mo:base/Nat8";
import Blob "mo:base/Blob";

import Script "mo:bitcoin/bitcoin/Script";
import Transaction "mo:bitcoin/bitcoin/Transaction";
import { tweakFromKeyAndHash; tweakPublicKey } "mo:bitcoin/bitcoin/P2tr";

import P2tr "P2tr";
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

  /// Sends a transaction to the network that transfers the given amount to the
  /// given destination, where the source of the funds is the canister itself
  /// at the given derivation path.
  public func send(schnorr_canister_actor : SchnorrCanisterActor, network : Network, derivation_path : [[Nat8]], key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
    let own_address = await get_address_key_only(schnorr_canister_actor, network, key_name, derivation_path);
    let untweaked_bip340_public_key_bytes = await P2tr.fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_path);
    let aux =
    #bip341({
      merkle_root_hash = Blob.fromArray(P2tr.unspendableMerkleRoot(untweaked_bip340_public_key_bytes));
    });
    await P2tr.send_key_path_generic(schnorr_canister_actor, own_address, network, derivation_path, key_name, ?aux, dst_address, amount);
  };

  /// Returns the P2TR key-only address of this canister at a specific
  /// derivation path. The Merkle tree root is computed as
  /// `taggedHash(bip340_public_key_bytes, "TapTweak")` and is unspendable.
  public func get_address_key_only(schnorr_canister_actor : SchnorrCanisterActor, network : Network, key_name : Text, derivation_path : [[Nat8]]) : async BitcoinAddress {
    let bip340_public_key_bytes = await P2tr.fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_path);

    let merkleRoot = P2tr.unspendableMerkleRoot(bip340_public_key_bytes);
    let tweak = Utils.get_ok(tweakFromKeyAndHash(bip340_public_key_bytes, merkleRoot));
    let tweaked_public_key = Utils.get_ok(tweakPublicKey(bip340_public_key_bytes, tweak)).bip340_public_key;

    P2tr.tweaked_public_key_to_p2tr_address(network, tweaked_public_key);
  };
};
