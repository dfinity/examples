actor WhoAmI {

  // Return the principal identifier of this caller.
  public shared ({caller}) func whoami() : async Principal {
    return caller;
  };

  // Return the principal identifier of this canister.
  public func whoareyou() : async Principal {
    let canisterId = await WhoAmI.whoami();
    return canisterId;
  };
};
