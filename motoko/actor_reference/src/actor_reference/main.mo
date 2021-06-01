import Nat "mo:base/Nat";
import Cycles "mo:base/ExperimentalCycles";
import Debug "mo:base/Debug";


actor {

  // Use an actor reference to access the well-known, virtual
  // IC management canister with specified Principal "aaaaa-aa",
  // asserting its interface type
  // NB: this is a smaller supertype of the full interface at
  //     https://sdk.dfinity.org/docs/interface-spec/index.html#ic-management-canister
  let IC =
    actor "aaaaa-aa" : actor {

      create_canister : {
          // richer in ic.did
        } -> async { canister_id : Principal };

      canister_status : { canister_id : Principal } ->
        async { // richer in ic.did
          cycles : Nat
        };

      stop_canister : { canister_id : Principal } -> async ();

      delete_canister : { canister_id : Principal } -> async ();
    };

  // Burn half of this actor's cycle balance by provisioning,
  // creating, stopping and deleting a fresh canister
  // (without ever installing any code)
  public func burn() : async () {
    Debug.print("balance before: " # Nat.toText(Cycles.balance()));
    Cycles.add(Cycles.balance()/2);
    let cid = await IC.create_canister({});
    let status = await IC.canister_status(cid);
    Debug.print("cycles: " # Nat.toText(status.cycles));
    await IC.stop_canister(cid);
    await IC.delete_canister(cid);
    Debug.print("balance after: " # Nat.toText(Cycles.balance()));
  };

};
