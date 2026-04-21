import Nat "mo:core/Nat";
import Time "mo:core/Time";
import Principal "mo:core/Principal";

persistent actor QueryStats {

  transient let IC = actor "aaaaa-aa" : actor {
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
    Time.now();
  };

  public func get_current_query_stats_as_string() : async Text {
    let stats = await IC.canister_status({
      canister_id = Principal.fromActor(QueryStats);
    });
    "Number of calls: " # stats.query_stats.num_calls_total.toText() # " - Number of instructions: " # stats.query_stats.num_instructions_total.toText() # " - Request payload bytes: " # stats.query_stats.request_payload_bytes_total.toText() # " - Response payload bytes: " # stats.query_stats.response_payload_bytes_total.toText();
  };
};
