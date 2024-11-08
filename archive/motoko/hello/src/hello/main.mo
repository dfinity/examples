import Debug "mo:base/Debug";

actor {
  public func greet(name : Text) : async Text {
    Debug.print("Received " # name);
    return "Hello, " # name # "!";
  };
};
