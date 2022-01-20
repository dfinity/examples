import type { Principal } from '@dfinity/principal';
export interface Order {
  'id' : string,
  'to' : Token,
  'fromAmount' : bigint,
  'from' : Token,
  'toAmount' : bigint,
}
export interface OrderPlacementResult { 'status' : string, 'order' : Order }
export type Token = string;
export interface TokenBalance {
  'principal' : Principal,
  'token' : Token,
  'amount' : bigint,
}
export interface _SERVICE {
  'balances' : () => Promise<Array<TokenBalance>>,
  'cancel_order' : (arg_0: string) => Promise<undefined>,
  'check_order' : (arg_0: string) => Promise<[] | [Order]>,
  'deposit' : () => Promise<undefined>,
  'list_order' : () => Promise<Array<Order>>,
  'place_order' : (
      arg_0: Token,
      arg_1: bigint,
      arg_2: Token,
      arg_3: bigint,
    ) => Promise<OrderPlacementResult>,
  'whoami' : () => Promise<Principal>,
  'withdraw' : () => Promise<undefined>,
}
