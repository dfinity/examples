use crate::{
    wallet::BitcoinWallet,
    bip32_extended_derivation::extended_bip32_derivation,
    canister_common::ManagementCanister,
    types::{
        AddAddressWithParametersError, BitcoinAddressError, EcdsaPubKey, UtxosState,
        MIN_CONFIRMATIONS_UPPER_BOUND,
    },
};
use bitcoin::{
    blockdata::{opcodes, script::Builder},
    hashes,
    hashes::Hash,
    util,
    util::address::Payload,
    Address, AddressType, Network, PublicKey, ScriptHash,
};

/// Returns the Bitcoin public key from a given ECDSA public key.
pub(crate) fn get_btc_public_key_from_ecdsa_public_key(
    ecdsa_public_key: &EcdsaPubKey,
) -> Result<PublicKey, bitcoin::util::key::Error> {
    PublicKey::from_slice(&ecdsa_public_key.public_key)
}

/// Adds an address based on the provided derivation path and address type to the list of managed addresses.
/// A minimum number of confirmations must further be specified, which is used when calling `get_utxos` and `get_balance`.
/// Returns the derived address if the operation is successful and an error otherwise.
pub(crate) fn add_address_with_parameters(
    bitcoin_wallet: &mut BitcoinWallet<impl ManagementCanister>,
    derivation_path: &[Vec<u8>],
    address_type: &crate::AddressType,
    min_confirmations: u32,
) -> Result<Address, AddAddressWithParametersError> {
    if min_confirmations > MIN_CONFIRMATIONS_UPPER_BOUND {
        return Err(AddAddressWithParametersError::MinConfirmationsTooHigh);
    }
    if derivation_path.len() > 255 {
        return Err(AddAddressWithParametersError::DerivationPathTooLong);
    }
    let address = add_address_from_extended_path(
        bitcoin_wallet,
        derivation_path,
        address_type,
        min_confirmations,
    );
    Ok(address)
}

/// Returns the public key and address of the derived child from the given
/// public key, chain code, derivation path, address type, and network.
pub(crate) fn derive_ecdsa_public_key_and_address_from_extended_path(
    derivation_path: &[Vec<u8>],
    address_type: &crate::AddressType,
    network: &Network,
    ecdsa_public_key: &EcdsaPubKey,
) -> (EcdsaPubKey, Address) {
    let (child_public_key, child_chain_code) = extended_bip32_derivation(
        &ecdsa_public_key.public_key,
        &ecdsa_public_key.chain_code,
        derivation_path,
    );

    let child_ecdsa_public_key = EcdsaPubKey {
        public_key: child_public_key,
        chain_code: child_chain_code,
        derivation_path: ecdsa_public_key
            .derivation_path
            .iter()
            .cloned()
            .chain(derivation_path.iter().cloned())
            .collect(),
    };
    let child_address = get_address(network, address_type, &child_ecdsa_public_key).unwrap();

    (child_ecdsa_public_key, child_address)
}

/// Adds the address for the given extended derivation path and address type to the given
/// Bitcoin wallet if the derived address is not already managed.
/// This function assumes that the passed derivation path is an extended path.
/// This assumption has to be checked in the caller function.
pub(crate) fn add_address_from_extended_path(
    bitcoin_wallet: &mut BitcoinWallet<impl ManagementCanister>,
    derivation_path: &[Vec<u8>],
    address_type: &crate::AddressType,
    min_confirmations: u32,
) -> Address {
    let (ecdsa_public_key, address) = derive_ecdsa_public_key_and_address_from_extended_path(
        derivation_path,
        address_type,
        &bitcoin_wallet.management_canister.get_network(),
        &bitcoin_wallet.management_canister.get_ecdsa_public_key(),
    );
    if !bitcoin_wallet.ecdsa_pub_key_addresses.contains_key(&address) {
        bitcoin_wallet
            .ecdsa_pub_key_addresses
            .insert(address.clone(), ecdsa_public_key);
        let utxos_state = UtxosState::new(min_confirmations);
        bitcoin_wallet
            .utxos_state_addresses
            .insert(address.clone(), utxos_state);
    }
    address
}

/// Returns the P2PKH address from a given network and public key.
pub(crate) fn get_p2pkh_address(
    network: &Network,
    ecdsa_public_key: &EcdsaPubKey,
) -> Result<Address, util::key::Error> {
    Ok(Address::p2pkh(
        &get_btc_public_key_from_ecdsa_public_key(ecdsa_public_key)?,
        *network,
    ))
}

/// Returns the P2SH address from a given network and script hash.
pub(crate) fn get_p2sh_address(
    network: &Network,
    script_hash: &[u8],
) -> Result<Address, hashes::error::Error> {
    Ok(Address {
        network: *network,
        payload: Payload::ScriptHash(ScriptHash::from_slice(script_hash)?),
    })
}

/// Returns the P2SH address from a given network and public key.
pub(crate) fn get_p2sh_address_for_pub_key(
    network: &Network,
    ecdsa_public_key: &EcdsaPubKey,
) -> Result<Address, BitcoinAddressError> {
    let public_key = get_btc_public_key_from_ecdsa_public_key(ecdsa_public_key)?;
    let public_key_hash = public_key.pubkey_hash();
    let script = Builder::new()
        .push_slice(&public_key_hash[..])
        .push_opcode(opcodes::all::OP_CHECKSIG)
        .into_script();
    Ok(get_p2sh_address(
        network,
        &script.script_hash().to_ascii_lowercase(),
    )?)
}

/// Returns the P2WPKH address from a given network and public key.
pub(crate) fn get_p2wpkh_address(
    network: &Network,
    ecdsa_public_key: &EcdsaPubKey,
) -> Result<Address, BitcoinAddressError> {
    Ok(Address::p2wpkh(
        &get_btc_public_key_from_ecdsa_public_key(ecdsa_public_key)?,
        *network,
    )?)
}

/// Returns the Bitcoin address from a given network, address type and ECDSA public key.
fn get_address(
    network: &Network,
    address_type: &crate::AddressType,
    ecdsa_public_key: &EcdsaPubKey,
) -> Result<Address, BitcoinAddressError> {
    match get_bitcoin_address_type(address_type) {
        AddressType::P2pkh => Ok(get_p2pkh_address(network, ecdsa_public_key)?),
        AddressType::P2sh => get_p2sh_address_for_pub_key(network, ecdsa_public_key),
        AddressType::P2wpkh => get_p2wpkh_address(network, ecdsa_public_key),
        // Other cases can't happen, see BitcoinWallet::new.
        _ => panic!(),
    }
}

/// Returns the Bitcoin address for a given network, address type, and ECDSA public key.
pub(crate) fn get_main_address(
    management_canister: &impl ManagementCanister,
    address_type: &crate::AddressType,
) -> Address {
    get_address(
        &management_canister.get_network(),
        address_type,
        &management_canister.get_ecdsa_public_key(),
    )
    .unwrap()
}

/// Returns the bitcoin::AddressType converted from an crate::AddressType
pub(crate) fn get_bitcoin_address_type(address_type: &crate::AddressType) -> AddressType {
    match address_type {
        crate::AddressType::P2pkh => AddressType::P2pkh,
        crate::AddressType::P2sh => AddressType::P2sh,
        crate::AddressType::P2wpkh => AddressType::P2wpkh,
    }
}
