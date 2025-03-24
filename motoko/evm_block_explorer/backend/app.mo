import EvmRpc "canister:evm_rpc";
import IC "ic:aaaaa-aa";
import Sha256 "mo:sha2/Sha256";
import Base16 "mo:base16/Base16";
import Debug "mo:base/Debug";
import Blob "mo:base/Blob";
import Text "mo:base/Text";
import Cycles "mo:base/ExperimentalCycles";

persistent actor EvmBlockExplorer {
  transient let key_name = "test_key_1"; // Use "key_1" for production and "dfx_test_key" locally

  public func get_evm_block(height : Nat) : async EvmRpc.Block {
    // Ethereum Mainnet RPC providers
    // Read more here: https://internetcomputer.org/docs/current/developer-docs/multi-chain/ethereum/evm-rpc/overview#supported-json-rpc-providers
    let services : EvmRpc.RpcServices = #EthMainnet(
      ?[
        #Llama,
        // #Alchemy,
        // #Cloudflare
      ]
    );

    // Base Mainnet RPC providers
    // Get chain ID and RPC providers from https://chainlist.org/
    // let services : EvmRpc.RpcServices = #Custom {
    //   chainId = 8453;
    //   services = [
    //     {url = "https://base.llamarpc.com"; headers = null},
    //     {url = "https://base-rpc.publicnode.com"; headers = null}
    //   ];
    // };

    // Call `eth_getBlockByNumber` RPC method (unused cycles will be refunded)
    Cycles.add<system>(10_000_000_000);
    let result = await EvmRpc.eth_getBlockByNumber(services, null, #Number(height));

    switch result {
      // Consistent, successful results.
      case (#Consistent(#Ok block)) {
        block;
      };
      // All RPC providers return the same error.
      case (#Consistent(#Err error)) {
        Debug.trap("Error: " # debug_show error);
      };
      // Inconsistent results between RPC providers. Should not happen if a single RPC provider is used.
      case (#Inconsistent(results)) {
        Debug.trap("Inconsistent results" # debug_show results);
      };
    };
  };

  public func get_ecdsa_public_key() : async Text {
    let { public_key } = await IC.ecdsa_public_key({
      canister_id = null;
      derivation_path = [];
      key_id = { curve = #secp256k1; name = key_name };
    });
    Base16.encode(public_key);
  };

  public func sign_message_with_ecdsa(message : Text) : async Text {
    let message_hash : Blob = Sha256.fromBlob(#sha256, Text.encodeUtf8(message));
    Cycles.add<system>(25_000_000_000);
    let { signature } = await IC.sign_with_ecdsa({
      message_hash;
      derivation_path = [];
      key_id = { curve = #secp256k1; name = key_name };
    });
    Base16.encode(signature);
  };

  public func get_schnorr_public_key() : async Text {
    let { public_key } = await IC.schnorr_public_key({
      canister_id = null;
      derivation_path = [];
      key_id = { algorithm = #ed25519; name = key_name };
    });
    Base16.encode(public_key);
  };

  public func sign_message_with_schnorr(message : Text) : async Text {
    Cycles.add<system>(25_000_000_000);
    let { signature } = await IC.sign_with_schnorr({
      message = Text.encodeUtf8(message);
      derivation_path = [];
      key_id = { algorithm = #ed25519; name = key_name };
      aux = null;
    });
    Base16.encode(signature);
  };
};
