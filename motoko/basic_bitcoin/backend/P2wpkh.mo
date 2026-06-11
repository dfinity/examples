//! P2WPKH (Pay-to-Witness-Public-Key-Hash) address generation.
//!
//! Adapted from https://github.com/caffeinelabs/motoko-bitcoin/pull/9
//!
//! Note: send_from_p2wpkh_address is not yet implemented — it requires BIP143
//! sighash support in mo:bitcoin (see https://github.com/caffeinelabs/motoko-bitcoin/pull/9).

import Array "mo:core/Array";
import Blob "mo:core/Blob";
import Nat "mo:core/Nat";
import Nat8 "mo:core/Nat8";
import Runtime "mo:core/Runtime";

import Segwit "mo:bitcoin/Segwit";
import Hash "mo:bitcoin/Hash";

import EcdsaApi "EcdsaApi";
import Types "Types";

module {
    type Network = Types.Network;
    type BitcoinAddress = Types.BitcoinAddress;

    /// Returns the P2WPKH (SegWit v0) address for the given derivation path.
    public func get_address(network : Network, key_name : Text, derivation_path : [[Nat8]]) : async BitcoinAddress {
        // Fetch the compressed SEC1 public key for the given derivation path.
        let public_key_bytes = (await EcdsaApi.ecdsa_public_key(key_name, derivation_path.map(Blob.fromArray))).toArray();

        // Derive the P2WPKH address from the compressed public key.
        public_key_to_p2wpkh_address(network, public_key_bytes);
    };

    /// Derives a P2WPKH Bech32 address from a compressed SEC1 public key (33 bytes).
    func public_key_to_p2wpkh_address(network : Network, public_key_bytes : [Nat8]) : BitcoinAddress {
        if (public_key_bytes.size() != 33) {
            Runtime.trap("P2WPKH requires a compressed public key (33 bytes), got " # public_key_bytes.size().toText());
        };

        // Compute HASH160(pubkey) = RIPEMD160(SHA256(pubkey))
        let pub_key_hash : [Nat8] = Hash.hash160(public_key_bytes);
        if (pub_key_hash.size() != 20) {
            Runtime.trap("Internal error: HASH160 result is not 20 bytes");
        };

        let hrp = switch (network) {
            case (#mainnet) "bc";
            case (#testnet) "tb";
            case (#regtest) "bcrt";
        };

        let witness_program : Segwit.WitnessProgram = {
            version = 0;
            program = pub_key_hash;
        };

        switch (Segwit.encode(hrp, witness_program)) {
            case (#ok address) address;
            case (#err msg) Runtime.trap("Error encoding P2WPKH segwit address: " # msg);
        };
    };

};
