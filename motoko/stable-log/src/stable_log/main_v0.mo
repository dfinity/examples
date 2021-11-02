import Seq "mo:sequence/Sequence";
import Types "Types";
import Common "Common";

actor {    
  stable var ourSeq : Types.Entries = #empty;

  let append = Seq.defaultAppend();

  public func greet(name : Text) : async Text {
    ourSeq := append(ourSeq, Seq.make(Common.entry(name)));
    return "Hello, " # name # "!";
  };
};
