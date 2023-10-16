import Blob "mo:base/Blob";
import Nat "mo:base/Nat";
import P "mo:base/Prelude";
import Principal "mo:base/Principal";
import Result "mo:base/Result";
import Trie "mo:base/Trie";

// TODO: Can we import the ICRC1/2 standard directly instead of one of the
// token canisters?
import ICRC1 "canister:token_a";

// The swap canister is the main backend canister for this example. To simplify
// this example we configure the swap canister with the two tokens it will be
// swapping.
shared(init_msg) actor class Swap(init_args: {
  token_a: Principal;
  token_b: Principal;
}) = this {

  // TODO: This needs to be per-token.
  private stable var balances : Trie.Trie<Principal, Nat> = Trie.empty();

  private func balanceKey(x : Principal) : Trie.Key<Principal> {
    return { hash = Principal.hash(x); key = x };
  };

  public type DepositArgs = {
    spender_subaccount: ?Blob;
    token: Principal;
    from: ICRC1.Account;
    amount: Nat;
    fee: ?Blob;
    memo: ?Blob;
    created_at_time: ?Nat64;
  };

  public type DepositError = {
    // TODO: Fill this in
  };

  // Accept deposits
  // - Accept TokenA, and TokenB
  // - user approves transfer: `token_a.icrc2_approve({ spender=swap_canister; amount=amount; ... })`
  // - user deposits their token: `swap_canister.deposit({ token=token_a; amount=amount; ... })`
  // - These deposit handlers show how to safely accept and register deposits of an ICRC-2 token. 
  public shared(msg) func deposit(args: DepositArgs): async Result.Result<Nat, DepositError> {
    // Perform the transfer, to capture the tokens.
    let token: ICRC1 = actor(args.token);
    let transfer_result = await token.icrc2_transfer_from({
      spender_subaccount = args.spender_subaccount;
      from = args.from;
      to = { owner = this; subaccount = null };
      amount = args.amount;
      fee = args.fee;
      memo = args.memo;
      created_at_time = args.created_at_time;
    });

    // Check that the transfer was successful.
    let block_height = switch (transfer_result) {
      (#Ok(block_height)) => { block_height };
      (#Err(err)) => {
        // TODO: Better error handling for this
        Debug.trap(debug_show(err));
      };
    };

    // From here on out, we need to make sure that this function does *not*
    // fail. If it failed, the token transfer would be complete (meaning we
    // would have the user's tokens), but we would not have credited their
    // account yet, so this canister would not *know* that it had received the
    // user's tokens.
    // 
    // If the function *can* fail here after this point, we should either:
    // - Move that code to a separate action later
    // - Have failure-handling code which returns the user's tokens
    
    // Credit the sender's account
    // TODO: This needs to be per-token
    let key = balanceKey(args.from);
    let old_balance = Option.get(Trie.get(balances, key, Principal.equal), 0 : Nat);
    let (new_balances, _) : (Trie<Principal, Nat>, ?Nat) = Trie.put(
      balances,
      key,
      Principal.equal,
      old_balance + args.amount,
    );
    balances := new_balances;

    // Return the "block height" of the transfer
    block_height
  };

  public type SwapArgs = {
    user_a: Principal;
    user_b: Principal;
    token_a: Principal;
    token_b: Principal;
    amount_a: Nat;
    // amount_b is determined by the internal exchange rate
  };

  public type SwapError = {
    // TODO: Fill this in
  };

  // Swap TokenA for TokenB
  // - Exchange tokens between the two given users.
  // - For this example, there will be no AMM mechanism, simply a 1-1 swap. The exact
  //   swap pricing mechanism is left as an exercise for the reader.
  public shared(msg) func swap(args: SwapArgs): async Result.Result<Nat, SwapError> {
    // Because both tokens were deposited before calling swap, we can execute
    // this function atomically. To do that there must be no `await` calls in
    // this function. Additionally, we need to be careful with the order that
    // we update any internal state variables. If this function were to update
    // some state variables, then fail, before updating others, it could leave
    // this canister with inconsistent internal state.
    //
    // Making this function atomic makes it safer, because either the whole
    // function will execute or none of it will.
    P.nyi();
  };

  public type WithdrawArgs = {
    token: Principal;
    to: ICRC1.Account;
    amount: Nat;
    fee: ?Blob;
    memo: ?Blob;
    created_at_time: ?Nat64;
  };

  public type WithdrawError = {
    // TODO: Fill this in
  };

  // Allow withdrawals
  // - Allow UserA to withdraw TokenB, and UserB to withdraw TokenA
  // - These withdrawal handlers show how to safely send outbound transfers of an ICRC-1 token.
  public shared(msg) func withdraw(args: WithdrawArgs): async Result.Result<Nat, WithdrawError> {
    // Debit the sender's account
    //
    // We do this first, due to the asynchronous nature of the IC. By debitting
    // the account first, we ensure that the user cannot withdraw more than
    // they have.
    let key = balanceKey(args.from);
    let old_balance = Option.get(Trie.get(balances, key, Principal.equal), 0 : Nat);
    let (new_balances, _) : (Trie<Principal, Nat>, ?Nat) = Trie.put(
      balances,
      key,
      Principal.equal,
      old_balance + args.amount,
    );
    balances := new_balances;

    // TODO: Ensure the user has sufficient balance.

    // Perform the transfer, to send the tokens.
    let token: ICRC1 = actor(args.token);
    let transfer_result = await token.icrc1_transfer({
      from_subaccount = null;
      to = args.to;
      amount = args.amount;
      fee = args.fee;
      memo = args.memo;
      created_at_time = args.created_at_time;
    });

    // Check that the transfer was successful.
    let block_height = switch (transfer_result) {
      (#Ok(block_height)) => { block_height };
      (#Err(err)) => {
        // The transfer failed, we need to refund the user's account (less
        // fees), so that they do not completely lose their tokens, and can
        // retry the withdrawal.
        // TODO: Better error handling for this
        Debug.trap(debug_show(err));
      };
    };

    // Return the "block height" of the transfer
    block_height
  };
};
