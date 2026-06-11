import Types "Types";
import { ic } "mo:ic";
import IC "mo:ic/Types";

module {
  type Cycles = Types.Cycles;
  type SchnorrAux = IC.SchnorrAux;

  // The fee for the `sign_with_schnorr` endpoint using the test key.
  let SIGN_WITH_SCHNORR_COST_CYCLES : Cycles = 10_000_000_000;

  /// Returns the Schnorr public key of this canister at the given derivation path.
  public func schnorr_public_key(key_name : Text, derivation_path : [Blob]) : async Blob {
    // Retrieve the public key of this canister at derivation path
    // from the Schnorr API.
    let res = await ic.schnorr_public_key({
      canister_id = null;
      derivation_path;
      key_id = {
        algorithm = #bip340secp256k1;
        name = key_name;
      };
    });

    res.public_key;
  };

  public func sign_with_schnorr(key_name : Text, derivation_path : [Blob], message : Blob, aux : ?SchnorrAux) : async Blob {
    let res = await (with cycles = SIGN_WITH_SCHNORR_COST_CYCLES) ic.sign_with_schnorr({
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
