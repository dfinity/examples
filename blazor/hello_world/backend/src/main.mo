persistent actor {

  stable var greeting : Text = "Hello from Motoko!";

  public query func getGreeting() : async Text {
    return greeting;
  };

  public func setGreeting(name : Text) : async Text {
    greeting := "Hello, " # name # "! Welcome to ICP + Blazor!";
    return greeting;
  };

  public query func hello(name : Text) : async Text {
    return "Hello, " # name # "!";
  };

};
