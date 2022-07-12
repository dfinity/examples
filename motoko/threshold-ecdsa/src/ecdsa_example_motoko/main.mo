import Cycles "mo:base/ExperimentalCycles";
import Error "mo:base/Error";
import Principal "mo:base/Principal";
import Time "mo:base/Time";

actor {
  // Only the ecdsa methods in the IC management canister is required here.
  type IC = actor {
    ecdsa_public_key : ({
      canister_id : ?Principal;
      derivation_path : [Blob];
      key_id : { curve: { #secp256k1; } ; name: Text };
    }) -> async ({ public_key : Blob; chain_code : Blob; });
    sign_with_ecdsa : ({
      message_hash : Blob;
      derivation_path : [Blob];
      key_id : { curve: { #secp256k1; } ; name: Text };
    }) -> async ({ signature : Blob });
  };

  let ic : IC = actor("aaaaa-aa");

  public shared (msg) func public_key() : async { #Ok : { public_key: Blob }; #Err : Text } {
    let caller = Principal.toBlob(msg.caller);
    try {
      let { public_key } = await ic.ecdsa_public_key({
          canister_id = null;
          derivation_path = [ caller ];
          key_id = { curve = #secp256k1; name = "somekey" };
      });
      #Ok({ public_key })
    } catch (err) {
      #Err(Error.message(err))
    }
  };

  public shared (msg) func sign(message: Blob) : async { signature: Blob; latency: Time.Time } {
    let caller = Principal.toBlob(msg.caller);
      Cycles.add(10_000_000_000);
      let start = Time.now();
      let result = await ic.sign_with_ecdsa({
          message_hash = message;
          derivation_path = [ caller ];
          key_id = { curve = #secp256k1; name = "somekey" };
      });
      let latency = Time.now() - start;
      { signature = result.signature; latency }
  };
}

