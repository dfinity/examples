use crate::types::{EcdsaPubKey, GetUtxosError, GetUtxosResponse, ManagementCanisterReject};
use async_trait::async_trait;
use bitcoin::{Address, Network};
use ic_btc_types::MillisatoshiPerByte;

const MILLION: u64 = 1_000_000; // One million
const BILLION: u64 = 1_000_000_000; // One billion

// Fees for the various Bitcoin endpoints.
pub(crate) const GET_UTXOS_COST_CYCLES: u64 = 100 * MILLION;
pub(crate) const GET_CURRENT_FEE_PERCENTILES_COST_CYCLES: u64 = 100 * MILLION;
pub(crate) const SEND_TRANSACTION_BASE_COST_CYCLES: u64 = 5 * BILLION;
pub(crate) const SEND_TRANSACTION_COST_CYCLES_PER_BYTE: u64 = 20 * MILLION;
pub(crate) const SIGN_WITH_ECDSA_COST_CYCLES: u64 = 10 * BILLION;

#[async_trait]
pub trait ManagementCanister {
    /// Creates a new instance of the management canister.
    fn new(network: crate::types::Network) -> Self;

    /// Creates a new instance of the management canister using the given ECDSA public key.
    fn new_using_ecdsa_public_key(
        network: crate::types::Network,
        ecdsa_public_key: EcdsaPubKey,
    ) -> Self;

    /// Initializes the management canister by initializing its ECDSA public key.
    fn set_ecdsa_public_key(&mut self, ecdsa_public_key: EcdsaPubKey);

    /// Returns the network the management canister interacts with.
    fn get_network(&self) -> Network;

    /// Returns the ECDSA public key of this canister.
    fn get_ecdsa_public_key(&self) -> EcdsaPubKey;

    /// Returns the UTXOs of the given Bitcoin `address` according to `min_confirmations`.
    async fn get_utxos(
        &self,
        address: &Address,
        min_confirmations: u32,
    ) -> Result<GetUtxosResponse, GetUtxosError>;

    /// Returns fees as percentiles in millisatoshis/byte over the last 10,000 transactions.
    async fn get_current_fees(&self) -> Result<Vec<MillisatoshiPerByte>, ManagementCanisterReject>;

    /// Returns the signature of the given `message_hash` associated with the ECDSA public key of this canister at the given derivation path.
    async fn sign_with_ecdsa(
        &self,
        derivation_path: &[Vec<u8>],
        message_hash: &[u8],
    ) -> Result<Vec<u8>, ManagementCanisterReject>;

    /// Sends the given transaction to the network the management canister interacts with.
    async fn send_transaction(
        &mut self,
        transaction: Vec<u8>,
        network: Network,
    ) -> Result<(), ManagementCanisterReject>;
}
