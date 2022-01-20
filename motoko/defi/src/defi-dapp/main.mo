import Map "mo:base/HashMap";
// import DIP20 "DIP20/Token";

actor DEX {
  type Entry = Map.HashMap<DIP20, Nat>;

  // The DEX should keep a book of balance for each user: Map<Principal,Map<Token,Nat>>.
  stable var book = Map.HashMap<Principal, Entry>(0, Principal.equal, Principal.hash);

  /*
  Dex::deposit (token: canisterID, amount: nat)->Res(status){
    Two methods:
      (Only with ICP because of different interface)
        User transfers token to subaccount of DEX ledger (Dex principle, hash(user principle))
        Notify DEX by calling dex::deposit that ICP is transfered
        Increase balance for user

      (DIP20)
        User sets allowance for DEX
        Calls dex::deposit and the DEX transfers the token to itself.
        Increase balance for user
  }
  */

  /*
  Dex::place_order (token: canisterID-have, amount: nat, token: canisterID-want, amount: nat) -> Res(order_id,OrderPlacementResult) {
    IF match:
      Execute order and adjust balances in DEX
      Return if both successfully executed. Think about what happens if one send fails
    ELSE:
      Store an order in the DEX and display to users.
  }
  */

  /*
  Dex::withdraw(token: canisterId, amount: Nat, destAccount: Account) {
    Check if enough funds in book for user/token.
    Decrease book value by amount for the user.
    Transfer amount of tokens to the specified account.
  }
  */

  /*
  Dex::cancel_order(order_id: order_id) ->Res(status){
    Delete order from principal
  }
  */

  /*
  Dex::check_order(order_id: order_id) ->Res(status){
    Return order from principal
  }
  */

  /*
  Dex::list_order() -> Result(List<Order>){
    Return all open orders from principal
  }
  */
};
