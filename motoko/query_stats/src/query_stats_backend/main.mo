import Nat "mo:base/Nat";
import Time "mo:base/Time";
import Principal "mo:base/Principal";

actor QueryStats {

  let IC = actor "aaaaa-aa" : actor {
    canister_status : { canister_id : Principal } -> async {
      query_stats : {
        num_calls_total : Nat;
        num_instructions_total : Nat;
        request_payload_bytes_total : Nat;
        response_payload_bytes_total : Nat;
      };
    };
  };

  public query func load() : async Int {
    return Time.now();
  };

  public func get_current_query_stats_as_string() : async Text {
    let stats = await IC.canister_status({
      canister_id = Principal.fromActor(QueryStats);
    });
    return "Number of calls: " # Nat.toText(stats.query_stats.num_calls_total) # " - Number of instructions: " # Nat.toText(stats.query_stats.num_instructions_total) # " - Request payload bytes: " # Nat.toText(stats.query_stats.request_payload_bytes_total) # " - Response payload bytes: " # Nat.toText(stats.query_stats.response_payload_bytes_total);
  };
};
