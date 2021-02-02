import AssocList "mo:base/AssocList";
import List "mo:base/List";
import Text "mo:base/Text";

actor {

  type Name = Text;
  type Phone = Text;

  type Entry = {
    desc: Text;
    phone: Phone;
  };

  type PhoneBookMap = AssocList.AssocList<Name, Entry>;

  var phonebook: PhoneBookMap = List.nil<(Name, Entry)>();

  public func insert(name : Name, entry : Entry): async () {
    phonebook := AssocList.replace<Name, Entry>(
      phonebook,
      name,
      Text.equal,
      ?entry
    ).0;
  };

  public query func lookup(name : Name) : async ?Entry {
    return AssocList.find<Name, Entry>(phonebook, name, Text.equal);
  };
};
