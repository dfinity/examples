use crate::{
    canister_common::ManagementCanister,
    ecdsa,
    ecdsa::get_key_name_from_network,
    transaction_management,
    types::{
        from_types_network_to_bitcoin_network, EcdsaPubKey, GetUtxosError, GetUtxosResponse,
        ManagementCanisterReject,
    },
    utxo_management,
};
use ic_btc_types::MillisatoshiPerByte;

use async_trait::async_trait;
use bitcoin::{Address, Network};

/// The real management canister is used to provide actual interaction with the Bitcoin integration.
#[derive(Clone)]
pub struct ManagementCanisterImpl {
    network: Network,
    ecdsa_public_key: EcdsaPubKey,
}

#[async_trait]
impl ManagementCanister for ManagementCanisterImpl {
    /// Creates a new instance of the real management canister.
    fn new(network: crate::types::Network) -> Self {
        Self::new_using_ecdsa_public_key(
            network,
            EcdsaPubKey {
                public_key: vec![],
                chain_code: vec![],
                derivation_path: vec![],
            },
        )
    }

    /// Creates a new instance of the real management canister using the given ECDSA public key.
    fn new_using_ecdsa_public_key(
        network: crate::types::Network,
        ecdsa_public_key: EcdsaPubKey,
    ) -> Self {
        Self {
            network: from_types_network_to_bitcoin_network(network),
            ecdsa_public_key,
        }
    }

    /// Initializes the management canister by initializing its ECDSA public key.
    fn set_ecdsa_public_key(&mut self, ecdsa_public_key: EcdsaPubKey) {
        self.ecdsa_public_key = ecdsa_public_key;
    }

    /// Returns the network the management canister interacts with.
    fn get_network(&self) -> Network {
        self.network
    }

    /// Returns the ECDSA public key of this canister.
    fn get_ecdsa_public_key(&self) -> EcdsaPubKey {
        self.ecdsa_public_key.clone()
    }

    /// Returns the UTXOs of the given Bitcoin `address` according to `min_confirmations`.
    /// This getter always return the same value until a block, with transactions concerning the address, is mined.
    async fn get_utxos(
        &self,
        address: &Address,
        min_confirmations: u32,
    ) -> Result<GetUtxosResponse, GetUtxosError> {
        utxo_management::get_utxos(self.network, address, min_confirmations).await
    }

    /// Returns fees as percentiles in millisatoshis/byte over the last 10,000 transactions.
    async fn get_current_fees(&self) -> Result<Vec<MillisatoshiPerByte>, ManagementCanisterReject> {
        transaction_management::get_current_fees(self.get_network()).await
    }

    /// Returns the signature of the given `message_hash` associated with the ECDSA public key of this canister at the given derivation path.
    async fn sign_with_ecdsa(
        &self,
        derivation_path: &[Vec<u8>],
        message_hash: &[u8],
    ) -> Result<Vec<u8>, ManagementCanisterReject> {
        ecdsa::sign_with_ecdsa(
            get_key_name_from_network(self.network),
            derivation_path.to_vec(),
            message_hash.to_vec(),
        )
        .await
    }

    /// Sends the given transaction to the network the management canister interacts with.
    async fn send_transaction(
        &mut self,
        transaction: Vec<u8>,
        network: Network,
    ) -> Result<(), ManagementCanisterReject> {
        transaction_management::send_transaction(transaction, network).await
    }
}
