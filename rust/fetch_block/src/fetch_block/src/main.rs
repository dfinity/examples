use candid::candid_method;
use ic_cdk_macros::update;
use ic_ledger_types::{
    query_archived_blocks, query_blocks, Block, BlockIndex, GetBlocksArgs,
    MAINNET_LEDGER_CANISTER_ID,
};

#[update]
#[candid_method]
async fn fetch_block(block_index: BlockIndex) -> Option<Block> {
    let args = GetBlocksArgs {
        start: block_index,
        length: 1,
    };

    let blocks_result = query_blocks(MAINNET_LEDGER_CANISTER_ID, args.clone())
        .await
        .expect("failed to query ledger");

    if blocks_result.blocks.len() >= 1 {
        return blocks_result.blocks.into_iter().next();
    }

    if let Some(func) = blocks_result.archived_blocks.into_iter().find_map(|b| {
        (b.start <= block_index && (block_index - b.start) < b.length).then(|| b.callback)
    }) {
        match query_archived_blocks(&func, args)
            .await
            .expect("failed to query archive")
        {
            Ok(range) => return range.blocks.into_iter().next(),
            _ => (),
        }
    }
    None
}

fn main() {}
