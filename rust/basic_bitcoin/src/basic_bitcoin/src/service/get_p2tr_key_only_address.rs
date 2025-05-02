use ic_cdk::update;

use crate::{p2tr_key_only, BTC_CONTEXT, P2TR_KEY_ONLY_DERIVATION_PATH_PREFIX};

/// Returns the P2TR address of this canister at a specific derivation path.
#[update]
pub async fn get_p2tr_key_only_address() -> String {
    let ctx = BTC_CONTEXT.with(|ctx| ctx.get());
    let derivation_path = vec![P2TR_KEY_ONLY_DERIVATION_PATH_PREFIX.as_bytes().to_vec()];

    p2tr_key_only::get_address(&ctx, derivation_path)
        .await
        .to_string()
}
