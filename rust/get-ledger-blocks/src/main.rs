use std::error::Error;
use ic_agent::Agent;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::ic_types::Principal;
use prost::Message;
use structopt::StructOpt;
use url::Url;
use ledger::GetBlockRequest;
use crate::ledger::get_blocks_response::GetBlocksContent;
use crate::ledger::{Block, GetBlocksResponse};

pub mod ledger {
    include!("gen/ledger.rs");
}

#[derive(Debug, StructOpt)]
#[structopt(name = "get-ledger-blocks", about = "An example of how to get blocks from the ledger")]
struct Opt {
    #[structopt(short, long)]
    url: Url,

    #[structopt(short, long)]
    ledger_id: String,

    #[structopt(short, long)]
    start: u64,

    #[structopt(short = "L", long)]
    length: u64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let transport = ReqwestHttpReplicaV2Transport::create(opt.url).unwrap();
    let agent = Agent::builder()
        .with_transport(transport)
        .build()
        .unwrap();
    let ledger_principal_id = Principal::from_text(opt.ledger_id).unwrap();

    // create the request and encode it as protobuf message
    let req = GetBlockRequest {
        start: opt.start,
        length: opt.length
    }.encode_to_vec();

    // call get_blocks_pb on the Ledger with the encoded request as argument
    let res = agent.query(&ledger_principal_id, "get_blocks_pb")
        .with_arg(req)
        .call()
        .await?;

    // decode the response as protobuf message
    let res = GetBlocksResponse::decode(res.as_slice()).unwrap().get_blocks_content.unwrap();

    // check if the response is an error or the encoded blocks
    let encoded_blocks = match res {
        GetBlocksContent::Blocks(blocks) => blocks.blocks,
        GetBlocksContent::Error(e) => panic!("Error fetching blocks {}", e),
    };

    // decode each encoded block into a block
    let blocks: Vec<Block> = encoded_blocks.iter().map(|b| Block::decode(b.block.as_slice()).unwrap()).collect();

    for block in blocks {
        println!("{:?}", block);
    }
    Ok(())
}
