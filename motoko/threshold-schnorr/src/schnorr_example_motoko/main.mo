import Cycles "mo:base/ExperimentalCycles";
import Error "mo:base/Error";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Blob "mo:base/Blob";
import Hex "./utils/Hex";

actor {
  public type SchnorrAlgotirhm = { #bip340secp256k1; #ed25519 };

  public type KeyId = {
    algorithm: SchnorrAlgotirhm;
    name: Text;
  };

  // Only the Schnorr methods in the IC management canister is required here.
  type IC = actor {
    schnorr_public_key : ({
      canister_id : ?Principal;
      derivation_path : [Blob];
      key_id : KeyId;
    }) -> async ({ public_key : Blob; });
    sign_with_schnorr : ({
      message : Text;
      derivation_path : [Blob];
      key_id : KeyId;
    }) -> async ({ signature : Blob });
  };

  let ic : IC = actor("aaaaa-aa");

  public func public_key(algorithm_arg : SchnorrAlgotirhm) : async { #Ok : { public_key_hex: Text }; #Err : Text } {
    try {
      let { public_key } = await ic.schnorr_public_key({
          canister_id = null;
          derivation_path = [];
          key_id = { algorithm = algorithm_arg; name = "dfx_test_key" };
      });
      #Ok({ public_key_hex = Hex.encode(Blob.toArray(public_key)) })
    } catch (err) {
      #Err(Error.message(err))
    }
  };

  public func sign(message_arg: Text, algorithm_arg : SchnorrAlgotirhm) : async { #Ok : { signature_hex: Text };  #Err : Text } {
    try {
      Cycles.add(25_000_000_000);
      let { signature } = await ic.sign_with_schnorr({
          message = message_arg;
          derivation_path = [];
          key_id = { algorithm = algorithm_arg; name = "dfx_test_key" };
      });
      #Ok({ signature_hex = Hex.encode(Blob.toArray(signature))})
    } catch (err) {
      #Err(Error.message(err))
    }
  };
}

