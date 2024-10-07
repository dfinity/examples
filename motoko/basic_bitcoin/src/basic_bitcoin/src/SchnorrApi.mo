import ExperimentalCycles "mo:base/ExperimentalCycles";

import Types "Types";

module {
  type SchnorrPublicKeyArgs = Types.SchnorrPublicKeyArgs;
  type SchnorrPublicKeyReply = Types.SchnorrPublicKeyReply;
  type SignWithSchnorrArgs = Types.SignWithSchnorrArgs;
  type SignWithSchnorrReply = Types.SignWithSchnorrReply;
  type Cycles = Types.Cycles;

  /// Actor definition to handle interactions with the Schnorr canister.
  type SchnorrCanisterActor = actor {
    schnorr_public_key : SchnorrPublicKeyArgs -> async SchnorrPublicKeyReply;
    sign_with_schnorr : SignWithSchnorrArgs -> async SignWithSchnorrReply;
  };

  // The fee for the `sign_with_schnorr` endpoint using the test key.
  let SIGN_WITH_SCHNORR_COST_CYCLES : Cycles = 10_000_000_000;

  let schnorr_canister_actor : SchnorrCanisterActor = actor ("aaaaa-aa");

  /// Returns the Schnorr public key of this canister at the given derivation path.
  public func schnorr_public_key(key_name : Text, derivation_path : [Blob]) : async Blob {
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

  public func sign_with_schnorr(key_name : Text, derivation_path : [Blob], message : Blob) : async Blob {
    ExperimentalCycles.add<system>(SIGN_WITH_SCHNORR_COST_CYCLES);
    let res = await schnorr_canister_actor.sign_with_schnorr({
      message;
      derivation_path;
      key_id = {
        algorithm = #bip340secp256k1;
        name = key_name;
      };
    });

    res.signature;
  };
};
