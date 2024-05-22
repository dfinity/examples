import Debug "mo:base/Debug";

actor CanisterLogs {

  public func print(text : Text) : async () {
    Debug.print(text);
  };

  public func trap(text : Text) : async () {
    Debug.print("right before trap");
    Debug.trap(text);
  };

};
