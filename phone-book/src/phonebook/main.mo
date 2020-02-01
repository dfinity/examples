import List "mo:stdlib/list.mo";
import AssocList "mo:stdlib/assocList.mo";

type Name = Text;
type Phone = Nat;

type Entry = {
    name: Name;
    description: Text;
    phone: Phone;
};

type PhoneBookMap = AssocList.AssocList<Name, Entry>;

actor {
    var phonebook: PhoneBookMap = List.nil<(Name, Entry)>();

    func nameEq(lhs: Name, rhs: Name): Bool {
        return lhs == rhs;
    };

    public func insert(name0: Name, description0: Text, phone0: Phone): async () {
        let newEntry : Entry = {
            name = name0;
            description = description0;
            phone = phone0;
        };

        let (newPhonebook, _) = AssocList.replace<Name, Entry>(
            phonebook,
            name0,
            func(n: Name, m: Name) = n == m,
            ?newEntry
        );
        phonebook := newPhonebook;
    };

    public query func lookup(name: Name): async ?Entry {
        return AssocList.find<Name, Entry>(phonebook, name, nameEq);
    };
};
