use crate::{
    canister_common::{ManagementCanister, GET_UTXOS_COST_CYCLES},
    types::{
        from_bitcoin_network_to_ic_btc_types_network, AddressNotTracked, BalanceUpdate,
        GetUtxosError, GetUtxosResponse, UtxosUpdate, MIN_CONFIRMATIONS_UPPER_BOUND,
    },
    wallet::BitcoinWallet,
};
use bitcoin::{Address, Network};
use ic_btc_types::{
    GetUtxosRequest, Satoshi, Utxo,
    UtxosFilter::{MinConfirmations, Page},
};
use ic_cdk::{api::call::call_with_payment, export::Principal};

/// Returns the actual UTXOs of the given Bitcoin `address` according to `min_confirmations`.
pub(crate) async fn get_utxos(
    network: Network,
    address: &Address,
    min_confirmations: u32,
) -> Result<GetUtxosResponse, GetUtxosError> {
    if min_confirmations > MIN_CONFIRMATIONS_UPPER_BOUND {
        return Err(GetUtxosError::MinConfirmationsTooHigh);
    }
    let mut filter = Some(MinConfirmations(min_confirmations));
    let mut utxos = vec![];
    let tip_height;
    loop {
        let res: Result<(ic_btc_types::GetUtxosResponse,), _> = call_with_payment(
            Principal::management_canister(),
            "bitcoin_get_utxos",
            (GetUtxosRequest {
                address: address.to_string(),
                network: from_bitcoin_network_to_ic_btc_types_network(network),
                filter,
            },),
            GET_UTXOS_COST_CYCLES,
        )
        .await;

        match res {
            Ok((mut get_utxos_response,)) => {
                utxos.append(&mut get_utxos_response.utxos);
                if get_utxos_response.next_page.is_none() {
                    tip_height = get_utxos_response.tip_height;
                    break;
                } else {
                    filter = get_utxos_response.next_page.map(Page);
                }
            }

            // The call to `get_utxos` was rejected for a given reason (e.g., not enough cycles were attached to the call).
            Err((rejection_code, message)) => {
                return Err(GetUtxosError::ManagementCanisterReject(
                    rejection_code,
                    message,
                ))
            }
        }
    }

    Ok(GetUtxosResponse { utxos, tip_height })
}

/// Returns the difference between the current UTXO state and the last seen state for this address.
/// The last seen state for an address is updated to the current unseen state by calling `update_state` or implicitly when invoking `get_utxos_update`.
/// If there are no changes to the UTXO set since the last call, the returned `UtxosUpdate` will be identical.
pub(crate) fn peek_utxos_update<C: ManagementCanister>(
    bitcoin_wallet: &BitcoinWallet<C>,
    address: &Address,
) -> Result<UtxosUpdate, AddressNotTracked> {
    if !bitcoin_wallet.utxos_state_addresses.contains_key(address) {
        return Err(AddressNotTracked);
    }
    let utxos_state_address = bitcoin_wallet.utxos_state_addresses.get(address).unwrap();
    Ok(UtxosUpdate::from_state(
        &utxos_state_address.seen_state,
        &utxos_state_address.unseen_state,
    ))
}

/// Updates the state of the `BitcoinWallet` for the given `address`.
/// This function doesn't invoke a Bitcoin integration API function.
pub(crate) fn update_state<C: ManagementCanister>(
    bitcoin_wallet: &mut BitcoinWallet<C>,
    address: &Address,
) -> Result<(), AddressNotTracked> {
    if !bitcoin_wallet.utxos_state_addresses.contains_key(address) {
        return Err(AddressNotTracked);
    }
    let unseen_state = bitcoin_wallet.utxos_state_addresses[address]
        .unseen_state
        .clone();
    bitcoin_wallet
        .utxos_state_addresses
        .get_mut(address)
        .unwrap()
        .seen_state = unseen_state;
    Ok(())
}

/// Returns the difference in the set of UTXOs of an address controlled by the `BitcoinWallet` between the current state and the seen state when the function was last called, considering only UTXOs with the number of confirmations specified when adding the given address.
/// The returned `UtxosUpdate` contains the information which UTXOs were added and removed. If the function is called for the first time, the current set of UTXOs is returned.
/// Note that the function changes the state of the `BitcoinWallet`: A subsequent call will return changes to the UTXO set that have occurred since the last call.
pub(crate) fn get_utxos_update<C: ManagementCanister>(
    bitcoin_wallet: &mut BitcoinWallet<C>,
    address: &Address,
) -> Result<UtxosUpdate, AddressNotTracked> {
    let utxos_update = peek_utxos_update(bitcoin_wallet, address)?;
    update_state(bitcoin_wallet, address).unwrap();
    Ok(utxos_update)
}

/// Returns the total value of a UTXOs set.
pub(crate) fn get_balance_from_utxos(utxos: &[Utxo]) -> Satoshi {
    utxos.iter().map(|utxo| utxo.value).sum()
}

/// Returns the difference in the balance of an address controlled by the `BitcoinWallet` between the current state and the seen state when the function was last called, considering only transactions with the specified number of confirmations.
/// The returned `BalanceUpdate` contains the information on how much balance was added and subtracted in total. If the function is called for the first time, the current balance of the address is returned.
/// It is equivalent to calling `get_utxos_update` and summing up the balances in the returned UTXOs.
pub(crate) fn get_balance_update<C: ManagementCanister>(
    bitcoin_wallet: &mut BitcoinWallet<C>,
    address: &Address,
) -> Result<BalanceUpdate, AddressNotTracked> {
    let utxos_update = get_utxos_update(bitcoin_wallet, address)?;
    Ok(BalanceUpdate::from(utxos_update))
}

/// Returns whether or not a given UTXO has been confirmed `min_confirmations` times according to current `tip_height`.
pub(crate) fn has_utxo_min_confirmations(
    utxo: &Utxo,
    tip_height: u32,
    min_confirmations: u32,
) -> bool {
    utxo.height <= tip_height + 1 - min_confirmations
}
