import type { Principal } from '@dfinity/principal';
export interface CancelOrderResponse { 'status' : string, 'order_id' : string }
export interface Order {
  'id' : string,
  'to' : Token__1,
  'fromAmount' : bigint,
  'owner' : Principal,
  'from' : Token__1,
  'toAmount' : bigint,
}
export interface OrderPlacementResponse { 'status' : string, 'order' : Order }
export type Token = string;
export type Token__1 = string;
export interface _SERVICE {
  'cancel_order' : (arg_0: string) => Promise<CancelOrderResponse>,
  'check_order' : (arg_0: string) => Promise<[] | [Order]>,
  'convert_icp' : () => Promise<[] | [string]>,
  'convert_token' : () => Promise<[] | [string]>,
  'deposit' : () => Promise<undefined>,
  'deposit_address' : () => Promise<Array<number>>,
  'deposit_dip' : (arg_0: Token) => Promise<[] | [string]>,
  'deposit_icp' : () => Promise<[] | [string]>,
  'init' : () => Promise<[] | [string]>,
  'list_order' : () => Promise<Array<Order>>,
  'place_order' : (
      arg_0: Token,
      arg_1: bigint,
      arg_2: Token,
      arg_3: bigint,
    ) => Promise<OrderPlacementResponse>,
  'whoami' : () => Promise<Principal>,
  'withdraw' : () => Promise<undefined>,
}
