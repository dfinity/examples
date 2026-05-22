import Principal "mo:core/Principal";

persistent actor Whoami {
  public query (message) func whoami() : async Principal {
    message.caller;
  };
};
