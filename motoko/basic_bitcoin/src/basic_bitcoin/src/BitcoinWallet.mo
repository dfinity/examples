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
import Nat32 "mo:base/Nat32";
import Nat64 "mo:base/Nat64";
import Iter "mo:base/Iter";

import EcdsaTypes "../../../motoko-bitcoin/src/ecdsa/Types";
import P2pkh "../../../motoko-bitcoin/src/bitcoin/P2pkh";
import Bitcoin "../../../motoko-bitcoin/src/bitcoin/Bitcoin";
import Address "../../../motoko-bitcoin/src/bitcoin/Address";
import Transaction "../../../motoko-bitcoin/src/bitcoin/Transaction";
import Script "../../../motoko-bitcoin/src/bitcoin/Script";
import Publickey "../../../motoko-bitcoin/src/ecdsa/Publickey";
import Affine "../../../motoko-bitcoin/src/ec/Affine";
import Hash "../../../motoko-bitcoin/src/Hash";
import Common "../../../motoko-bitcoin/src/Common";
import bitcoinTestTools "../../../motoko-bitcoin/test/bitcoin/bitcoinTestTools";

import Types "Types";
import EcdsaApi "EcdsaApi";
import BitcoinApi "BitcoinApi";
import Utils "Utils";

module {
  type Network = Types.Network;
  type BitcoinAddress = Types.BitcoinAddress;
  type Satoshi = Types.Satoshi;
  type Utxo = Types.Utxo;
  type MillisatoshiPerByte = Types.MillisatoshiPerByte;
  let CURVE = Types.CURVE;
  type PublicKey = EcdsaTypes.PublicKey;
  type Transaction = Transaction.Transaction;
  type Script = Script.Script;
  type SighashType = Nat32;

  let SIGHASH_ALL : SighashType = 0x01;

  /// Returns the P2PKH address of this canister at the given derivation path.
  public func get_p2pkh_address(network : Network, derivation_path : [[Nat8]]) : async BitcoinAddress {
    // Fetch the public key of the given derivation path.
    let public_key = await EcdsaApi.ecdsa_public_key(derivation_path);

    // Compute the address.
    public_key_to_p2pkh_address(network, public_key)
  };

  /// Sends a transaction to the network that transfers the given amount to the
  /// given destination, where the source of the funds is the canister itself
  /// at the given derivation path.
  public func send(network : Network, derivation_path : [[Nat8]], dst_address : BitcoinAddress, amount : Satoshi) : async () {
    // Get fee percentiles from previous transactions to estimate our own fee.
    let fee_percentiles = await BitcoinApi.get_current_fee_percentiles(network);

    let fee_per_byte : MillisatoshiPerByte = if(fee_percentiles.size() == 0) {
        // There are no fee percentiles. This case can only happen on a regtest
        // network where there are no non-coinbase transactions. In this case,
        // we use a default of 1000 millisatoshis/byte (i.e. 1 satoshi/byte)
        1000
    } else {
        // Choose the 25th percentile for sending fees to minimize the cost.
        fee_percentiles[24]
    };

    // Fetch our public key, P2PKH address, and UTXOs.
    let own_public_key = await EcdsaApi.ecdsa_public_key(derivation_path);
    let own_address = public_key_to_p2pkh_address(network, own_public_key);

    Debug.print("Fetching UTXOs...");
    let own_utxos = (await BitcoinApi.get_utxos(network, own_address)).utxos;

    // Build the transaction that sends `amount` to the destination address.
    let tx_bytes = await build_transaction(own_public_key, own_address, own_utxos, dst_address, amount, fee_per_byte);
    let transaction =
        Utils.get_ok(Transaction.fromBytes(Iter.fromArray(tx_bytes)));

    Debug.print("Transaction to sign: " # debug_show(tx_bytes));

    // Sign the transaction.
    let signed_transaction_bytes = await sign_transaction(own_public_key, own_address, transaction, derivation_path, EcdsaApi.sign_with_ecdsa);
    
    Debug.print("Signed transaction: " # debug_show(signed_transaction_bytes));

    Debug.print("Sending transaction...");
    await BitcoinApi.send_transaction(network, signed_transaction_bytes);
    Debug.print("Done");
  };


// Builds a transaction to send the given `amount` of satoshis to the
// destination address.
public func build_transaction(
    own_public_key : [Nat8],
    own_address : BitcoinAddress,
    own_utxos : [Utxo],
    dst_address : BitcoinAddress,
    amount : Satoshi,
    fee_per_byte : MillisatoshiPerByte,
) : async [Nat8] {
    // We have a chicken-and-egg problem where we need to know the length
    // of the transaction in order to compute its proper fee, but we need
    // to know the proper fee in order to figure out the inputs needed for
    // the transaction.
    //
    // We solve this problem iteratively. We start with a fee of zero, build
    // and sign a transaction, see what its size is, and then update the fee,
    // rebuild the transaction, until the fee is set to the correct amount.
    let fee_per_byte_nat = Nat64.toNat(fee_per_byte);
    Debug.print("Building transaction...");
    var total_fee : Nat = 0;
    loop {
        let transaction =
            Utils.get_ok_except(Bitcoin.buildTransaction(2, own_utxos, [(#p2pkh dst_address, amount)], #p2pkh own_address, Nat64.fromNat(total_fee)), "Error building transaction.");

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction_bytes = await sign_transaction(
            own_public_key,
            own_address,
            transaction,
            [], // mock derivation path
            mock_signer,
        );

        let signed_tx_bytes_len : Nat = signed_transaction_bytes.size();

        if((signed_tx_bytes_len * fee_per_byte_nat) / 1000 == total_fee) {
            Debug.print("Transaction built with fee " # debug_show(total_fee));
            return transaction.toBytes();
        } else {
            total_fee := (signed_tx_bytes_len * fee_per_byte_nat) / 1000;
        }
    }
  };

  type SignFun = ([[Nat8]], [Nat8]) -> async [Nat8];

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
    derivation_path : [[Nat8]],
    signer : SignFun,
  ) : async [Nat8] {
    // Obtain the scriptPubKey of the source address which is also the
    // scriptPubKey of the Tx output being spent.
    switch (Address.scriptPubKey(#p2pkh own_address)) {
      case (#ok scriptPubKey) {
        let public_key = public_key_bytes_to_public_key(own_public_key);
        let scriptSigs = Array.init<Script>(transaction.txInputs.size(), []);

        // Obtain scriptSigs for each Tx input.
        for (i in Iter.range(0, transaction.txInputs.size() - 1)) {
          let sighash = Hash.doubleSHA256(transaction.createSignatureHash(
              scriptPubKey, Nat32.fromIntWrap(i), SIGHASH_ALL));

          let signature_sec = await signer(derivation_path, sighash);

          let signature = {
            r = Common.readBE256(signature_sec, 0);
            s = Common.readBE256(signature_sec, 32);
          };
          let signature_der = bitcoinTestTools.signatureToDer(signature, SIGHASH_ALL);

          // Create Script Sig which looks like:
          // ScriptSig = <Signature> <Public Key>.
          let script = [
            #data signature_der,
            #data (Publickey.toSec1(public_key, true).0)
          ];
          scriptSigs[i] := script;
        };
        // Assign ScriptSigs to their associated TxInputs.
        for (i in Iter.range(0, scriptSigs.size() - 1)) {
          transaction.txInputs[i].script := scriptSigs[i];
        };
      };
      // Verify that our own address is P2PKH.
      case (#err msg)
        Debug.trap("This example supports signing p2pkh addresses only.");
    };

    transaction.toBytes()
  };

  // Converts a public key to a P2PKH address.
  func public_key_to_p2pkh_address(network : Network, public_key_bytes : [Nat8]) : BitcoinAddress {
    let public_key = public_key_bytes_to_public_key(public_key_bytes);

    // Compute the P2PKH address from our public key.
    P2pkh.deriveAddress(network, public_key, true)
  };

  // A mock for rubber-stamping ECDSA signatures.
  func mock_signer(_derivation_path : [[Nat8]], _message_hash : [Nat8]) : async [Nat8] {
      Array.freeze(Array.init<Nat8>(64, 1))
  };

  func public_key_bytes_to_public_key(public_key_bytes : [Nat8]) : PublicKey {
      let point = Utils.unwrap(Affine.fromBytes(public_key_bytes, CURVE));
      Utils.get_ok(Publickey.decode(#point point))
  };
}
