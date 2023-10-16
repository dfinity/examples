import Blob "mo:base/Blob";
import Nat "mo:base/Nat";
import P "mo:base/Prelude";
import Principal "mo:base/Principal";
import Result "mo:base/Result";

// TODO: Can we import the ICRC1/2 standard directly instead of one of the
// token canisters?
import TokenA "canister:token_a";

// The swap canister is the main backend canister for this example. To simplify
// this example we configure the swap canister with the two tokens it will be
// swapping.
shared(init_msg) actor class Swap(init_args: {
  token_a: Principal;
  token_b: Principal;
}) = this {

  public type DepositArgs = {
    spender_subaccount: ?Blob;
    token: Principal;
    from: TokenA.Account;
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
    P.nyi();
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
    P.nyi();
  };

  public type WithdrawArgs = {
    token: Principal;
    to: TokenA.Account;
    amount: Nat;
    fee: ?Blob;
    memo: ?Blob;
  };

  public type WithdrawError = {
    // TODO: Fill this in
  };

  // Allow withdrawals
  // - Allow UserA to withdraw TokenB, and UserB to withdraw TokenA
  // - These withdrawal handlers show how to safely send outbound transfers of an ICRC-1 token.
  public shared(msg) func withdraw(args: WithdrawArgs): async Result.Result<Nat, WithdrawError> {
    P.nyi();
  };
};
