use bitcoin::{blockdata::constants::genesis_block, Network, OutPoint, TxOut};
use btc::{
    proto::{btc_adapter_client::BtcAdapterClient, GetSuccessorsRequest},
    store::State,
};
use prost::Message;
use std::io;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;
use tonic::Request;

mod proto {
    tonic::include_proto!("btc");
}

const DELTA: u64 = 6;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Initialize the state with the mainnet genesis block.
    let mut state = Arc::new(RwLock::new(State::new(
        DELTA,
        Network::Bitcoin,
        genesis_block(Network::Bitcoin),
    )));

    if args.len() > 1 {
        // A state file was specified in the first argument. Load that state.
        println!("Reading state from disk...");
        let now = SystemTime::now();
        let state_from_disk = std::fs::read(&args[1]).unwrap();
        println!(
            "Done. Duration: {} seconds",
            now.elapsed().unwrap().as_secs()
        );

        println!("Deserializing State...");
        let now = SystemTime::now();
        let decoded_state = btc::proto::State::decode(&*state_from_disk).unwrap();
        state = Arc::new(RwLock::new(State::from_proto(decoded_state)));
        println!(
            "Done. Duration: {} seconds",
            now.elapsed().unwrap().as_secs()
        );
    }

    // A reference to the lock of the state for use in another thread.
    let state_2 = Arc::clone(&state);

    tokio::spawn(async move {
        let mut rpc_client = BtcAdapterClient::connect("http://127.0.0.1:34254")
            .await
            .unwrap();

        loop {
            let block_hashes = {
                let state_read = state_2
                    .read()
                    .expect("Cannot get read-only access to state");

                let mut block_hashes: Vec<Vec<u8>> = state_read
                    .get_unstable_blocks()
                    .iter()
                    .map(|b| b.block_hash().to_vec())
                    .collect();

                block_hashes.push(state_read.anchor_hash().to_vec());

                block_hashes
            };

            // Start requesting more blocks.
            let rpc_request = Request::new(GetSuccessorsRequest { block_hashes });

            // Send the request to the BTC adapter. We assume that the TCP can
            // accept connections in this hard-coded port.
            match rpc_client.get_successors(rpc_request).await {
                Ok(tonic_response) => {
                    let mut state_write = state_2.write().unwrap();
                    let response = tonic_response.into_inner();

                    for block_proto in response.blocks {
                        let block = btc::block::from_proto(&block_proto);
                        println!("Processing block with hash: {}", block.block_hash());
                        state_write.insert_block(block);
                        println!("New mainchain height: {}", state_write.main_chain_height());
                    }
                }
                Err(_) => {}
            }

            // Sleep for a second to not spam the adapter.
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    loop {
        print!(">> ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading from STDIN");

        // Command for getting utxos of an address.
        // e.g. "utxos 12tGGuawKdkw5NeDEzS3UANhCRa1XggBbK"
        if input.contains("utxos") {
            let state_read = state.read().expect("Cannot get read-only access to state");
            let address_str = input.as_str().split(" ").collect::<Vec<&str>>()[1].trim();
            println!(
                "{:#?}",
                state_read
                    .get_utxos(address_str, 0)
                    .into_iter()
                    .map(|x| (x.0, x.1, x.2))
                    .collect::<Vec<(OutPoint, TxOut, u32)>>()
            );
        }
        // Command for getting the balance of an address.
        // e.g. "balance 12tGGuawKdkw5NeDEzS3UANhCRa1XggBbK"
        else if input.contains("balance") {
            let state_read = state.read().expect("Cannot get read-only access to state");
            let address_str = input.as_str().split(" ").collect::<Vec<&str>>()[1].trim();
            let balance = state_read.get_balance(address_str, 0);
            let decimals = balance % 100_000_000;
            let whole = balance / 100_000_000;
            println!("{}.{:0>8}", whole, decimals);
        }
        // Command for saving the state.
        // e.g. "save file_name"
        else if input.contains("save") {
            let file_name = input.as_str().split(" ").collect::<Vec<&str>>()[1].trim();
            let state_read = state.read().expect("Cannot get read-only access to state");

            let mut file = std::fs::File::create(file_name).expect("create failed");
            file.write_all(&state_read.to_proto().encode_to_vec())
                .expect("write failed");
            println!("Save complete");
        }
    }
}
