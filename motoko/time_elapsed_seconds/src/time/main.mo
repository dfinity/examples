import Nat64 = "mo:base/Nat64";
import Time = "mo:base/Time";
actor {
    var lastTime = Time.time();
    public func greet(name : Text) : async Text {
        let now = Time.time();
        let elapsedSeconds = (now - lastTime)/(1000_000_000: Nat64);
        lastTime := now;
        return "Hello, " # name # "!" #
          " I was last called " # Nat64.toText(elapsedSeconds) # " seconds ago.";
    };
};
