persistent actor HelloWorld {
  // We store the greeting in a stable variable such that it gets persisted over canister upgrades.
  var greeting : Text = "Hello, ";

  // This update method stores the greeting prefix in stable memory.
  public func setGreeting(prefix : Text) : async () {
    greeting := prefix;
  };

  // This query method returns the currently persisted greeting with the given name.
  public query func greet(name : Text) : async Text {
    return greeting # name # "!";
  };
};
