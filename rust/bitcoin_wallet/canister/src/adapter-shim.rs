//! A shim to facilitate communication between the canister and the adapter.
use btc::proto::{
    btc_adapter_client::BtcAdapterClient, GetSuccessorsRequest, SendTransactionRequest,
};
use candid::{Decode, Encode};
use ic_agent::{export::Principal, Agent};
use prost::Message;
use std::env;
use tonic::Request;

mod proto {
    tonic::include_proto!("btc");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("BTC Canister ID not specified");
    }

    let btc_canister_id =
        Principal::from_text(args[1].clone()).expect(&format!("Invalid canister ID '{}'", args[1]));

    // An agent to connect to the local replica.
    let agent = Agent::builder()
        .with_url("http://127.0.0.1:8000")
        .build()
        .unwrap();
    agent
        .fetch_root_key()
        .await
        .expect("Cannot connect to local replica. Are you sure it's running?");

    let mut rpc_client = BtcAdapterClient::connect("http://127.0.0.1:34254")
        .await
        .unwrap();

    let mut current_height = 1;
    let mut first: bool = true;

    loop {
        // Look up a `get_successors` request from the canister.
        let raw_request = {
            let req = agent
                .query(&btc_canister_id, "get_successors_request")
                .with_arg(&Encode!().unwrap())
                .call()
                .await
                .unwrap();
            Decode!(&req, Vec<u8>).unwrap()
        };

        // Send the request to the adapter.
        let rpc_request = Request::new(
            GetSuccessorsRequest::decode(raw_request.as_slice())
                .map_err(|err| err.to_string())
                .unwrap(),
        );

        match rpc_client.get_successors(rpc_request).await {
            Ok(response) => {
                // Read the response.
                let response_vec = response.into_inner().encode_to_vec();

                // Send response to canister.
                let result = agent
                    .update(&btc_canister_id, "get_successors_response")
                    .with_arg(&Encode!(&response_vec).unwrap())
                    .call_and_wait(delay())
                    .await
                    .unwrap();

                let new_height = Decode!(&result, u32).unwrap();
                if current_height == new_height {
                    if first {
                        first = false;
                        println!("No new block received. Tip height: {}", current_height);
                    }

                    // Sleep for a second to not spam the adapter.
                    std::thread::sleep(std::time::Duration::from_secs(1));
                } else {
                    first = true;
                    println!("Processed new blocks. New height: {:?}", new_height);
                    current_height = new_height;
                }
            }
            Err(err) => {
                println!("Error communicating with adapter: {:?}", err);
            }
        }

        // Are there any outgoing transactions to send?
        let has_outgoing_transaction = {
            let req = agent
                .query(&btc_canister_id, "has_outgoing_transaction")
                .with_arg(&Encode!().unwrap())
                .call()
                .await
                .unwrap();
            Decode!(&req, bool).unwrap()
        };

        if !has_outgoing_transaction {
            continue;
        }

        // Look up outgoing transactions from the btc canister.
        let maybe_raw_tx = agent
            .update(&btc_canister_id, "get_outgoing_transaction")
            .with_arg(&Encode!().unwrap())
            .call_and_wait(delay())
            .await
            .map(|res| Decode!(&res, Option<Vec<u8>>).unwrap())
            .unwrap();

        if let Some(raw_tx) = maybe_raw_tx {
            println!("Sending tx to the adapter...");

            let rpc_request = Request::new(SendTransactionRequest { raw_tx });
            match rpc_client.send_transaction(rpc_request).await {
                Ok(_) => {
                    println!("Done.");
                }
                Err(err) => {
                    println!("Error sending transaction to adapter: {:?}", err);
                }
            }
        }

        // Sleep for a second to not spam the adapter.
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn delay() -> garcon::Delay {
    garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build()
}
