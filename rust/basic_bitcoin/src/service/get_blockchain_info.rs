use crate::BTC_CONTEXT;
use ic_cdk::update;
use ic_cdk_bitcoin_canister::BlockchainInfo;

/// Returns information about the state of the Bitcoin blockchain
/// including the tip height, block hash, timestamp, difficulty, and UTXO count.
#[update]
pub async fn get_blockchain_info() -> BlockchainInfo {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());

    ic_cdk_bitcoin_canister::get_blockchain_info(ctx.network)
        .await
        .unwrap()
}
