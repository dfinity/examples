import Trie "mo:base/Trie";
import Principal "mo:base/Principal";
import Option "mo:base/Option";
import Iter "mo:base/Iter";
import Types "./Types";

shared(install) actor class DAO(init : ?Types.SystemParams) = Self {
    stable var accounts : Trie.Trie<Principal, Types.Tokens> =
      Trie.put(Trie.empty(), Types.account_key(install.caller), Principal.equal, { amount_e8s = 1_000_000_000_000 }).0;
    stable var proposals : Trie.Trie<Nat, Types.Proposal> = Trie.empty();
    stable var next_proposal_id : Nat = 0;
    stable var system_params : Types.SystemParams = Option.get(init, Types.defaultSystemParams);

    /// Transfer tokens from the caller's account to another account
    public shared({caller}) func transfer(transfer: Types.TransferArgs) : async Types.Result<(), Text> {
        switch (Trie.get(accounts, Types.account_key caller, Principal.equal)) {
        case null { #err "Caller needs an account to transfer funds" };
        case (?from_tokens) {
                 if (from_tokens.amount_e8s < transfer.amount.amount_e8s) {
                     #err ("Caller's account has insufficient funds to transfer " # debug_show(transfer.amount.amount_e8s));
                 } else {
                     let from_amount : Nat = from_tokens.amount_e8s - transfer.amount.amount_e8s;
                     accounts := Trie.put(accounts, Types.account_key(caller), Principal.equal, { amount_e8s = from_amount }).0;
                     let to_amount = Option.get(Trie.get(accounts, Types.account_key(transfer.to), Principal.equal), { amount_e8s = 0 }).amount_e8s + transfer.amount.amount_e8s;
                     accounts := Trie.put(accounts, Types.account_key(transfer.to), Principal.equal, { amount_e8s = to_amount }).0;
                     #ok();
                 };
        };
      };
    };

    /// Return the account balance of the caller
    public shared({caller}) func account_balance() : async Types.Tokens {
        Option.get(Trie.get(accounts, Types.account_key caller, Principal.equal), { amount_e8s = 0 })
    };

    /// Lists all accounts
    public func list_accounts() : async [Types.Account] {
        Iter.toArray(
          Iter.map(Trie.iter(accounts),
                   func ((owner : Principal, tokens : Types.Tokens)) : Types.Account = { owner; tokens }))
    };
};
