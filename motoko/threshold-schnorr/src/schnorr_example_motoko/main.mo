import Cycles "mo:base/ExperimentalCycles";
import Error "mo:base/Error";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Blob "mo:base/Blob";
import Debug "mo:base/Debug";
import Hex "./utils/Hex";

actor {
  public type SchnorrAlgorithm = { #bip340secp256k1; #ed25519 };

  public type KeyId = {
    algorithm : SchnorrAlgorithm;
    name : Text;
  };

  public type SchnorrAux = {
    #bip341 : {
      merkle_root_hash : Blob;
    };
  };

  // Only the Schnorr methods in the IC management canister is required here.
  type IC = actor {
    schnorr_public_key : ({
      canister_id : ?Principal;
      derivation_path : [Blob];
      key_id : KeyId;
    }) -> async ({ public_key : Blob });
    sign_with_schnorr : ({
      message : Blob;
      derivation_path : [Blob];
      key_id : KeyId;
      aux : ?SchnorrAux;
    }) -> async ({ signature : Blob });
  };

  var ic : IC = actor ("aaaaa-aa");

  public func for_test_only_change_management_canister_id(mock_id : Text) {
    ic := actor (mock_id);
  };

  public shared ({ caller }) func public_key(algorithm : SchnorrAlgorithm) : async {
    #Ok : { public_key_hex : Text };
    #Err : Text;
  } {
    try {
      let { public_key } = await ic.schnorr_public_key({
        canister_id = null;
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm; name = "insecure_test_key_1" };
      });
      #Ok({ public_key_hex = Hex.encode(Blob.toArray(public_key)) });
    } catch (err) {
      #Err(Error.message(err));
    };
  };

  public shared ({ caller }) func sign(message_arg : Text, algorithm : SchnorrAlgorithm, bip341Tweak : ?Text) : async {
    #Ok : { signature_hex : Text };
    #Err : Text;
  } {
    let aux = switch (bip341Tweak) {
      case (?tweak) {
        ?#bip341({
          merkle_root_hash = Blob.fromArray(
            switch (Hex.decode(tweak)) {
              case (#ok bytes) bytes;
              case (#err _) Debug.trap("failed to decode tweak");
            }
          );
        });
      };
      case (null) null;
    };
    try {
      Cycles.add<system>(25_000_000_000);
      let signArgs = {
        message = Text.encodeUtf8(message_arg);
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm; name = "insecure_test_key_1" };
        aux;
      };
      Debug.print("signArgs: " # debug_show (signArgs));
      let { signature } = await ic.sign_with_schnorr(signArgs);
      #Ok({ signature_hex = Hex.encode(Blob.toArray(signature)) });
    } catch (err) {
      #Err(Error.message(err));
    };
  };
};
