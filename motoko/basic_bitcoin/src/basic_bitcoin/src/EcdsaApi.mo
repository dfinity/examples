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

  let KEY_NAME : Text = "test";

  // TODO: Replace this principal with the management canister when it's available.
  // For now, call a canister that provides a mock implementation.
  let ecdsa_canister_actor : EcdsaCanisterActor = actor("rrkah-fqaaa-aaaaa-aaaaq-cai");

  /// Returns the ECDSA public key of this canister at the given derivation path.
  public func ecdsa_public_key(derivation_path : [[Nat8]]) : async [Nat8] {
    // Retrieve the public key of this canister at derivation path
    // from the ECDSA API.
    let res = await ecdsa_canister_actor.ecdsa_public_key({
        canister_id = null;
        derivation_path;
        key_id = {
            curve = #Secp256k1;
            name = KEY_NAME;
        };
    });
        
    res.public_key
  };

  public func sign_with_ecdsa(derivation_path : [[Nat8]], message_hash : [Nat8]) : async [Nat8] {
    let res = await ecdsa_canister_actor.sign_with_ecdsa({
        message_hash;
        derivation_path;
        key_id = {
            curve = #Secp256k1;
            name = KEY_NAME;
        };
    });
        
    res.signature
  };
}