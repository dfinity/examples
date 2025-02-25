//! A demo of a very bare-bones Bitcoin "wallet".
//!
//! The wallet here showcases how Bitcoin addresses can be computed
//! and how Bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't P2PKH.
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.

import Debug "mo:base/Debug";
import Array "mo:base/Array";
import Nat8 "mo:base/Nat8";
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import Iter "mo:base/Iter";
import Blob "mo:base/Blob";
import Nat "mo:base/Nat";

import EcdsaTypes "mo:bitcoin/ecdsa/Types";
import P2pkh "mo:bitcoin/bitcoin/P2pkh";
import Bitcoin "mo:bitcoin/bitcoin/Bitcoin";
import Address "mo:bitcoin/bitcoin/Address";
import Transaction "mo:bitcoin/bitcoin/Transaction";
import Script "mo:bitcoin/bitcoin/Script";
import Publickey "mo:bitcoin/ecdsa/Publickey";
import Der "mo:bitcoin/ecdsa/Der";
import Affine "mo:bitcoin/ec/Affine";

import BitcoinApi "BitcoinApi";
import EcdsaApi "EcdsaApi";
import Types "Types";
import Utils "Utils";

module {
    type Network = Types.Network;
    type BitcoinAddress = Types.BitcoinAddress;
    type Satoshi = Types.Satoshi;
    type Utxo = Types.Utxo;
    type MillisatoshiPerVByte = Types.MillisatoshiPerVByte;
    let CURVE = Types.CURVE;
    type PublicKey = EcdsaTypes.PublicKey;
    type Transaction = Transaction.Transaction;
    type Script = Script.Script;
    type SighashType = Nat32;
    type EcdsaCanisterActor = Types.EcdsaCanisterActor;

    let SIGHASH_ALL : SighashType = 0x01;

    /// Returns the P2PKH address of this canister at the given derivation path.
    public func get_address(ecdsa_canister_actor : EcdsaCanisterActor, network : Network, key_name : Text, derivation_path : [[Nat8]]) : async BitcoinAddress {
        // Fetch the public key of the given derivation path.
        let public_key = await EcdsaApi.ecdsa_public_key(ecdsa_canister_actor, key_name, Array.map(derivation_path, Blob.fromArray));

        // Compute the address.
        public_key_to_p2pkh_address(network, Blob.toArray(public_key));
    };

    /// Sends a transaction to the network that transfers the given amount to the
    /// given destination, where the source of the funds is the canister itself
    /// at the given derivation path.
    public func send(ecdsa_canister_actor : EcdsaCanisterActor, network : Network, derivation_path : [[Nat8]], key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
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

        // Fetch our public key, P2PKH address, and UTXOs.
        let own_public_key = Blob.toArray(await EcdsaApi.ecdsa_public_key(ecdsa_canister_actor, key_name, Array.map(derivation_path, Blob.fromArray)));
        let own_address = public_key_to_p2pkh_address(network, own_public_key);

        // Note that pagination may have to be used to get all UTXOs for the given address.
        // For the sake of simplicity, it is assumed here that the `utxo` field in the response
        // contains all UTXOs.
        let own_utxos = (await BitcoinApi.get_utxos(network, own_address)).utxos;

        // Build the transaction that sends `amount` to the destination address.
        let tx_bytes = await build_transaction(ecdsa_canister_actor, own_public_key, own_address, own_utxos, dst_address, amount, fee_per_vbyte);
        let transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(tx_bytes)));

        // Sign the transaction.
        let signed_transaction_bytes = await sign_transaction(ecdsa_canister_actor, own_public_key, own_address, transaction, key_name, Array.map(derivation_path, Blob.fromArray), EcdsaApi.sign_with_ecdsa);
        let signed_transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(signed_transaction_bytes)));

        Debug.print("Sending transaction");
        await BitcoinApi.send_transaction(network, signed_transaction_bytes);

        signed_transaction.txid();
    };

    // Builds a transaction to send the given `amount` of satoshis to the
    // destination address.
    func build_transaction(
        ecdsa_canister_actor : EcdsaCanisterActor,
        own_public_key : [Nat8],
        own_address : BitcoinAddress,
        own_utxos : [Utxo],
        dst_address : BitcoinAddress,
        amount : Satoshi,
        fee_per_vbyte : MillisatoshiPerVByte,
    ) : async [Nat8] {
        let dst_address_typed = Utils.get_ok_expect(Address.addressFromText(dst_address), "failed to decode destination address");

        // We have a chicken-and-egg problem where we need to know the length
        // of the transaction in order to compute its proper fee, but we need
        // to know the proper fee in order to figure out the inputs needed for
        // the transaction.
        //
        // We solve this problem iteratively. We start with a fee of zero, build
        // and sign a transaction, see what its size is, and then update the fee,
        // rebuild the transaction, until the fee is set to the correct amount.
        let fee_per_vbyte_nat = Nat64.toNat(fee_per_vbyte);
        Debug.print("Building transaction...");
        var total_fee : Nat = 0;
        loop {
            let transaction = Utils.get_ok_expect(Bitcoin.buildTransaction(2, own_utxos, [(dst_address_typed, amount)], #p2pkh own_address, Nat64.fromNat(total_fee)), "Error building transaction.");

            // Sign the transaction. In this case, we only care about the size
            // of the signed transaction, so we use a mock signer here for efficiency.
            let signed_transaction_bytes = await sign_transaction(
                ecdsa_canister_actor,
                own_public_key,
                own_address,
                transaction,
                "", // mock key name
                [], // mock derivation path
                Utils.ecdsa_mock_signer,
            );

            let signed_tx_bytes_len : Nat = signed_transaction_bytes.size();

            if ((signed_tx_bytes_len * fee_per_vbyte_nat) / 1000 == total_fee) {
                Debug.print("Transaction built with fee " # debug_show (total_fee));
                return transaction.toBytes();
            } else {
                total_fee := (signed_tx_bytes_len * fee_per_vbyte_nat) / 1000;
            };
        };
    };

    // Sign a bitcoin transaction.
    //
    // IMPORTANT: This method is for demonstration purposes only and it only
    // supports signing transactions if:
    //
    // 1. All the inputs are referencing outpoints that are owned by `own_address`.
    // 2. `own_address` is a P2PKH address.
    func sign_transaction(
        ecdsa_canister_actor : EcdsaCanisterActor,
        own_public_key : [Nat8],
        own_address : BitcoinAddress,
        transaction : Transaction,
        key_name : Text,
        derivation_path : [Blob],
        signer : Types.EcdsaSignFunction,
    ) : async [Nat8] {
        // Obtain the scriptPubKey of the source address which is also the
        // scriptPubKey of the Tx output being spent.
        switch (Address.scriptPubKey(#p2pkh own_address)) {
            case (#ok scriptPubKey) {
                let scriptSigs = Array.init<Script>(transaction.txInputs.size(), []);

                // Obtain scriptSigs for each Tx input.
                for (i in Iter.range(0, transaction.txInputs.size() - 1)) {
                    let sighash = transaction.createP2pkhSignatureHash(
                        scriptPubKey,
                        Nat32.fromIntWrap(i),
                        SIGHASH_ALL,
                    );

                    let signature_sec = await signer(ecdsa_canister_actor, key_name, derivation_path, Blob.fromArray(sighash));
                    let signature_der = Blob.toArray(Der.encodeSignature(signature_sec));

                    // Append the sighash type.
                    let encodedSignatureWithSighashType = Array.tabulate<Nat8>(
                        signature_der.size() + 1,
                        func(n) {
                            if (n < signature_der.size()) {
                                signature_der[n];
                            } else {
                                Nat8.fromNat(Nat32.toNat(SIGHASH_ALL));
                            };
                        },
                    );

                    // Create Script Sig which looks like:
                    // ScriptSig = <Signature> <Public Key>.
                    let script = [
                        #data encodedSignatureWithSighashType,
                        #data own_public_key,
                    ];
                    scriptSigs[i] := script;
                };
                // Assign ScriptSigs to their associated TxInputs.
                for (i in Iter.range(0, scriptSigs.size() - 1)) {
                    transaction.txInputs[i].script := scriptSigs[i];
                };
            };
            // Verify that our own address is P2PKH.
            case (#err msg) Debug.trap("This example supports signing p2pkh addresses only: " # msg);
        };

        transaction.toBytes();
    };

    // Converts a public key to a P2PKH address.
    func public_key_to_p2pkh_address(network : Network, public_key_bytes : [Nat8]) : BitcoinAddress {
        let public_key = public_key_bytes_to_public_key(public_key_bytes);

        // Compute the P2PKH address from our public key.
        P2pkh.deriveAddress(Types.network_to_network_camel_case(network), Publickey.toSec1(public_key, true));
    };

    func public_key_bytes_to_public_key(public_key_bytes : [Nat8]) : PublicKey {
        let point = Utils.unwrap(Affine.fromBytes(public_key_bytes, CURVE));
        Utils.get_ok(Publickey.decode(#point point));
    };
};
