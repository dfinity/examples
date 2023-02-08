import ExperimentalCycles "mo:base/ExperimentalCycles";

import Types "Types";

module {
  type ECDSAPublicKey = Types.ECDSAPublicKey;
  type ECDSAPublicKeyReply = Types.ECDSAPublicKeyReply;
  type SignWithECDSA = Types.SignWithECDSA;
  type SignWithECDSAReply = Types.SignWithECDSAReply;

  /// Actor definition to handle interactions with the ECDSA canister.
  type EcdsaCanisterActor = actor {
      ecdsa_public_key : ECDSAPublicKey -> async ECDSAPublicKeyReply;
      sign_with_ecdsa : SignWithECDSA -> async SignWithECDSAReply;
  };

  let ecdsa_canister_actor : EcdsaCanisterActor = actor("aaaaa-aa");

  /// Returns the ECDSA public key of this canister at the given derivation path.
  public func ecdsa_public_key(key_name : Text, derivation_path : [Blob]) : async Blob {
    // Retrieve the public key of this canister at derivation path
    // from the ECDSA API.
    let res = await ecdsa_canister_actor.ecdsa_public_key({
        canister_id = null;
        derivation_path;
        key_id = {
            curve = #secp256k1;
            name = key_name;
        };
    });
        
    res.public_key
  };

  public func sign_with_ecdsa(key_name : Text, derivation_path : [Blob], message_hash : Blob) : async Blob {
    ExperimentalCycles.add(10_000_000_000);
    let res = await ecdsa_canister_actor.sign_with_ecdsa({
        message_hash;
        derivation_path;
        key_id = {
            curve = #secp256k1;
            name = key_name;
        };
    });
        
    res.signature
  };
}
