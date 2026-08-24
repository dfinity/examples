import Principal "mo:core/Principal";

actor Greeter {
  public query (message) func greet() : async Text {
    "Hello, " # message.caller.toText() # "!"
  };
};
