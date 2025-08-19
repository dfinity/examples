import Principal "mo:base/Principal";

persistent actor {
  public query (message) func greet() : async Text {
    return "Hello, " # Principal.toText(message.caller) # "!";
  };
};
