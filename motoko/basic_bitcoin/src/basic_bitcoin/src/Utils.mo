import Result "mo:base/Result";
import Debug "mo:base/Debug";

module {
    type Result<Ok, Err> = Result.Result<Ok, Err>;

    /// Returns the value of the result and traps if there isn't any value to return.
    public func get_ok<T, U>(result : Result<T, U>) : T {
        switch result {
            case (#ok value)
                value;
            case (#err error)
                Debug.trap("pattern failed");
        }
    };

    /// Returns the value of the result and traps with a custom message if there isn't any value to return.
    public func get_ok_except<T, U>(result : Result<T, U>, expect : Text) : T {
        switch result {
            case (#ok value)
                value;
            case (#err error) {
                Debug.print("pattern failed");
                Debug.trap(expect);
            };
        }
    };

    /// Unwraps the value of the option.
    public func unwrap<T>(option : ?T) : T {
        switch option {
            case (?value)
                value;
            case null
                Debug.trap("Prelude.unreachable()");
        }
    };
}