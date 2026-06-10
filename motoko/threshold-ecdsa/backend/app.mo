import Error "mo:core/Error";
import Principal "mo:core/Principal";
import Text "mo:core/Text";
import Blob "mo:core/Blob";
import Hex "./Hex";
import SHA256 "./SHA256";
import { ic } "mo:ic";

persistent actor ThresholdEcdsa {
  transient let key_id : Text = "test_key_1"; // Use "key_1" for mainnet production

  public shared (msg) func public_key() : async {
    #Ok : { public_key_hex : Text };
    #Err : Text;
  } {
    let caller = msg.caller.toBlob();
    try {
      let { public_key } = await ic.ecdsa_public_key({
        canister_id = null;
        derivation_path = [caller];
        key_id = { curve = #secp256k1; name = key_id };
      });
      #Ok({ public_key_hex = Hex.encode(public_key.toArray()) });
    } catch (err) {
      #Err(err.message());
    };
  };

  public shared (msg) func sign(message : Text) : async {
    #Ok : { signature_hex : Text };
    #Err : Text;
  } {
    let caller = msg.caller.toBlob();
    try {
      let message_hash : Blob = Blob.fromArray(SHA256.sha256(message.encodeUtf8().toArray()));
      let { signature } = await (with cycles = 30_000_000_000) ic.sign_with_ecdsa({
        message_hash;
        derivation_path = [caller];
        key_id = { curve = #secp256k1; name = key_id };
      });
      #Ok({ signature_hex = Hex.encode(signature.toArray()) });
    } catch (err) {
      #Err(err.message());
    };
  };
};
