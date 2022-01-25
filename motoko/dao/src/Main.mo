import Trie "mo:base/Trie";
import Principal "mo:base/Principal";
import Option "mo:base/Option";
import Types "./Types";

shared(install) actor class DAO(init : ?Types.SystemParams) = Self {
    stable var accounts : Trie.Trie<Principal, Types.Tokens> =
      Trie.put(Trie.empty(), Types.account_key(install.caller), Principal.equal, { amount_e8s = 1_000_000_000_000 }).0;
    stable var proposals : Trie.Trie<Nat, Types.Proposal> = Trie.empty();
    stable var next_proposal_id : Nat = 0;
    stable var system_params : Types.SystemParams = Option.get(init, Types.defaultSystemParams);
};
