import Cycles "mo:base/ExperimentalCycles";
import Error "mo:base/Error";
import Principal "mo:base/Principal";
import Text "mo:base/Text";
import Blob "mo:base/Blob";
import Option "mo:base/Option";
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

  public shared ({ caller }) func public_key(algorithm : SchnorrAlgorithm) : async {
    #Ok : { public_key_hex : Text };
    #Err : Text;
  } {
    try {
      let { public_key } = await ic.schnorr_public_key({
        canister_id = null;
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm; name = "dfx_test_key" };
      });
      #Ok({ public_key_hex = Hex.encode(Blob.toArray(public_key)) });
    } catch (err) {
      #Err(Error.message(err));
    };
  };

  public shared ({ caller }) func sign(message_arg : Text, algorithm : SchnorrAlgorithm, bip341TweakHex : ?Text) : async {
    #ok : { signature_hex : Text };
    #err : Text;
  } {
    let aux = switch (Option.map(bip341TweakHex, tryHexToTweak)) {
      case (null) null;
      case (?#ok aux) ?aux;
      case (?#err err) return #err err;
    };

    try {
      Cycles.add<system>(30_000_000_000);
      let signArgs = {
        message = Text.encodeUtf8(message_arg);
        derivation_path = [Principal.toBlob(caller)];
        key_id = { algorithm; name = "dfx_test_key" };
        aux;
      };
      let { signature } = await ic.sign_with_schnorr(signArgs);
      #ok({ signature_hex = Hex.encode(Blob.toArray(signature)) });
    } catch (err) {
      #err(Error.message(err));
    };
  };

  func tryHexToTweak(hex : Text) : { #ok : SchnorrAux; #err : Text } {
    switch (Hex.decode(hex)) {
      case (#ok bytes) {
        #ok(
          #bip341({
            merkle_root_hash = Blob.fromArray(
              bytes
            );
          })
        );
      };
      case (#err _) {
        #err "failed to decode tweak hex";
      };
    };
  };
};
