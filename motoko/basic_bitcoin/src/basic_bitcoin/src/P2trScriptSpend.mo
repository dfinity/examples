//! A demo of a very bare-bones Bitcoin "wallet".
//!
//! The wallet here showcases how Bitcoin addresses can be computed
//! and how Bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't P2TR script key spend.
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
import Option "mo:base/Option";

import Address "mo:bitcoin/bitcoin/Address";
import Bitcoin "mo:bitcoin/bitcoin/Bitcoin";
import { leafHash; leafScript; tweakFromKeyAndHash; tweakPublicKey } "mo:bitcoin/bitcoin/P2tr";
import Transaction "mo:bitcoin/bitcoin/Transaction";
import TxInput "mo:bitcoin/bitcoin/TxInput";
import Script "mo:bitcoin/bitcoin/Script";

import BitcoinApi "BitcoinApi";
import P2trRawKeySpend "P2trRawKeySpend";
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

    public func get_address(network : Network, key_name : Text, derivation_path : [[Nat8]]) : async BitcoinAddress {
        // Fetch the public key of the given derivation path.
        let sec1_public_key = await SchnorrApi.schnorr_public_key(key_name, Array.map(derivation_path, Blob.fromArray));
        assert sec1_public_key.size() == 33;
        let bip340_public_key_bytes = Array.subArray(Blob.toArray(sec1_public_key), 1, 32);
        let { tweaked_address; is_even = _ } = public_key_to_p2tr_script_spend_address(network, bip340_public_key_bytes);
        tweaked_address;
    };

    // Converts a public key to a P2TR script spend address.
    public func public_key_to_p2tr_script_spend_address(network : Network, bip340_public_key_bytes : [Nat8]) : {
        tweaked_address : BitcoinAddress;
        is_even : Bool;
    } {
        let leaf_script = Utils.get_ok(leafScript(bip340_public_key_bytes));
        let leaf_hash = leafHash(leaf_script);
        let tweak = Utils.get_ok(tweakFromKeyAndHash(bip340_public_key_bytes, leaf_hash));
        let { bip340_public_key = tweaked_public_key; is_even } = Utils.get_ok(tweakPublicKey(bip340_public_key_bytes, tweak));

        // we can reuse `public_key_to_p2tr_key_spend_address` because this
        // essentially encodes the input public key as a P2TR address without tweaking
        {
            tweaked_address = P2trRawKeySpend.public_key_to_p2tr_key_spend_address(network, tweaked_public_key);
            is_even;
        };
    };

    // Builds a transaction to send the given `amount` of satoshis to the
    // destination address.
    func build_transaction(
        own_address : BitcoinAddress,
        leaf_script : Script.Script,
        internal_public_key : [Nat8],
        tweaked_key_is_even : Bool,
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
        var total_fee : Nat = 0;

        loop {
            let transaction = Utils.get_ok_expect(Bitcoin.buildTransaction(2, own_utxos, [(dst_address_typed, amount)], #p2tr_key own_address, Nat64.fromNat(total_fee)), "Error building transaction.");
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

            // Sign the transaction. In this case, we only care about the size
            // of the signed transaction, so we use a mock signer here for efficiency.
            let signed_transaction_bytes = await sign_transaction(
                own_address,
                leaf_script,
                internal_public_key,
                tweaked_key_is_even,
                transaction,
                amounts,
                "", // mock key name
                [], // mock derivation path
                Utils.mock_signer,
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
    // 2. `own_address` is a P2TR script spend address.
    func sign_transaction(
        own_address : BitcoinAddress,
        leaf_script : Script.Script,
        internal_public_key : [Nat8],
        tweaked_key_is_even : Bool,
        transaction : Transaction,
        amounts : [Nat64],
        key_name : Text,
        derivation_path : [Blob],
        signer : Types.SignFunction,
    ) : async [Nat8] {
        let leaf_hash = leafHash(leaf_script);

        assert internal_public_key.size() == 32;

        let script_bytes_sized = Script.toBytes(leaf_script);
        // remove the size prefix
        let script_bytes = Array.subArray(script_bytes_sized, 1, script_bytes_sized.size() - 1);

        let _control_block = control_block(tweaked_key_is_even, internal_public_key);
        // Obtain the scriptPubKey of the source address which is also the
        // scriptPubKey of the Tx output being spent.
        switch (Address.scriptPubKey(#p2tr_key own_address)) {
            case (#ok scriptPubKey) {
                assert scriptPubKey.size() == 2;

                // Obtain a witness for each Tx input.
                for (i in Iter.range(0, transaction.txInputs.size() - 1)) {
                    let sighash = transaction.createTaprootScriptSpendSignatureHash(
                        amounts,
                        scriptPubKey,
                        Nat32.fromIntWrap(i),
                        leaf_hash,
                    );

                    Debug.print("Signing sighash: " # debug_show (sighash));

                    let signature = Blob.toArray(await signer(key_name, derivation_path, Blob.fromArray(sighash)));
                    transaction.witnesses[i] := [signature, script_bytes, _control_block];
                };
            };
            // Verify that our own address is P2TR key spend address.
            case (#err msg) Debug.trap("This example supports signing p2tr key spend addresses only: " # msg);
        };

        transaction.toBytes();
    };

    /// Sends a transaction to the network that transfers the given amount to the
    /// given destination, where the source of the funds is the canister itself
    /// at the given derivation path.
    public func send(network : Network, derivation_path : [[Nat8]], key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
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

        // Fetch our public key, P2TR script spend address, and UTXOs.
        let own_sec1_public_key = Blob.toArray(await SchnorrApi.schnorr_public_key(key_name, Array.map(derivation_path, Blob.fromArray)));
        let own_bip340_public_key = Array.subArray(own_sec1_public_key, 1, 32);
        let { tweaked_address = own_tweaked_address; is_even } = public_key_to_p2tr_script_spend_address(network, own_bip340_public_key);

        let own_leaf_script = Utils.get_ok(leafScript(own_bip340_public_key));

        let _control_block = control_block(is_even, own_bip340_public_key);

        Debug.print("Fetching UTXOs...");
        // Note that pagination may have to be used to get all UTXOs for the given address.
        // For the sake of simplicity, it is assumed here that the `utxo` field in the response
        // contains all UTXOs.
        let own_utxos = (await BitcoinApi.get_utxos(network, own_tweaked_address)).utxos;

        // Build the transaction that sends `amount` to the destination address.
        let tx_bytes = await build_transaction(own_tweaked_address, own_leaf_script, own_bip340_public_key, is_even, own_utxos, dst_address, amount, fee_per_vbyte);
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

        // Sign the transaction.
        let signed_transaction_bytes = await sign_transaction(
            own_tweaked_address,
            own_leaf_script,
            own_bip340_public_key,
            is_even,
            transaction,
            amounts,
            key_name,
            Array.map(derivation_path, Blob.fromArray),
            SchnorrApi.sign_with_schnorr,
        );
        Debug.print("Sending transaction : " # debug_show (signed_transaction_bytes));
        let signed_transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(signed_transaction_bytes)));

        Debug.print("Sending transaction...");
        await BitcoinApi.send_transaction(network, signed_transaction_bytes);

        signed_transaction.txid();
    };

    func control_block(tweaked_public_key_is_even : Bool, internal_public_key : [Nat8]) : [Nat8] {
        let leaf_version : Nat8 = 0xc0;
        let parity : Nat8 = if (tweaked_public_key_is_even) 0 else 1;
        let first_byte = [leaf_version | parity];

        if (internal_public_key.size() != 32) {
            Debug.trap("Internal public key must be 32 bytes long to be used in control block.");
        };

        let result = Array.flatten([first_byte, internal_public_key]);

        if (result.size() != 33) {
            Debug.trap("Control block must be 33 bytes long.");
        };

        result;
    };
};
