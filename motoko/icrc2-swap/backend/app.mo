import Error "mo:core/Error";
import Iter "mo:core/Iter";
import Map "mo:core/Map";
import Option "mo:core/Option";
import Principal "mo:core/Principal";
import Result "mo:core/Result";
import Runtime "mo:core/Runtime";

import ICRC "ICRC";

// The swap canister accepts deposits of two ICRC-2 tokens, swaps balances
// 1:1 between users, and allows withdrawals.
//
// Token principals are read from canister environment variables injected
// automatically by icp-cli during `icp deploy` / `make deploy`.
actor Swap {

  // Track the deposited per-user balances for token A and token B
  private let balancesA = Map.empty<Principal, Nat>();
  private let balancesB = Map.empty<Principal, Nat>();

  // balances is a simple getter to check the balances of all users.
  public query func balances() : async ([(Principal, Nat)], [(Principal, Nat)]) {
    (balancesA.entries().toArray(), balancesB.entries().toArray())
  };

  // Read token principals from PUBLIC_CANISTER_ID:token_a / token_b,
  // injected by icp-cli during `make deploy`.
  func tokenA<system>() : Principal {
    let ?id = Runtime.envVar<system>("PUBLIC_CANISTER_ID:token_a") else
      Runtime.trap("PUBLIC_CANISTER_ID:token_a not set — run make deploy");
    Principal.fromText(id)
  };

  func tokenB<system>() : Principal {
    let ?id = Runtime.envVar<system>("PUBLIC_CANISTER_ID:token_b") else
      Runtime.trap("PUBLIC_CANISTER_ID:token_b not set — run make deploy");
    Principal.fromText(id)
  };

  public type DepositArgs = {
    spender_subaccount : ?Blob;
    token : Principal;
    from : ICRC.Account;
    amount : Nat;
    fee : ?Nat;
    memo : ?Blob;
    created_at_time : ?Nat64;
  };

  public type DepositError = {
    #TransferFromError : ICRC.TransferFromError;
  };

  // Accept deposits
  // - Accept TokenA, and TokenB
  // - user approves transfer: `token_a.icrc2_approve({ spender=swap_canister; amount=amount; ... })`
  // - user deposits their token: `swap_canister.deposit({ token=token_a; amount=amount; ... })`
  // - These deposit handlers show how to safely accept and register deposits of an ICRC-2 token.
  public shared func deposit(args : DepositArgs) : async Result.Result<Nat, DepositError> {
    let token : ICRC.Actor = actor (args.token.toText());
    let balances = which_balances<system>(args.token);

    // Load the fee from the token here. The user can pass a null fee, which
    // means "use the default". So we need to look up the default in order to
    // correctly deduct it from their balance.
    let fee = switch (args.fee) {
      case (?f) { f };
      case (null) { await token.icrc1_fee() };
    };

    // Perform the transfer, to capture the tokens.
    let transfer_result = await token.icrc2_transfer_from({
      spender_subaccount = args.spender_subaccount;
      from = args.from;
      to = { owner = Principal.fromActor(Swap); subaccount = null };
      amount = args.amount;
      fee = ?fee;
      memo = args.memo;
      created_at_time = args.created_at_time;
    });

    // Check that the transfer was successful.
    let block_height = switch (transfer_result) {
      case (#Ok(block_height)) { block_height };
      case (#Err(err)) {
        // Transfer failed. There's no cleanup for us to do since no state has
        // changed, so we can just wrap and return the error to the frontend.
        return #err(#TransferFromError(err));
      };
    };

    // From here on out, we need to make sure that this function does *not*
    // fail. If it failed, the token transfer would be complete (meaning we
    // would have the user's tokens), but we would not have credited their
    // account yet, so this canister would not *know* that it had received the
    // user's tokens.

    // Credit the sender's account
    let sender = args.from.owner;
    let old_balance = balances.get(sender).get(0 : Nat);
    let _ = balances.swap(sender, old_balance + args.amount);

    // Return the "block height" of the transfer
    #ok(block_height);
  };

  public type SwapArgs = {
    user_a : Principal;
    user_b : Principal;
  };

  public type SwapError = {
    // Left as a placeholder for future implementors.
  };

  // Swap TokenA for TokenB
  // - Exchange tokens between the two given users.
  // - UserA's full balance of TokenA is given to UserB, and UserB's full
  //   balance of TokenB is given to UserA.
  // - This function executes atomically (no `await` calls) to ensure consistency.
  public shared func swap(args : SwapArgs) : async Result.Result<(), SwapError> {
    // Give user_a's token_a to user_b
    let _ = balancesA.swap(
      args.user_b,
      balancesA.get(args.user_a).get(0 : Nat) +
      balancesA.get(args.user_b).get(0 : Nat),
    );
    balancesA.remove(args.user_a);

    // Give user_b's token_b to user_a
    let _ = balancesB.swap(
      args.user_a,
      balancesB.get(args.user_a).get(0 : Nat) +
      balancesB.get(args.user_b).get(0 : Nat),
    );
    balancesB.remove(args.user_b);

    #ok(());
  };

  public type WithdrawArgs = {
    token : Principal;
    to : ICRC.Account;
    amount : Nat;
    fee : ?Nat;
    memo : ?Blob;
    created_at_time : ?Nat64;
  };

  public type WithdrawError = {
    #InsufficientFunds : { balance : ICRC.Tokens };
    #TransferError : ICRC.TransferError;
    // The token ledger call trapped — outcome is ambiguous. The balance was
    // already debited and a refund was issued, but the on-chain transfer
    // may or may not have executed.
    #CallFailed : Text;
  };

  // Allow withdrawals
  // - Allow users to withdraw any tokens they hold.
  // - These withdrawal handlers show how to safely send outbound transfers of an ICRC-1 token.
  public shared (msg) func withdraw(args : WithdrawArgs) : async Result.Result<Nat, WithdrawError> {
    // Reject the anonymous principal — it has no private key so nobody can
    // exclusively own its balance, but it is still callable by unauthenticated callers.
    if (msg.caller.isAnonymous()) {
      Runtime.trap("anonymous caller not allowed");
    };

    let token : ICRC.Actor = actor (args.token.toText());
    let balances = which_balances<system>(args.token);

    let fee = switch (args.fee) {
      case (?f) { f };
      case (null) { await token.icrc1_fee() };
    };

    // Check the user's balance is sufficient
    let deduction = args.amount + fee;
    let old_balance = balances.get(msg.caller).get(0 : Nat);
    if (old_balance < deduction) {
      return #err(#InsufficientFunds { balance = old_balance });
    };

    // Debit the sender's account first (before the transfer) to prevent
    // double-withdrawal races. See inline comments in the original code.
    // old_balance >= deduction is guaranteed by the InsufficientFunds check
    // above, so this subtraction cannot underflow at runtime.
    let new_balance = old_balance - deduction;
    if (new_balance == 0) {
      balances.remove(msg.caller);
    } else {
      let _ = balances.swap(msg.caller, new_balance);
    };

    // Perform the transfer, to send the tokens.
    // Wrapped in try/catch: if the token ledger traps (rather than returning
    // a result), Motoko would otherwise propagate the trap and skip the refund
    // below, leaving the user's balance permanently debited. The try/catch
    // ensures we always attempt a refund even if the callee traps.
    let transfer_result = try {
      await token.icrc1_transfer({
        from_subaccount = null;
        to = args.to;
        amount = args.amount;
        fee = ?fee;
        memo = args.memo;
        created_at_time = args.created_at_time;
      })
    } catch (e) {
      // Token ledger trapped — refund and surface the error.
      let b = balances.get(msg.caller).get(0 : Nat);
      let _ = balances.swap(msg.caller, b + args.amount + fee);
      return #err(#CallFailed(e.message()));
    };

    let block_height = switch (transfer_result) {
      case (#Ok(block_height)) { block_height };
      case (#Err(err)) {
        // Transfer failed — refund the user's account.
        let b = balances.get(msg.caller).get(0 : Nat);
        let _ = balances.swap(msg.caller, b + args.amount + fee);
        return #err(#TransferError(err));
      };
    };

    #ok(block_height);
  };

  // which_balances returns the balance map for the given token, asserting it
  // is either token_a or token_b.
  private func which_balances<system>(t : Principal) : Map.Map<Principal, Nat> {
    if (t == tokenA<system>()) {
      balancesA
    } else if (t == tokenB<system>()) {
      balancesB
    } else {
      Runtime.trap("invalid token canister")
    }
  };
};
