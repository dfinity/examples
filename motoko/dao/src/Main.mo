import Trie "mo:base/Trie";
import Principal "mo:base/Principal";
import Option "mo:base/Option";
import Iter "mo:base/Iter";
import Nat "mo:base/Nat";
import Result "mo:base/Result";
import List "mo:base/List";
import Time "mo:base/Time";
import Types "./Types";

shared(install) actor class DAO(init : ?Types.SystemParams) = Self {
    stable var accounts : Trie.Trie<Principal, Types.Tokens> =
      Trie.put(Trie.empty(), Types.account_key(install.caller), Principal.equal, { amount_e8s = 1_000_000_000_000 }).0;
    stable var proposals : Trie.Trie<Nat, Types.Proposal> = Trie.empty();
    stable var next_proposal_id : Nat = 0;
    stable var system_params : Types.SystemParams = Option.get(init, Types.defaultSystemParams);

    system func heartbeat() : async () {
        await execute_accepted_proposals();
        await remove_expired_proposals();
    };

    /// Transfer tokens from the caller's account to another account
    public shared({caller}) func transfer(transfer: Types.TransferArgs) : async Types.Result<(), Text> {
        switch (Trie.get(accounts, Types.account_key caller, Principal.equal)) {
        case null { #err "Caller needs an account to transfer funds" };
        case (?from_tokens) {
                 let amount = transfer.amount.amount_e8s;
                 if (from_tokens.amount_e8s < amount) {
                     #err ("Caller's account has insufficient funds to transfer " # debug_show(amount));
                 } else {
                     let from_amount : Nat = from_tokens.amount_e8s - amount;
                     accounts := Trie.put(accounts, Types.account_key(caller), Principal.equal, { amount_e8s = from_amount }).0;
                     let to_amount = Option.get(Trie.get(accounts, Types.account_key(transfer.to), Principal.equal), Types.zeroToken).amount_e8s + amount;
                     accounts := Trie.put(accounts, Types.account_key(transfer.to), Principal.equal, { amount_e8s = to_amount }).0;
                     #ok;
                 };
        };
      };
    };

    /// Return the account balance of the caller
    public query({caller}) func account_balance() : async Types.Tokens {
        Option.get(Trie.get(accounts, Types.account_key caller, Principal.equal), Types.zeroToken)
    };

    /// Lists all accounts
    public query func list_accounts() : async [Types.Account] {
        Iter.toArray(
          Iter.map(Trie.iter(accounts),
                   func ((owner : Principal, tokens : Types.Tokens)) : Types.Account = { owner; tokens }))
    };

    /// Submit a proposal
    public shared({caller}) func submit_proposal(payload: Types.ProposalPayload) : async Types.Result<Nat, Text> {
        Result.chain(deduct_proposal_submission_deposit(caller), func (()) : Types.Result<Nat, Text> {
            let proposal_id = next_proposal_id;
            next_proposal_id += 1;

            let proposal : Types.Proposal = {
                id = proposal_id;
                timestamp = Time.now();
                proposer = caller;
                payload;
                state = #open;
                votes_yes = Types.zeroToken;
                votes_no = Types.zeroToken;
                voters = List.nil();
            };
            proposals := Trie.put(proposals, Types.proposal_key proposal_id, Nat.equal, proposal).0;
            #ok(proposal_id)
        })
    };

    /// Return the proposal with the given ID, if one exists
    public query func get_proposal(proposal_id: Nat) : async ?Types.Proposal {
        Trie.get(proposals, Types.proposal_key proposal_id, Nat.equal)
    };

    /// Return the list of all proposals
    public query func list_proposals() : async [Types.Proposal] {
        Iter.toArray(Iter.map(Trie.iter(proposals), func (kv : (Nat, Types.Proposal)) : Types.Proposal = kv.1))
    };

    // Vote on an open proposal
    public shared({caller}) func vote(args: Types.VoteArgs) : async Types.Result<Types.ProposalState, Text> {
        switch (Trie.get(proposals, Types.proposal_key(args.proposal_id), Nat.equal)) {
        case null { #err("No proposal with ID " # debug_show(args.proposal_id) # " exists") };
        case (?proposal) {
                 var state = proposal.state;
                 if (state != #open) {
                     return #err("Proposal " # debug_show(args.proposal_id) # " is not open for voting");
                 };
                 switch (Trie.get(accounts, Types.account_key caller, Principal.equal)) {
                 case null { return #err("Caller does not have any tokens to vote with") };
                 case (?{ amount_e8s = voting_tokens }) {
                          if (List.some(proposal.voters, func (e : Principal) : Bool = e == caller)) {
                              return #err("Already voted");
                          };
                          
                          var votes_yes = proposal.votes_yes.amount_e8s;
                          var votes_no = proposal.votes_no.amount_e8s;
                          switch (args.vote) {
                          case (#yes) { votes_yes += voting_tokens };
                          case (#no) { votes_no += voting_tokens };
                          };
                          let voters = List.push(caller, proposal.voters);

                          if (votes_yes >= system_params.proposal_vote_threshold.amount_e8s) {
                              // Refund the proposal deposit when the proposal is accepted
                              ignore do ? {
                                  let account = Trie.get(accounts, Types.account_key(proposal.proposer), Principal.equal)!;
                                  let refunded = account.amount_e8s + system_params.proposal_submission_deposit.amount_e8s;
                                  accounts := Trie.put(accounts, Types.account_key(proposal.proposer), Principal.equal, { amount_e8s = refunded }).0;
                              };
                              state := #accepted;
                          };
                          
                          if (votes_no >= system_params.proposal_vote_threshold.amount_e8s) {
                              state := #rejected;
                          };

                          let updated_proposal = {
                              id = proposal.id;
                              votes_yes = { amount_e8s = votes_yes };                              
                              votes_no = { amount_e8s = votes_no };
                              voters;
                              state;
                              timestamp = proposal.timestamp;
                              proposer = proposal.proposer;
                              payload = proposal.payload;
                          };

                          proposals := Trie.put(proposals, Types.proposal_key(args.proposal_id), Nat.equal, updated_proposal).0;
                      };
                 };
                 #ok(state)
             };
        };
    };

    /// Update system params
    ///
    /// Only callable via proposal execution
    public shared({caller}) func update_system_params(payload: Types.UpdateSystemParamsPayload) : async () {
        if (caller != Principal.fromActor(Self)) {
            return;
        };
        system_params := {
            transfer_fee = Option.get(payload.transfer_fee, system_params.transfer_fee);
            proposal_vote_threshold = Option.get(payload.proposal_vote_threshold, system_params.proposal_vote_threshold);
            proposal_submission_deposit = Option.get(payload.proposal_submission_deposit, system_params.proposal_submission_deposit);
        };
    };

    /// Deduct the proposal submission deposit from the caller's account
    func deduct_proposal_submission_deposit(caller : Principal) : Types.Result<(), Text> {
        switch (Trie.get(accounts, Types.account_key caller, Principal.equal)) {
        case null { #err "Caller needs an account to submit a proposal" };
        case (?from_tokens) {
                 let threshold = system_params.proposal_submission_deposit.amount_e8s;
                 if (from_tokens.amount_e8s < threshold) {
                     #err ("Caller's account must have at least " # debug_show(threshold) # " to submit a proposal")
                 } else {
                     let from_amount : Nat = from_tokens.amount_e8s - threshold;
                     accounts := Trie.put(accounts, Types.account_key(caller), Principal.equal, { amount_e8s = from_amount }).0;
                     #ok
                 };
             };
        };
    };

    /// Execute all accepted proposals
    func execute_accepted_proposals() : async () {
        let accepted_proposals = Trie.filter(proposals, func (_ : Nat, proposal : Types.Proposal) : Bool = proposal.state == #accepted);
        // Update proposal state, so that it won't be picked up by the next heartbeat
        for ((id, proposal) in Trie.iter(accepted_proposals)) {
            update_proposal_state(proposal, #executing);
        };

        for ((id, proposal) in Trie.iter(accepted_proposals)) {
            switch (await execute_proposal(proposal)) {
            case (#ok) { update_proposal_state(proposal, #succeeded); };
            case (#err(err)) { update_proposal_state(proposal, #failed(err)); };
            };
        };
    };

    /// Execute the given proposal
    func execute_proposal(proposal: Types.Proposal) : async Types.Result<(), Text> {
        // unimplemented until raw call is supported
        #ok
    };

    /// Remove expired proposals
    func remove_expired_proposals() : async () {
    };

    func update_proposal_state(proposal: Types.Proposal, state: Types.ProposalState) {
        let updated = {
            state;
            id = proposal.id;
            votes_yes = proposal.votes_yes;
            votes_no = proposal.votes_no;
            voters = proposal.voters;
            timestamp = proposal.timestamp;
            proposer = proposal.proposer;
            payload = proposal.payload;
        };
        proposals := Trie.put(proposals, Types.proposal_key(proposal.id), Nat.equal, updated).0;
    };
};
