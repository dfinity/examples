use crate::{
    canister_common::{
        GET_CURRENT_FEE_PERCENTILES_COST_CYCLES, SEND_TRANSACTION_BASE_COST_CYCLES,
        SEND_TRANSACTION_COST_CYCLES_PER_BYTE,
    },
    ecdsa::sign_with_ecdsa,
    types::{
        from_bitcoin_network_to_ic_btc_types_network, from_types_network_to_bitcoin_network,
        BuiltTransaction,
    },
    types::{
        AddressUsingPrimitives, EcdsaPubKey, Fee, FeeRequest, GetCurrentFeeError,
        ManagementCanisterReject, MultiTransferArgs, MultiTransferError, MultiTransferResult,
        TransactionInfo, MIN_CONFIRMATIONS_UPPER_BOUND,
    },
    upgrade_management::get_address_using_primitives,
    utxo_management::{get_utxos, has_utxo_min_confirmations},
};
use bitcoin::{
    blockdata::script::Builder, hashes::Hash, psbt::serialize::Serialize, Address, AddressType,
    EcdsaSighashType, Network, OutPoint, Script, Transaction, TxIn, TxOut, Txid, Witness,
};
use ic_btc_types::{GetCurrentFeePercentilesRequest, SendTransactionRequest};
use ic_btc_types::{MillisatoshiPerByte, Satoshi, Utxo};
use ic_cdk::{api::call::call_with_payment, export::Principal};
use std::{collections::BTreeMap, future::Future};

// The signature hash type that is always used.
const SIG_HASH_TYPE: EcdsaSighashType = EcdsaSighashType::All;

// Dust is the amount below which spending the `TxOut` would cost more in fee than the amount of the `TxOut`.
// Here we calculate the dust threshold by calculating the minimum number of bytes to spend an additional `TxOut`.
//
// Signed `TxIn` (between 147 and 149 bytes): (source: https://en.bitcoin.it/wiki/Transaction#General_format_.28inside_a_block.29_of_each_input_of_a_transaction_-_Txin)
// previous transaction hash (32 bytes)
// previous `TxOut`-index (4 bytes)
// `TxIn`-script length (1 byte)
// scriptSig (108 bytes): (source: https://bitcoin.stackexchange.com/questions/48279/how-big-is-the-input-of-a-p2pkh-transaction)
// - push opcode (1 byte)
// - signature (between 71 and 73 bytes) (source: https://transactionfee.info/charts/bitcoin-script-ecdsa-length/)
// - push opcode (1 byte)
// - compressed pubkey (33 bytes)
// sequence number (4 bytes)
//
// `TxOut` (33 bytes): (source: https://en.bitcoin.it/wiki/Transaction#General_format_.28inside_a_block.29_of_each_output_of_a_transaction_-_Txout)
// value (8 bytes)
// scriptPubKey length (1 byte)
// scriptPubKey (24 bytes): (source: https://en.bitcoin.it/wiki/Transaction#Pay-to-PubkeyHash)
// - OP_DUP
// - OP_HASH160
// - <public key hash>
// - OP_EQUAL
// - OP_CHECKSIG
//
// The dust relay fee is 3 sat/byte (source: https://github.com/bitcoin/bitcoin/blob/26ec2f2d6bb12525044b6d09422b42715fc09319/src/policy/policy.h#L52-L57)
// The calculation of the dust threshold is done assuming that there isn't any incentive to increase the fee because the mempool is below the block size limit.
// This calculation is done assuming that we add this dust `TxOut` and redeem `TxIn` in already existing transaction (so we don't have to count number of bytes of other transaction fields).
const DUST_THRESHOLD: Satoshi = 546;

/// Returns fees as percentiles in millisatoshis/byte over the last 10,000 transactions.
pub(crate) async fn get_current_fees(
    network: Network,
) -> Result<Vec<MillisatoshiPerByte>, ManagementCanisterReject> {
    let res: Result<(Vec<MillisatoshiPerByte>,), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_get_current_fee_percentiles",
        (GetCurrentFeePercentilesRequest {
            network: from_bitcoin_network_to_ic_btc_types_network(network),
        },),
        GET_CURRENT_FEE_PERCENTILES_COST_CYCLES,
    )
    .await;

    match res {
        // Return the fees to the caller.
        Ok(data) => Ok(data.0),

        // The call to `get_current_fees` was rejected for a given reason (e.g., not enough cycles were attached to the call).
        Err((rejection_code, message)) => Err(ManagementCanisterReject(rejection_code, message)),
    }
}

/// Returns the percentile associated with the given `FeeRequest`.
pub(crate) fn evaluate_fee_request(fee_request: FeeRequest) -> Result<usize, GetCurrentFeeError> {
    let percentile = match fee_request {
        FeeRequest::Slow => 25,
        FeeRequest::Standard => 50,
        FeeRequest::Fast => 75,
        FeeRequest::Percentile(percentile) => percentile,
    } as usize;
    if percentile >= 99 {
        return Err(GetCurrentFeeError::InvalidPercentile);
    }
    Ok(percentile)
}

/// Returns the fee as a percentile in millisatoshis/byte over the last 10,000 transactions.
pub(crate) async fn get_current_fee(
    fee_request: FeeRequest,
    network: Network,
) -> Result<MillisatoshiPerByte, GetCurrentFeeError> {
    let percentile = evaluate_fee_request(fee_request)?;
    let fees = get_current_fees(network).await?;
    // A given percentile between 0 and 99 is invalid if the management canister doesn't have enough transactions to compute current fees.
    if percentile > fees.len() {
        return Err(GetCurrentFeeError::InvalidPercentile);
    }
    Ok(fees[percentile])
}

/// Sends the given transaction to the network the management canister interacts with.
pub(crate) async fn send_transaction(
    transaction: Vec<u8>,
    network: Network,
) -> Result<(), ManagementCanisterReject> {
    let transaction_cost_cycles = SEND_TRANSACTION_BASE_COST_CYCLES
        + (transaction.len() as u64) * SEND_TRANSACTION_COST_CYCLES_PER_BYTE;
    let res: Result<(), _> = call_with_payment(
        Principal::management_canister(),
        "bitcoin_send_transaction",
        (SendTransactionRequest {
            transaction,
            network: from_bitcoin_network_to_ic_btc_types_network(network),
        },),
        transaction_cost_cycles,
    )
    .await;

    match res {
        // Return to the caller.
        Ok(()) => Ok(()),

        // The call to `send_transaction` was rejected for a given reason (e.g., not enough cycles were attached to the call).
        Err((rejection_code, message)) => Err(ManagementCanisterReject(rejection_code, message)),
    }
}

/// Sends a transaction, transferring the specified Bitcoin amounts to the provided addresses.
/// When `replaceable` is set to true, the transaction is marked as replaceable using Bitcoin’s replace-by-fee (RBF) mechanism.
/// The `min_confirmations` parameter states that only outputs with at least that many confirmations may be used to construct a transaction.
/// Note that `min_confirmations` = 0 implies that unconfirmed outputs may be used to create a transaction.
/// Further note that the set of UTXO is restricted to those in the updated state: If new UTXOs are discovered when calling `peek_utxos_update` (or `peek_balance_update`), these UTXOs will not be spent in any transaction until they are made available by calling `update_state`.
/// On the other hand, the library is free to choose UTXOs of any managed address when constructing transactions.
pub(crate) async fn multi_transfer(
    multi_transfer_args: MultiTransferArgs,
) -> Result<MultiTransferResult, MultiTransferError> {
    if multi_transfer_args.min_confirmations > MIN_CONFIRMATIONS_UPPER_BOUND {
        return Err(MultiTransferError::MinConfirmationsTooHigh);
    }
    // Retrieves Bitcoin blockchain tip height.
    let tip_height = get_tip_height(&multi_transfer_args).await;

    let utxos_addresses = get_utxos_addresses(&multi_transfer_args, tip_height);

    let built_transaction = get_built_transaction(&multi_transfer_args, &utxos_addresses).await?;

    if built_transaction.fee < built_transaction.mock_signed_transaction_size as u64 {
        return Err(MultiTransferError::FeeTooLow);
    }

    #[cfg(test)]
    let sign_fun = mock_signer;
    #[cfg(not(test))]
    let sign_fun = sign_with_ecdsa;

    // Sign the transaction.
    let signed_transaction = sign_transaction(
        multi_transfer_args.key_name.clone(),
        &get_spending_addresses(&built_transaction),
        &built_transaction.spending_ecdsa_pub_keys,
        built_transaction.transaction,
        sign_fun,
    )
    .await?;

    // Send the transaction to the Bitcoin network.
    let signed_transaction_bytes = signed_transaction.serialize();
    let network = from_types_network_to_bitcoin_network(multi_transfer_args.network);
    send_transaction(signed_transaction_bytes, network).await?;

    let spending_utxos_addresses = built_transaction
        .spending_utxos_addresses
        .into_iter()
        .map(|(address, uxtos)| (get_address_using_primitives(&address), uxtos))
        .collect();

    let txid = signed_transaction.txid();
    let transaction_info = TransactionInfo {
        id: txid.to_string(),
        utxos_addresses: spending_utxos_addresses,
        fee: built_transaction.fee,
        size: signed_transaction.size() as u32,
        timestamp: time(),
    };

    let generated_utxos_addresses =
        get_generated_utxos_addresses(&multi_transfer_args, tip_height, &txid, &transaction_info);

    Ok(MultiTransferResult {
        transaction_info,
        generated_utxos_addresses,
        height: tip_height,
    })
}

/// Returns the Bitcoin blockchain tip height.
async fn get_tip_height(multi_transfer_args: &MultiTransferArgs) -> u32 {
    let tip_height = get_utxos(
        from_types_network_to_bitcoin_network(multi_transfer_args.network),
        &multi_transfer_args.change_address,
        0,
    )
    .await
    .unwrap()
    .tip_height;
    tip_height
}

/// Returns the UTXOs associated with their addresses that may be used to build the transaction.
fn get_utxos_addresses(
    multi_transfer_args: &MultiTransferArgs,
    tip_height: u32,
) -> BTreeMap<Address, Vec<Utxo>> {
    let mut utxos_addresses: BTreeMap<Address, Vec<Utxo>> = multi_transfer_args
        .utxos_state_addresses
        .iter()
        .map(|(address, utxos_state)| (address.clone(), utxos_state.seen_state.clone()))
        .collect();

    utxos_addresses.retain(|address, utxos| {
        // Filter UTXOs, keeping those with enough confirmations and that weren't previously spent in a transaction.
        let spent_txos_address = &multi_transfer_args.utxos_state_addresses[address].spent_state;
        utxos.retain(|utxo| {
            has_utxo_min_confirmations(utxo, tip_height, multi_transfer_args.min_confirmations)
                && !spent_txos_address.contains(&utxo.outpoint)
        });
        // Filter our addresses to only keep the P2PKH ones.
        address.address_type() == Some(AddressType::P2pkh)
    });
    utxos_addresses
}

/// Returns the final unsigned transaction.
async fn get_built_transaction(
    multi_transfer_args: &MultiTransferArgs,
    utxos_addresses: &BTreeMap<Address, Vec<Utxo>>,
) -> Result<BuiltTransaction, MultiTransferError> {
    match multi_transfer_args.fee {
        Fee::Constant(fee) => build_transaction_with_fee(
            &multi_transfer_args.ecdsa_pub_key_addresses,
            utxos_addresses,
            &multi_transfer_args.change_address,
            &multi_transfer_args.payouts,
            fee,
            multi_transfer_args.replaceable,
        ),
        _ => {
            let fee_per_byte = match multi_transfer_args.fee {
                Fee::PerByte(fee_per_byte) => fee_per_byte,
                // This case can't happen see above
                Fee::Constant(_) => panic!(),
                fee_percentile => {
                    get_current_fee(
                        FeeRequest::from(fee_percentile),
                        from_types_network_to_bitcoin_network(multi_transfer_args.network),
                    )
                    .await?
                }
            };
            build_transaction(
                multi_transfer_args.key_name.clone(),
                &multi_transfer_args.ecdsa_pub_key_addresses,
                utxos_addresses,
                &multi_transfer_args.change_address,
                &multi_transfer_args.payouts,
                fee_per_byte,
                multi_transfer_args.replaceable,
            )
            .await
        }
    }
}

/// Returns the generated UTXOs in the built transaction.
fn get_generated_utxos_addresses(
    multi_transfer_args: &MultiTransferArgs,
    tip_height: u32,
    txid: &Txid,
    transaction_info: &TransactionInfo,
) -> BTreeMap<AddressUsingPrimitives, Vec<Utxo>> {
    let mut generated_utxos_addresses = BTreeMap::default();
    let mut vout = 0;
    multi_transfer_args
        .payouts
        .iter()
        .for_each(|(address, value)| {
            let utxo = Utxo {
                outpoint: ic_btc_types::OutPoint {
                    txid: txid.to_vec(),
                    vout,
                },
                value: *value,
                height: tip_height,
            };
            generated_utxos_addresses
                .entry(get_address_using_primitives(address))
                .or_insert_with(Vec::new)
                .push(utxo);
            vout += 1;
        });
    let total_spent: Satoshi = transaction_info
        .utxos_addresses
        .iter()
        .map(|(_, utxos)| utxos.iter().map(|utxo| utxo.value).sum::<Satoshi>())
        .sum();
    let total_amount: Satoshi = multi_transfer_args.payouts.values().sum();
    let change_amount = total_spent - total_amount - transaction_info.fee;
    if change_amount > DUST_THRESHOLD {
        generated_utxos_addresses
            .entry(get_address_using_primitives(
                &multi_transfer_args.change_address,
            ))
            .or_insert_with(Vec::new)
            .push(Utxo {
                outpoint: ic_btc_types::OutPoint {
                    txid: txid.to_vec(),
                    vout,
                },
                value: change_amount,
                height: tip_height,
            });
    }
    generated_utxos_addresses
}

pub(crate) fn time() -> u64 {
    if cfg!(test) {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    } else {
        ic_cdk::api::time()
    }
}

// Builds a transaction to send the given `amount` of satoshis to the
// destination address.
async fn build_transaction(
    key_name: String,
    ecdsa_pub_key_addresses: &BTreeMap<Address, EcdsaPubKey>,
    utxos_addresses: &BTreeMap<Address, Vec<Utxo>>,
    change_address: &Address,
    payouts: &BTreeMap<Address, Satoshi>,
    fee_per_byte: MillisatoshiPerByte,
    replaceable: bool,
) -> Result<BuiltTransaction, MultiTransferError> {
    // We have a chicken-and-egg problem where we need to know the size
    // of the transaction in order to compute its proper fee, but we need
    // to know the proper fee in order to figure out the inputs needed for
    // the transaction.
    //
    // We solve this problem iteratively. We start with a fee of zero, build
    // and sign a transaction, see what its size is, and then update the fee,
    // rebuild the transaction, until the fee is set to the correct amount.
    let mut total_fee = 0;
    loop {
        let mut built_transaction = build_transaction_with_fee(
            ecdsa_pub_key_addresses,
            utxos_addresses,
            change_address,
            payouts,
            total_fee,
            replaceable,
        )?;

        // Sign the transaction. In this case, we only care about the size
        // of the signed transaction, so we use a mock signer here for efficiency.
        let signed_transaction = sign_transaction(
            key_name.clone(),
            &get_spending_addresses(&built_transaction),
            &built_transaction.spending_ecdsa_pub_keys,
            built_transaction.transaction.clone(),
            mock_signer,
        )
        .await?;

        let signed_tx_bytes_len = signed_transaction.serialize().len() as u64;

        if (signed_tx_bytes_len * fee_per_byte) / 1000 == total_fee {
            built_transaction.mock_signed_transaction_size = signed_tx_bytes_len;
            return Ok(built_transaction);
        } else {
            total_fee = (signed_tx_bytes_len * fee_per_byte) / 1000;
        }
    }
}

/// Builds a transaction that sends the given `payouts` amounts of satoshis to the given `payouts` addresses.
/// Sends back the change to `change_address`.
fn build_transaction_with_fee(
    ecdsa_pub_key_addresses: &BTreeMap<Address, EcdsaPubKey>,
    utxos_addresses: &BTreeMap<Address, Vec<Utxo>>,
    change_address: &Address,
    payouts: &BTreeMap<Address, Satoshi>,
    fee: Satoshi,
    replaceable: bool,
) -> Result<BuiltTransaction, MultiTransferError> {
    // TODO (FI-313): Add smarter coin selection
    // Select which UTXOs to spend. For now, we naively spend the first available UTXOs.
    let mut spending_utxos_addresses = BTreeMap::default();
    let mut spending_ecdsa_pub_keys = vec![];
    let mut inputs: Vec<TxIn> = vec![];
    let mut total_spent = 0;
    let total_amount: Satoshi = payouts.values().sum();
    'select_utxos: for (address, utxos) in utxos_addresses.iter() {
        for utxo in utxos.iter() {
            total_spent += utxo.value;
            spending_utxos_addresses
                .entry(address.clone())
                .or_insert_with(Vec::new)
                .push(utxo.clone());
            spending_ecdsa_pub_keys.push(ecdsa_pub_key_addresses[address].clone());
            inputs.push(TxIn {
                previous_output: OutPoint {
                    txid: Txid::from_hash(Hash::from_slice(&utxo.outpoint.txid).unwrap()),
                    vout: utxo.outpoint.vout,
                },
                sequence: if replaceable {
                    // If `replaceable`, then enable Replace-By-Fee according to BIP 125.
                    0x00000000
                } else {
                    0xffffffff
                },
                witness: Witness::new(),
                script_sig: Script::new(),
            });
            if total_spent >= total_amount + fee {
                break 'select_utxos;
            }
        }
    }

    if total_spent < total_amount + fee {
        return Err(MultiTransferError::InsufficientBalance);
    }

    let mut outputs: Vec<TxOut> = payouts
        .iter()
        .map(|(address, amount)| TxOut {
            script_pubkey: address.script_pubkey(),
            value: *amount,
        })
        .collect();

    let remaining_amount = total_spent - total_amount - fee;

    // Assume that any amount below this threshold is dust.
    if remaining_amount > DUST_THRESHOLD {
        outputs.push(TxOut {
            script_pubkey: change_address.script_pubkey(),
            value: remaining_amount,
        });
    }

    let transaction = Transaction {
        input: inputs,
        output: outputs,
        lock_time: 0,
        version: 2,
    };

    Ok(BuiltTransaction {
        transaction,
        mock_signed_transaction_size: 0,
        spending_utxos_addresses,
        spending_ecdsa_pub_keys,
        fee,
    })
}

/// Sign a Bitcoin transaction given the addresses of the funds and the change address.
///
/// Constraint:
/// * All the inputs are referencing outpoints that are owned by managed supported addresses.
async fn sign_transaction<SignFun, Fut>(
    key_name: String,
    addresses: &[Address],
    ecdsa_pub_keys: &[EcdsaPubKey],
    mut transaction: Transaction,
    signer: SignFun,
) -> Result<Transaction, ManagementCanisterReject>
where
    SignFun: Fn(String, Vec<Vec<u8>>, Vec<u8>) -> Fut,
    Fut: Future<Output = Result<Vec<u8>, ManagementCanisterReject>>,
{
    let txclone = transaction.clone();
    for (index, input) in transaction.input.iter_mut().enumerate() {
        let address = &addresses[index];
        let sighash =
            txclone.signature_hash(index, &address.script_pubkey(), SIG_HASH_TYPE.to_u32());

        let ecdsa_pub_key = &ecdsa_pub_keys[index];
        let signature = signer(
            key_name.clone(),
            ecdsa_pub_key.derivation_path.clone(),
            sighash.to_vec(),
        )
        .await?;

        // Convert signature to DER.
        let der_signature = sec1_to_der(signature);

        let mut sig_with_hashtype = der_signature;
        sig_with_hashtype.push(SIG_HASH_TYPE.to_u32() as u8);
        input.script_sig = Builder::new()
            .push_slice(sig_with_hashtype.as_slice())
            .push_slice(&ecdsa_pub_key.public_key)
            .into_script();
    }

    Ok(transaction)
}

// A mock for rubber-stamping ECDSA signatures.
async fn mock_signer(
    _key_name: String,
    _derivation_path: Vec<Vec<u8>>,
    _message_hash: Vec<u8>,
) -> Result<Vec<u8>, ManagementCanisterReject> {
    Ok(vec![1; 64])
}

// Converts a SEC1 ECDSA signature to the DER format.
fn sec1_to_der(sec1_signature: Vec<u8>) -> Vec<u8> {
    let r: Vec<u8> = if sec1_signature[0] & 0x80 != 0 {
        // r is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[..32].to_vec());
        tmp
    } else {
        // r is positive.
        sec1_signature[..32].to_vec()
    };

    let s: Vec<u8> = if sec1_signature[32] & 0x80 != 0 {
        // s is negative. Prepend a zero byte.
        let mut tmp = vec![0x00];
        tmp.extend(sec1_signature[32..].to_vec());
        tmp
    } else {
        // s is positive.
        sec1_signature[32..].to_vec()
    };

    // Convert signature to DER.
    vec![
        vec![0x30, 4 + r.len() as u8 + s.len() as u8, 0x02, r.len() as u8],
        r,
        vec![0x02, s.len() as u8],
        s,
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// Returns the spending addresses from a given built transaction.
fn get_spending_addresses(built_transaction: &BuiltTransaction) -> Vec<Address> {
    let mut spending_addresses: Vec<Address> = vec![];
    built_transaction
        .spending_utxos_addresses
        .iter()
        .for_each(|(address, spending_utxos)| {
            spending_addresses.append(&mut vec![address.clone(); spending_utxos.len()])
        });
    spending_addresses
}
