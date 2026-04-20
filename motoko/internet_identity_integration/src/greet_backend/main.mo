import Principal "mo:core/Principal";

persistent actor {
  public query (message) func greet() : async Text {
    return "Hello, " # message.caller.toText() # "!";
  };
};
