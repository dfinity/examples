import type { Principal } from '@dfinity/principal';
export interface Balance { 'token_canister_id' : Principal, 'amount' : bigint }
export interface Order {
  'id' : bigint,
  'from_amount' : bigint,
  'from_token_canister_id' : Principal,
  'owner' : Principal,
  'to_amount' : bigint,
  'to_token_canister_id' : Principal,
}
export interface OwnerBalance {
  'owner' : Principal,
  'token_canister_id' : Principal,
  'amount' : bigint,
}
export interface _SERVICE {
  'cancel_order' : (arg_0: bigint) => Promise<string>,
  'clear' : () => Promise<string>,
  'deposit' : (arg_0: Principal, arg_1: bigint) => Promise<string>,
  'get_all_balances' : () => Promise<Array<OwnerBalance>>,
  'get_all_orders' : () => Promise<Array<Order>>,
  'get_balance' : (arg_0: Principal) => Promise<[] | [Balance]>,
  'get_balances' : () => Promise<Array<Balance>>,
  'get_from_orders' : (arg_0: Principal) => Promise<Array<Order>>,
  'get_order' : (arg_0: bigint) => Promise<[] | [Order]>,
  'get_orders' : () => Promise<Array<Order>>,
  'get_to_orders' : (arg_0: Principal) => Promise<Array<Order>>,
  'icp_deposit_account' : () => Promise<string>,
  'place_order' : (
      arg_0: Principal,
      arg_1: bigint,
      arg_2: Principal,
      arg_3: bigint,
    ) => Promise<string>,
  'whoami' : () => Promise<Principal>,
  'withdraw' : (arg_0: Principal, arg_1: bigint, arg_2: Principal) => Promise<
      undefined
    >,
}
