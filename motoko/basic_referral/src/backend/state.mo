import Text "mo:base/Text";
import TrieMap "mo:base/TrieMap";
import Principal "mo:base/Principal";
import Int "mo:base/Int";
import Hash "mo:base/Hash";

import Types "types";

module {
  private type Map<K, V> = TrieMap.TrieMap<K, V>;

  public type State = {
    profiles : Map<Principal, Types.Profile>;
    referrals : Map<Text, Types.Referral>;
    transactions : Map<Text, Types.TxRecord>;
  };

  public func empty() : State {
    {
      profiles = TrieMap.TrieMap<Principal, Types.Profile>(Principal.equal, Principal.hash);
      referrals = TrieMap.TrieMap<Text, Types.Referral>(Text.equal, Text.hash);
      transactions = TrieMap.TrieMap<Text, Types.TxRecord>(Text.equal, Text.hash);
    };
  };
};
