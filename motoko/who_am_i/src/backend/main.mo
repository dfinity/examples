import Principal "mo:core/Principal";

actor Whoami {
  public query (message) func whoami() : async Principal {
    message.caller;
  };
};
