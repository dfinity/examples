use crate::{
    address_management,
    address_management::get_main_address,
    canister_common::ManagementCanister,
    ecdsa::{get_btc_ecdsa_public_key, get_key_name_from_network},
    transaction_management,
    transaction_management::get_current_fees,
    types::{from_bitcoin_network_to_types_network, GetUtxosResponse},
    types::{
        AddAddressWithParametersError, AddressNotTracked, AddressType, BalanceUpdate,
        CurrentFeesArgs, DerivationPathTooLong, EcdsaPubKey, Fee, GetUtxosError,
        InitializationParametersArgs, ManagementCanisterReject, MinConfirmationsTooHigh,
        MultiTransferArgs, MultiTransferError, MultiTransferResult, UtxosArgs, UtxosResult,
        UtxosState, UtxosUpdate, MIN_CONFIRMATIONS_UPPER_BOUND,
    },
    upgrade_management::get_address,
    utxo_management,
    utxo_management::{get_balance_from_utxos, get_utxos},
};
use ic_btc_types::{MillisatoshiPerByte, OutPoint, Satoshi, Utxo};

use bitcoin::Address;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub struct BitcoinAgent<C: ManagementCanister> {
    pub(crate) management_canister: C,
    pub(crate) main_address_type: AddressType,
    pub(crate) ecdsa_pub_key_addresses: BTreeMap<Address, EcdsaPubKey>,
    pub(crate) min_confirmations: u32,
    pub(crate) utxos_state_addresses: BTreeMap<Address, UtxosState>,
}

impl<C: ManagementCanister> BitcoinAgent<C> {
    /// Creates a new Bitcoin agent using the given management canister.
    pub fn new(
        management_canister: C,
        main_address_type: &AddressType,
        min_confirmations: u32,
    ) -> Result<Self, MinConfirmationsTooHigh> {
        if min_confirmations > MIN_CONFIRMATIONS_UPPER_BOUND {
            return Err(MinConfirmationsTooHigh);
        }
        Ok(Self {
            management_canister,
            main_address_type: *main_address_type,
            ecdsa_pub_key_addresses: BTreeMap::default(),
            utxos_state_addresses: BTreeMap::default(),
            min_confirmations,
        })
    }

    /// Adds an address based on the provided derivation path and address type to the list of managed addresses.
    /// A minimum number of confirmations must further be specified, which is used when calling `get_utxos` and `get_balance`.
    /// Returns the derived address if the operation is successful and an error otherwise.
    pub fn add_address_with_parameters(
        &mut self,
        derivation_path: &[Vec<u8>],
        address_type: &AddressType,
        min_confirmations: u32,
    ) -> Result<Address, AddAddressWithParametersError> {
        address_management::add_address_with_parameters(
            self,
            derivation_path,
            address_type,
            min_confirmations,
        )
    }

    /// Adds an address to the agent with the provided derivation path.
    /// The default address type and default number of confirmations are used.
    pub fn add_address(
        &mut self,
        derivation_path: &[Vec<u8>],
    ) -> Result<Address, DerivationPathTooLong> {
        let address_type = self.main_address_type;
        match self.add_address_with_parameters(
            derivation_path,
            &address_type,
            self.min_confirmations,
        ) {
            Err(AddAddressWithParametersError::DerivationPathTooLong) => Err(DerivationPathTooLong),
            Ok(address) => Ok(address),
            // Other case AddAddressWithParameters::MinConfirmationsTooHigh can't happen see BitcoinAgent::new
            _ => panic!(),
        }
    }

    /// Returns the difference in the balance of an address controlled by the `BitcoinAgent` between the current state and the seen state when the function was last called, considering only transactions with the specified number of confirmations.
    /// The returned `BalanceUpdate` contains the information on how much balance was added and subtracted in total. If the function is called for the first time, the current balance of the address is returned.
    /// It is equivalent to calling `get_utxos_update` and summing up the balances in the returned UTXOs.
    pub fn get_balance_update(
        &mut self,
        address: &Address,
    ) -> Result<BalanceUpdate, AddressNotTracked> {
        utxo_management::get_balance_update(self, address)
    }

    // ---
    // Usage pattern to update the utxos state of the agent (eg. with thread_local agents):
    // let args = AGENT.with(|s| s.borrow().get_utxos_args(address));
    // let result = get_utxos_from_args(args).await.unwrap();
    // let utxos = AGENT.with(|s| s.borrow_mut().apply_utxos(result));

    pub fn get_utxos_args(&self, address: &Address, min_confirmations: u32) -> UtxosArgs {
        UtxosArgs {
            network: self.management_canister.get_network(),
            address: address.clone(),
            min_confirmations,
            utxos_state: self
                .utxos_state_addresses
                .get(address)
                .unwrap_or(&UtxosState::new(min_confirmations))
                .clone(),
        }
    }

    pub fn apply_utxos(&mut self, utxos_result: UtxosResult) -> UtxosUpdate {
        let mut utxos_state_address = self
            .utxos_state_addresses
            .get_mut(&utxos_result.address)
            .unwrap();
        utxos_state_address.unseen_state = utxos_result.utxos;
        UtxosUpdate::from_state(
            &utxos_state_address.seen_state,
            &utxos_state_address.unseen_state,
        )
    }

    pub fn get_current_fees_args(&self) -> CurrentFeesArgs {
        CurrentFeesArgs {
            network: self.management_canister.get_network(),
        }
    }

    pub fn get_initialization_parameters_args(&self) -> InitializationParametersArgs {
        InitializationParametersArgs {
            key_name: get_key_name_from_network(self.management_canister.get_network()),
            ecdsa_public_key: self.management_canister.get_ecdsa_public_key(),
        }
    }

    /// Initializes the Bitcoin agent by setting its ECDSA public key.
    pub fn initialize(&mut self, ecdsa_public_key: EcdsaPubKey) {
        self.management_canister
            .set_ecdsa_public_key(ecdsa_public_key);
        let main_address = get_main_address(&self.management_canister, &self.main_address_type);
        self.ecdsa_pub_key_addresses = BTreeMap::from([(
            main_address.clone(),
            self.management_canister.get_ecdsa_public_key(),
        )]);
        self.utxos_state_addresses =
            BTreeMap::from([(main_address, UtxosState::new(self.min_confirmations))]);
    }

    /// Returns arguments to send a transaction, transferring the specified Bitcoin amounts to the provided addresses.
    /// When `replaceable` is set to true, the transaction is marked as replaceable using Bitcoin's replace-by-fee (RBF) mechanism.
    /// The `min_confirmations` parameter states that only outputs with at least that many confirmations may be used to construct a transaction.
    /// Note that `min_confirmations` = 0 implies that unconfirmed outputs may be used to create a transaction.
    /// Further note that the set of UTXO is restricted to those in the updated state: If new UTXOs are discovered when calling `peek_utxos_update` (or `peek_balance_update`), these UTXOs will not be spent in any transaction until they are made available by calling `update_state`.
    /// On the other hand, the library is free to choose UTXOs of any managed address when constructing transactions.
    /// Also note that the library verifies if the final fee is at least 1 sat/B.
    pub fn get_multi_transfer_args(
        &self,
        payouts: &BTreeMap<Address, Satoshi>,
        change_address: &Address,
        fee: Fee,
        min_confirmations: u32,
        replaceable: bool,
    ) -> MultiTransferArgs {
        MultiTransferArgs {
            key_name: get_key_name_from_network(self.management_canister.get_network()),
            ecdsa_pub_key_addresses: self.ecdsa_pub_key_addresses.clone(),
            utxos_state_addresses: self.utxos_state_addresses.clone(),
            payouts: payouts.clone(),
            change_address: change_address.clone(),
            fee,
            min_confirmations,
            replaceable,
            network: from_bitcoin_network_to_types_network(self.management_canister.get_network()),
        }
    }

    /// Caches the spent and generated outputs to build valid future transactions even with `min_confirmations = 0`.
    pub fn apply_multi_transfer_result(&mut self, multi_transfer_result: &MultiTransferResult) {
        // Cache the spent outputs to not use them for future transactions.
        multi_transfer_result
            .transaction_info
            .utxos_addresses
            .clone()
            .into_iter()
            .for_each(|(address_using_primitives, utxos)| {
                let address = get_address(address_using_primitives);
                utxos.iter().for_each(|utxo| {
                    self.utxos_state_addresses
                        .get_mut(&address)
                        .unwrap()
                        .spent_state
                        .push(utxo.outpoint.clone())
                })
            });
        // Cache the generated outputs to be able to use them for future transactions.
        multi_transfer_result
            .generated_utxos_addresses
            .clone()
            .into_iter()
            .for_each(|(address_using_primitives, mut utxos)| {
                let address = get_address(address_using_primitives);
                if self.utxos_state_addresses.get(&address).is_none() {
                    self.utxos_state_addresses
                        .insert(address.clone(), UtxosState::new(0));
                }
                let utxos_state_address = self.utxos_state_addresses.get_mut(&address).unwrap();
                utxos_state_address.generated_state.append(&mut utxos);
            })
    }
}

pub async fn multi_transfer_from_args(
    multi_transfer_args: MultiTransferArgs,
) -> Result<MultiTransferResult, MultiTransferError> {
    transaction_management::multi_transfer(multi_transfer_args).await
}

pub async fn get_initialization_parameters_from_args(
    initialization_parameters_args: InitializationParametersArgs,
) -> Result<EcdsaPubKey, ManagementCanisterReject> {
    if initialization_parameters_args
        .ecdsa_public_key
        .public_key
        .is_empty()
    {
        get_btc_ecdsa_public_key(&initialization_parameters_args.key_name).await
    } else {
        Ok(initialization_parameters_args.ecdsa_public_key)
    }
}

/// Modify the provided `GetUtxosResponse` to remove spent UTXOs and add generated UTXOs if using `min_confirmations = 0`.
fn get_utxos_from_args_common(
    address: &Address,
    get_utxos_response: GetUtxosResponse,
    utxos_state: UtxosState,
) -> Result<UtxosResult, GetUtxosError> {
    let utxos = if utxos_state.min_confirmations == 0 {
        let mut utxos: Vec<Utxo> = get_utxos_response.utxos;
        utxos.append(&mut utxos_state.generated_state.clone());
        utxos.retain(|utxo| {
            utxos_state
                .spent_state
                .iter()
                .all(|spent_outpoint| utxo.outpoint != spent_outpoint.clone())
        });
        // Remove any duplicated UTXOs with a possible different height, keeping the UTXO with the heighest height.
        // Likewise if a UTXO was generated at height `n` thanks to a sent transaction, if the transaction is confirmed, the UTXO return by this function won't have its height still be `n` but the actual one.
        let mut utxos_occurrences: HashMap<OutPoint, Utxo> = HashMap::default();
        utxos.into_iter().for_each(|utxo| {
            if let Some(utxo_occurrence) = utxos_occurrences.get(&utxo.outpoint) {
                if utxo.height > utxo_occurrence.height {
                    utxos_occurrences.insert(utxo.outpoint.clone(), utxo);
                }
            } else {
                utxos_occurrences.insert(utxo.outpoint.clone(), utxo);
            }
        });
        utxos_occurrences.values().cloned().collect()
    } else {
        get_utxos_response.utxos
    };

    Ok(UtxosResult {
        address: address.clone(),
        utxos,
        tip_height: get_utxos_response.tip_height,
    })
}

pub async fn get_utxos_from_args(utxos_args: UtxosArgs) -> Result<UtxosResult, GetUtxosError> {
    get_utxos_from_args_common(
        &utxos_args.address,
        get_utxos(
            utxos_args.network,
            &utxos_args.address,
            utxos_args.min_confirmations,
        )
        .await?,
        utxos_args.utxos_state,
    )
}

/// Returns the balance of the given Bitcoin `address` according to `min_confirmations`.
pub async fn get_balance_from_args(utxos_args: UtxosArgs) -> Result<Satoshi, GetUtxosError> {
    Ok(get_balance_from_utxos(
        &get_utxos_from_args(utxos_args).await?.utxos,
    ))
}

/// Returns fees as percentiles in millisatoshis/byte over the last 10,000 transactions.
pub async fn get_current_fees_from_args(
    current_fees_args: CurrentFeesArgs,
) -> Result<Vec<MillisatoshiPerByte>, ManagementCanisterReject> {
    get_current_fees(current_fees_args.network).await
}
