use candid::{Nat, Principal};
use evm_rpc_canister_types::{
    Block, EthMainnetService, EvmRpcCanister, GetBlockByNumberResult,
    MultiGetBlockByNumberResult, RpcServices, CANISTER_ID,
};
use ic_cdk::api::management_canister::{
    ecdsa::{
        ecdsa_public_key, sign_with_ecdsa, EcdsaCurve, EcdsaKeyId, EcdsaPublicKeyArgument,
        SignWithEcdsaArgument,
    },
    schnorr::{
        schnorr_public_key, sign_with_schnorr, SchnorrAlgorithm, SchnorrKeyId,
        SchnorrPublicKeyArgument, SignWithSchnorrArgument,
    },
};

const KEY_NAME: &str = "test_key_1"; // Use "key_1" for production

/// Resolve the EVM RPC canister ID at runtime. icp-cli sets the
/// `PUBLIC_CANISTER_ID:evm_rpc` env var; fall back to the well-known mainnet ID.
fn evm_rpc() -> EvmRpcCanister {
    let name = "PUBLIC_CANISTER_ID:evm_rpc";
    let id = if ic0::env_var_name_exists(name) != 0 {
        let mut buf = vec![0u8; ic0::env_var_value_size(name)];
        ic0::env_var_value_copy(name, &mut buf, 0);
        let val = String::from_utf8(buf).expect("env var is not valid UTF-8");
        Principal::from_text(&val).expect("Invalid evm_rpc canister ID")
    } else {
        CANISTER_ID
    };
    EvmRpcCanister(id)
}

#[ic_cdk::update]
async fn get_evm_block(height: u128) -> Block {
    // Ethereum Mainnet RPC providers
    let rpc_providers = RpcServices::EthMainnet(Some(vec![
        EthMainnetService::Llama,
        // EthMainnetService::Alchemy,
        // EthMainnetService::Cloudflare,
    ]));

    // Base Mainnet RPC providers
    // Get chain ID and RPC providers from https://chainlist.org/
    // let rpc_providers = RpcServices::Custom {
    //     chainId: 8453,
    //     services: vec![
    //         evm_rpc_canister_types::RpcApi {
    //             url: "https://base.llamarpc.com".to_string(),
    //             headers: None,
    //         },
    //         evm_rpc_canister_types::RpcApi {
    //             url: "https://base-rpc.publicnode.com".to_string(),
    //             headers: None,
    //         },
    //     ],
    // };

    // Call `eth_get_block_by_number` RPC method (unused cycles will be refunded)
    let cycles = 10_000_000_000;
    let (result,) = evm_rpc()
        .eth_get_block_by_number(
            rpc_providers,
            None,
            evm_rpc_canister_types::BlockTag::Number(Nat::from(height)),
            cycles,
        )
        .await
        .expect("Failed to call RPC canister");

    match result {
        MultiGetBlockByNumberResult::Consistent(r) => match r {
            GetBlockByNumberResult::Ok(block) => block,
            GetBlockByNumberResult::Err(err) => panic!("{err:?}"),
        },
        MultiGetBlockByNumberResult::Inconsistent(v) => {
            panic!("RPC providers gave inconsistent results: {:?}", v)
        }
    }
}

fn sha256(input: &String) -> [u8; 32] {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(input.as_bytes());
    hasher.finalize().into()
}

#[ic_cdk::update]
async fn get_ecdsa_public_key() -> String {
    let (pub_key,) = ecdsa_public_key(EcdsaPublicKeyArgument {
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KEY_NAME.to_string(),
        },
        ..Default::default()
    })
    .await
    .expect("Failed to get ecdsa public key");
    hex::encode(pub_key.public_key)
}

#[ic_cdk::update]
async fn get_schnorr_public_key() -> String {
    let (pub_key,) = schnorr_public_key(SchnorrPublicKeyArgument {
        key_id: SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Ed25519,
            name: KEY_NAME.to_string(),
        },
        ..Default::default()
    })
    .await
    .expect("Failed to get schnorr public key");
    hex::encode(pub_key.public_key)
}

#[ic_cdk::update]
async fn sign_message_with_ecdsa(message: String) -> String {
    let message_hash = sha256(&message);
    let (signature,) = sign_with_ecdsa(SignWithEcdsaArgument {
        message_hash: message_hash.to_vec(),
        key_id: EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: KEY_NAME.to_string(),
        },
        ..Default::default()
    })
    .await
    .expect("Failed to sign");
    hex::encode(signature.signature)
}

#[ic_cdk::update]
async fn sign_message_with_schnorr(message: String) -> String {
    let message = sha256(&message);
    let (signature,) = sign_with_schnorr(SignWithSchnorrArgument {
        message: message.to_vec(),
        key_id: SchnorrKeyId {
            algorithm: SchnorrAlgorithm::Ed25519,
            name: KEY_NAME.to_string(),
        },
        ..Default::default()
    })
    .await
    .expect("Failed to sign");
    hex::encode(signature.signature)
}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
