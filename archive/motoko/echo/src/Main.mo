actor Echo {

  // Say the given phase.
  public query func say(phrase : Text) : async Text {
    return phrase;
  };
};
