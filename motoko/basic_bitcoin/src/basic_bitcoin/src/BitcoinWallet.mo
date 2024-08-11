//! A demo of a very bare-bones Bitcoin "wallet".
//!
//! The wallet here showcases how Bitcoin addresses can be be computed
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
import Option "mo:base/Option";

import EcdsaTypes "../../../motoko-bitcoin/src/ecdsa/Types";
import P2pkh "../../../motoko-bitcoin/src/bitcoin/P2pkh";
import Bitcoin "../../../motoko-bitcoin/src/bitcoin/Bitcoin";
import Address "../../../motoko-bitcoin/src/bitcoin/Address";
import Transaction "../../../motoko-bitcoin/src/bitcoin/Transaction";
import TxInput "../../../motoko-bitcoin/src/bitcoin/TxInput";
import Script "../../../motoko-bitcoin/src/bitcoin/Script";
import Publickey "../../../motoko-bitcoin/src/ecdsa/Publickey";
import Der "../../../motoko-bitcoin/src/ecdsa/Der";
import Affine "../../../motoko-bitcoin/src/ec/Affine";
import Segwit "../../../motoko-bitcoin/src/Segwit";

import BitcoinApi "BitcoinApi";
import EcdsaApi "EcdsaApi";
import SchnorrApi "SchnorrApi";
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

  let SIGHASH_ALL : SighashType = 0x01;

  /// Returns the P2PKH address of this canister at the given derivation path.
  public func get_p2pkh_address(network : Network, key_name : Text, derivation_path : [[Nat8]]) : async BitcoinAddress {
    // Fetch the public key of the given derivation path.
    let public_key = await EcdsaApi.ecdsa_public_key(key_name, Array.map(derivation_path, Blob.fromArray));

    // Compute the address.
    public_key_to_p2pkh_address(network, Blob.toArray(public_key));
  };

  public func get_p2tr_raw_key_spend_address(network : Network, key_name : Text, derivation_path : [[Nat8]]) : async BitcoinAddress {
    // Fetch the public key of the given derivation path.
    let sec1_public_key = await SchnorrApi.schnorr_public_key(key_name, Array.map(derivation_path, Blob.fromArray));
    assert sec1_public_key.size() == 33;

    public_key_to_p2tr_key_spend_address(network, Blob.toArray(sec1_public_key));
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

    // Fetch our public key, P2PKH address, and UTXOs.
    let own_public_key = Blob.toArray(await EcdsaApi.ecdsa_public_key(key_name, Array.map(derivation_path, Blob.fromArray)));
    let own_address = public_key_to_p2pkh_address(network, own_public_key);

    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = (await BitcoinApi.get_utxos(network, own_address)).utxos;

    // Build the transaction that sends `amount` to the destination address.
    let tx_bytes = await build_transaction(own_public_key, own_address, own_utxos, dst_address, amount, fee_per_vbyte);
    let transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(tx_bytes)));

    // Sign the transaction.
    let signed_transaction_bytes = await sign_transaction(own_public_key, own_address, transaction, key_name, Array.map(derivation_path, Blob.fromArray), EcdsaApi.sign_with_ecdsa);
    let signed_transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(signed_transaction_bytes)));

    Debug.print("Sending transaction");
    await BitcoinApi.send_transaction(network, signed_transaction_bytes);

    signed_transaction.id();
  };

  // Builds a transaction to send the given `amount` of satoshis to the
  // destination address.
  public func build_transaction(
    own_public_key : [Nat8],
    own_address : BitcoinAddress,
    own_utxos : [Utxo],
    dst_address : BitcoinAddress,
    amount : Satoshi,
    fee_per_vbyte : MillisatoshiPerVByte,
  ) : async [Nat8] {
    let dst_address_typed = Utils.get_ok_except(Address.addressFromText(dst_address), "failed to decode destination address");

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
      let transaction = Utils.get_ok_except(Bitcoin.buildTransaction(2, own_utxos, [(dst_address_typed, amount)], #p2pkh own_address, Nat64.fromNat(total_fee)), "Error building transaction.");

      // Sign the transaction. In this case, we only care about the size
      // of the signed transaction, so we use a mock signer here for efficiency.
      let signed_transaction_bytes = await sign_transaction(
        own_public_key,
        own_address,
        transaction,
        "", // mock key name
        [], // mock derivation path
        mock_signer,
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

  type SignFun = (Text, [Blob], Blob) -> async Blob;

  // Sign a bitcoin transaction.
  //
  // IMPORTANT: This method is for demonstration purposes only and it only
  // supports signing transactions if:
  //
  // 1. All the inputs are referencing outpoints that are owned by `own_address`.
  // 2. `own_address` is a P2PKH address.
  public func sign_transaction(
    own_public_key : [Nat8],
    own_address : BitcoinAddress,
    transaction : Transaction,
    key_name : Text,
    derivation_path : [Blob],
    signer : SignFun,
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

          let signature_sec = await signer(key_name, derivation_path, Blob.fromArray(sighash));
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

  // Converts a public key to a P2PKH address.
  func public_key_to_p2tr_key_spend_address(network : Network, public_key_bytes : [Nat8]) : BitcoinAddress {
    // human-readable part of the address
    let hrp = switch (network) {
      case (#mainnet) "bc";
      case (#testnet) "tb";
      case (#regtest) "bcrt";
    };

    let version : Nat8 = 1;
    let bip340PublicKeyBytes = Array.subArray(public_key_bytes, 1, 32);
    assert bip340PublicKeyBytes.size() == 32;
    let program = bip340PublicKeyBytes;

    switch (Segwit.encode(hrp, { version; program })) {
      case (#ok address) address;
      case (#err msg) Debug.trap("Error encoding segwit address: " # msg);
    };
  };

  // A mock for rubber-stamping ECDSA signatures.
  func mock_signer(_key_name : Text, _derivation_path : [Blob], _message_hash : Blob) : async Blob {
    Blob.fromArray(Array.freeze(Array.init<Nat8>(64, 255)));
  };

  func public_key_bytes_to_public_key(public_key_bytes : [Nat8]) : PublicKey {
    let point = Utils.unwrap(Affine.fromBytes(public_key_bytes, CURVE));
    Utils.get_ok(Publickey.decode(#point point));
  };

  // Builds a transaction to send the given `amount` of satoshis to the
  // destination address.
  public func build_taproot_transaction(
    own_address : BitcoinAddress,
    own_utxos : [Utxo],
    dst_address : BitcoinAddress,
    amount : Satoshi,
    fee_per_vbyte : MillisatoshiPerVByte,
  ) : async [Nat8] {
    let dst_address_typed = Utils.get_ok_except(Address.addressFromText(dst_address), "failed to decode destination address");

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
      let transaction = Utils.get_ok_except(Bitcoin.buildTransaction(2, own_utxos, [(dst_address_typed, amount)], #p2tr_key own_address, Nat64.fromNat(total_fee)), "Error building transaction.");
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
      let signed_transaction_bytes = await sign_taproot_transaction(
        own_address,
        transaction,
        amounts,
        "", // mock key name
        [], // mock derivation path
        mock_signer,
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
  public func sign_taproot_transaction(
    own_address : BitcoinAddress,
    transaction : Transaction,
    amounts : [Nat64],
    key_name : Text,
    derivation_path : [Blob],
    signer : SignFun,
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

          let signature = Blob.toArray(await signer(key_name, derivation_path, Blob.fromArray(sighash)));
          transaction.witnesses[i] := [signature];
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
  public func send_p2tr_raw_key_spend(network : Network, derivation_path : [[Nat8]], key_name : Text, dst_address : BitcoinAddress, amount : Satoshi) : async [Nat8] {
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
    let own_sec1_public_key = Blob.toArray(await SchnorrApi.schnorr_public_key(key_name, Array.map(derivation_path, Blob.fromArray)));
    let own_address = public_key_to_p2tr_key_spend_address(network, own_sec1_public_key);

    Debug.print("Fetching UTXOs...");
    // Note that pagination may have to be used to get all UTXOs for the given address.
    // For the sake of simplicity, it is assumed here that the `utxo` field in the response
    // contains all UTXOs.
    let own_utxos = (await BitcoinApi.get_utxos(network, own_address)).utxos;

    // Build the transaction that sends `amount` to the destination address.
    let tx_bytes = await build_taproot_transaction(own_address, own_utxos, dst_address, amount, fee_per_vbyte);
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
    let signed_transaction_bytes = await sign_taproot_transaction(own_address, transaction, amounts, key_name, Array.map(derivation_path, Blob.fromArray), SchnorrApi.sign_with_schnorr);
    let signed_transaction = Utils.get_ok(Transaction.fromBytes(Iter.fromArray(signed_transaction_bytes)));

    Debug.print("Sending transaction...");
    await BitcoinApi.send_transaction(network, signed_transaction_bytes);

    signed_transaction.id();
  };
};
