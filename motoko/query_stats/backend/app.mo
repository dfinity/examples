import Nat "mo:core/Nat";
import Principal "mo:core/Principal";
import Time "mo:core/Time";
import { ic } "mo:ic";

persistent actor QueryStats {

  public query func load() : async Int {
    Time.now();
  };

  public func get_current_query_stats_as_string() : async Text {
    let stats = await ic.canister_status({
      canister_id = Principal.fromActor(QueryStats);
    });
    "Number of calls: " # stats.query_stats.num_calls_total.toText() # " - Number of instructions: " # stats.query_stats.num_instructions_total.toText() # " - Request payload bytes: " # stats.query_stats.request_payload_bytes_total.toText() # " - Response payload bytes: " # stats.query_stats.response_payload_bytes_total.toText();
  };
};
