import AssocList "mo:base/AssocList";
import List "mo:base/List";
import Text "mo:base/Text";

actor {
  public query func greet(name: Text) : async Text {
    return "Hello, " # name # ", from [src/hello/main.mo]";
  };
  public query func test() : async Text {
    return "test from IC main!";
  };
};
