import Seq "mo:sequence/Sequence";
import Common "Common";

actor {    
  stable var ourSeq : Common.Entries = #empty;

  let append = Seq.defaultAppend();

  public func greet(name : Text) : async Text {
    ourSeq := append(ourSeq, Seq.make(Common.entry(name)));
    return "Hello, " # name # "!";
  };
};
