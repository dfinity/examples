import ExperimentalCycles "mo:base/ExperimentalCycles";
import Blob "mo:base/Blob";

import Types "Types";

module {
  type SchnorrPublicKeyArgs = Types.SchnorrPublicKeyArgs;
  type SchnorrPublicKeyReply = Types.SchnorrPublicKeyReply;
  type SignWithSchnorrArgs = Types.SignWithSchnorrArgs;
  type SignWithSchnorrReply = Types.SignWithSchnorrReply;
  type Cycles = Types.Cycles;
  type SchnorrCanisterActor = Types.SchnorrCanisterActor;

  // The fee for the `sign_with_schnorr` endpoint using the test key.
  let SIGN_WITH_SCHNORR_COST_CYCLES : Cycles = 10_000_000_000;

  /// Returns the Schnorr public key of this canister at the given derivation path.
  public func schnorr_public_key(schnorr_canister_actor : SchnorrCanisterActor, key_name : Text, derivation_path : [Blob]) : async Blob {
    // Retrieve the public key of this canister at derivation path
    // from the Schnorr API.
    let res = await schnorr_canister_actor.schnorr_public_key({
      canister_id = null;
      derivation_path;
      key_id = {
        algorithm = #bip340secp256k1;
        name = key_name;
      };
    });

    res.public_key;
  };

  public func sign_with_schnorr(schnorr_canister_actor : SchnorrCanisterActor, key_name : Text, derivation_path : [Blob], message : Blob, aux : ?Types.SchnorrAux) : async Blob {
    ExperimentalCycles.add<system>(SIGN_WITH_SCHNORR_COST_CYCLES);
    let res = await schnorr_canister_actor.sign_with_schnorr({
      message;
      derivation_path;
      key_id = {
        algorithm = #bip340secp256k1;
        name = key_name;
      };
      aux;
    });

    res.signature;
  };
};
