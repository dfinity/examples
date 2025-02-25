//! A demo of a very bare-bones Bitcoin "wallet".
//!
//! The wallet here showcases how Bitcoin addresses can be computed
//! and how Bitcoin transactions can be signed. It is missing several
//! pieces that any production-grade wallet would have, including:
//!
//! * Support for address types that aren't P2TR spends.
//! * Caching spent UTXOs so that they are not reused in future transactions.
//! * Option to set the fee.
//!
//! The P2TR address obtained from the `get_address` function allows for
//! spending with an internal key as well as a simple script, allowing to use just
//! one key for spending. The key in the script is different from the internal
//! key, but both are derived using the ICP.

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
import {
    leafHash;
    leafScript;
    tweakFromKeyAndHash;
    tweakPublicKey;
} "mo:bitcoin/bitcoin/P2tr";
import Transaction "mo:bitcoin/bitcoin/Transaction";
import TxInput "mo:bitcoin/bitcoin/TxInput";
import Script "mo:bitcoin/bitcoin/Script";
import Segwit "mo:bitcoin/Segwit";
import Hash "mo:bitcoin/Hash";

import BitcoinApi "BitcoinApi";
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
    type P2trDerivationPaths = Types.P2trDerivationPaths;

    /// Returns the P2TR address that allows for key as well as script spends.
    public func get_address(schnorr_canister_actor : SchnorrCanisterActor, network : Network, key_name : Text, derivation_paths : P2trDerivationPaths) : async BitcoinAddress {
        let internal_bip340_public_key = await fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_paths.key_path_derivation_path);
        let script_bip340_public_key = await fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_paths.script_path_derivation_path);

        let { tweaked_address; is_even = _ } = internal_key_and_script_key_to_p2tr_address(internal_bip340_public_key, script_bip340_public_key, network);
        tweaked_address;
    };

    /// Converts an internal public key and a script public key to a P2TR spend
    /// address. The script public key is used to derive the leaf script, which
    /// can be spent only using the script public key.
    public func internal_key_and_script_key_to_p2tr_address(internal_bip340_public_key : [Nat8], script_bip340_public_key : [Nat8], network : Network) : {
        tweaked_address : BitcoinAddress;
        is_even : Bool;
    } {
        let leaf_script = Utils.get_ok(leafScript(script_bip340_public_key));
        let leaf_hash = leafHash(leaf_script);
        let tweak = Utils.get_ok(tweakFromKeyAndHash(internal_bip340_public_key, leaf_hash));
        let { bip340_public_key = tweaked_public_key; is_even } = Utils.get_ok(tweakPublicKey(internal_bip340_public_key, tweak));

        {
            tweaked_address = tweaked_public_key_to_p2tr_address(network, tweaked_public_key);
            is_even;
        };
    };

    // Builds a transaction to send the given `amount` of satoshis to the
    // destination address.
    public func build_transaction(
        schnorr_canister_actor : SchnorrCanisterActor,
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
            let signed_transaction_bytes = await sign_key_spend_transaction(
                schnorr_canister_actor,
                own_address,
                transaction,
                amounts,
                "", // mock key name
                [], // mock derivation path
                null, // mock aux
                Utils.schnorr_mock_signer,
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

    // Sign a key spend bitcoin taproot transaction.
    //
    // IMPORTANT: This method is for demonstration purposes only and it only
    // supports signing transactions if:
    //
    // 1. All the inputs are referencing outpoints that are owned by `own_address`.
    // 2. `own_address` is a P2TR address.
    public func sign_key_spend_transaction(
        schnorr_canister_actor : SchnorrCanisterActor,
        own_address : BitcoinAddress,
        transaction : Transaction,
        amounts : [Nat64],
        key_name : Text,
        derivation_path : [Blob],
        aux : ?Types.SchnorrAux,
        signer : Types.SchnorrSignFunction,
    ) : async [Nat8] {
        // Obtain the scriptPubKey of the source address which is also the
        // scriptPubKey of the Tx output being spent.
        switch (Address.scriptPubKey(#p2tr_key own_address)) {
            case (#ok scriptPubKey) {
                assert scriptPubKey.size() == 2;

                // Obtain a witness for each Tx input.
                for (i in Iter.range(0, transaction.txInputs.size() - 1)) {
                    let sighash = transaction.createTaprootKeySpendSignatureHash(
                        amounts,
                        scriptPubKey,
                        Nat32.fromIntWrap(i),
                    );

                    let signature = Blob.toArray(await signer(schnorr_canister_actor, key_name, derivation_path, Blob.fromArray(sighash), aux));
                    transaction.witnesses[i] := [signature];
                };
            };
            // Verify that our own address is P2TR key spend address.
            case (#err msg) Debug.trap("This example supports signing p2tr key spend addresses only: " # msg);
        };

        transaction.toBytes();
    };

    /// Sign a script spend bitcoin taproot transaction.
    ///
    /// IMPORTANT: This method is for demonstration purposes only and it only
    /// supports signing transactions if:
    ///
    /// 1. All the inputs are referencing outpoints that are owned by `own_address`.
    /// 2. `own_address` is a P2TR address with a single leaf in MAST that just
    ///    allows one key to be used for spending.
    func sign_script_spend_transaction(
        schnorr_canister_actor : SchnorrCanisterActor,
        own_address : BitcoinAddress,
        leaf_script : Script.Script,
        internal_public_key : [Nat8],
        tweaked_key_is_even : Bool,
        transaction : Transaction,
        amounts : [Nat64],
        key_name : Text,
        derivation_path : [Blob],
        signer : Types.SchnorrSignFunction,
    ) : async [Nat8] {
        let leaf_hash = leafHash(leaf_script);

        assert internal_public_key.size() == 32;

        let script_bytes_sized = Script.toBytes(leaf_script);
        let script_len : Nat = script_bytes_sized.size() - 1;
        // remove the size prefix
        let script_bytes = Array.subArray(script_bytes_sized, 1, script_len);

        let control_block_bytes = control_block(tweaked_key_is_even, internal_public_key);
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

                    let signature = Blob.toArray(await signer(schnorr_canister_actor, key_name, derivation_path, Blob.fromArray(sighash), null));
                    transaction.witnesses[i] := [signature, script_bytes, control_block_bytes];
                };
            };
            // Verify that our own address is P2TR key spend address.
            case (#err msg) Debug.trap("This example supports signing p2tr key spend addresses only: " # msg);
        };

        transaction.toBytes();
    };

    /// Sends a key spend transaction with a non-empty MAST to the network that
    /// transfers the given amount to the given destination, where the source
    /// of the funds is the canister itself at the given derivation path.
    public func send_key_path(schnorr_canister_actor : SchnorrCanisterActor, network : Network, derivation_paths : P2trDerivationPaths, key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
        let internal_bip340_public_key = await fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_paths.key_path_derivation_path);
        let script_bip340_public_key = await fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_paths.script_path_derivation_path);
        let { tweaked_address = own_tweaked_address } = internal_key_and_script_key_to_p2tr_address(internal_bip340_public_key, script_bip340_public_key, network);

        let leaf_script = Utils.get_ok(leafScript(script_bip340_public_key));
        let aux =
        #bip341({
            merkle_root_hash = Blob.fromArray(leafHash(leaf_script));
        });

        await send_key_path_generic(schnorr_canister_actor, own_tweaked_address, network, derivation_paths.key_path_derivation_path, key_name, ?aux, dst_address, amount);
    };

    /// Sends a key spend transaction to the network that transfers the given amount to the
    /// given destination, where the source of the funds is the canister itself
    /// at the given derivation path.
    public func send_key_path_generic(schnorr_canister_actor : SchnorrCanisterActor, own_address : BitcoinAddress, network : Network, signer_derivation_path : [[Nat8]], key_name : Text, aux : ?Types.SchnorrAux, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
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

        Debug.print("Fetching UTXOs...");
        // Note that pagination may have to be used to get all UTXOs for the given address.
        // For the sake of simplicity, it is assumed here that the `utxo` field in the response
        // contains all UTXOs.
        let own_utxos = (await BitcoinApi.get_utxos(network, own_address)).utxos;

        // Build the transaction that sends `amount` to the destination address.
        let tx_bytes = await build_transaction(schnorr_canister_actor, own_address, own_utxos, dst_address, amount, fee_per_vbyte);
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

        let signed_transaction_bytes = await sign_key_spend_transaction(schnorr_canister_actor, own_address, transaction, amounts, key_name, Array.map(signer_derivation_path, Blob.fromArray), aux, SchnorrApi.sign_with_schnorr);

        Debug.print("Sending transaction : " # debug_show (signed_transaction_bytes));
        let signed_transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(signed_transaction_bytes)));

        Debug.print("Sending transaction...");
        await BitcoinApi.send_transaction(network, signed_transaction_bytes);

        signed_transaction.txid();
    };

    /// Sends a script spend transaction to the network that transfers the given amount to the
    /// given destination, where the source of the funds is the canister itself
    /// at the given derivation path.
    public func send_script_path(schnorr_canister_actor : SchnorrCanisterActor, network : Network, derivation_paths : P2trDerivationPaths, key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
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

        // Fetch our public keys, P2TR script spend address, and UTXOs.
        let internal_bip340_public_key = await fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_paths.key_path_derivation_path);
        let script_bip340_public_key = await fetch_bip340_public_key(schnorr_canister_actor, key_name, derivation_paths.script_path_derivation_path);
        let { tweaked_address = own_tweaked_address; is_even } = internal_key_and_script_key_to_p2tr_address(internal_bip340_public_key, script_bip340_public_key, network);

        let own_leaf_script = Utils.get_ok(leafScript(script_bip340_public_key));

        Debug.print("Fetching UTXOs...");
        // Note that pagination may have to be used to get all UTXOs for the given address.
        // For the sake of simplicity, it is assumed here that the `utxo` field in the response
        // contains all UTXOs.
        let own_utxos = (await BitcoinApi.get_utxos(network, own_tweaked_address)).utxos;

        // Build the transaction that sends `amount` to the destination address.
        let tx_bytes = await build_transaction(schnorr_canister_actor, own_tweaked_address, own_utxos, dst_address, amount, fee_per_vbyte);
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
        let signed_transaction_bytes = await sign_script_spend_transaction(
            schnorr_canister_actor,
            own_tweaked_address,
            own_leaf_script,
            internal_bip340_public_key,
            is_even,
            transaction,
            amounts,
            key_name,
            Array.map(derivation_paths.script_path_derivation_path, Blob.fromArray),
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

    // Converts a tweaked public key to a P2TR address.
    public func tweaked_public_key_to_p2tr_address(network : Network, bip340_public_key_bytes : [Nat8]) : BitcoinAddress {
        // human-readable part of the address
        let hrp = switch (network) {
            case (#mainnet) "bc";
            case (#testnet) "tb";
            case (#regtest) "bcrt";
        };

        let version : Nat8 = 1;
        assert bip340_public_key_bytes.size() == 32;

        switch (Segwit.encode(hrp, { version; program = bip340_public_key_bytes })) {
            case (#ok address) address;
            case (#err msg) Debug.trap("Error encoding segwit address: " # msg);
        };
    };

    public func unspendableMerkleRoot(untweaked_bip340_public_key : [Nat8]) : [Nat8] {
        Hash.taggedHash(untweaked_bip340_public_key, "TapTweak");
    };

    public func fetch_bip340_public_key(schnorr_canister_actor : SchnorrCanisterActor, key_name : Text, derivation_path : [[Nat8]]) : async [Nat8] {
        let sec1_public_key = Blob.toArray(await SchnorrApi.schnorr_public_key(schnorr_canister_actor, key_name, Array.map(derivation_path, Blob.fromArray)));
        Array.subArray(sec1_public_key, 1, 32);
    };
};
