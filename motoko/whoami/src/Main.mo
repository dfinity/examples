actor WhoAmI {

  // Return the principal identifier of the caller.
  public shared (message) func whoami() : async Principal {
    return message.caller;
  };

  // Return the principal identifier of the canister.
  public func whoareyou() : async Principal {
    let canisterId = await WhoAmI.whoami();
    return canisterId;
  };
};
