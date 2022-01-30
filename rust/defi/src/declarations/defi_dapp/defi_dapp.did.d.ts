import type { Principal } from '@dfinity/principal';
export interface Balance {
  'token' : Token,
  'owner' : Principal,
  'amount' : bigint,
}
export type CancelOrderReceipt = { 'Ok' : OrderId } |
  { 'Err' : { 'NotAllowed' : null } | { 'NotExistingOrder' : null } };
export type DepositReceipt = { 'Ok' : bigint } |
  { 'Err' : { 'TransferFailure' : null } | { 'BalanceLow' : null } };
export interface Order {
  'id' : OrderId,
  'to' : Token,
  'fromAmount' : bigint,
  'owner' : Principal,
  'from' : Token,
  'toAmount' : bigint,
}
export type OrderId = number;
export type OrderPlacementReceipt = { 'Ok' : Order } |
  { 'Err' : { 'InvalidOrder' : null } | { 'OrderBookFull' : null } } |
  { 'Executed' : null };
export type Token = Principal;
export type WithdrawReceipt = { 'Ok' : bigint } |
  { 'Err' : { 'TransferFailure' : null } | { 'BalanceLow' : null } };
export interface _SERVICE {
  'cancelOrder' : (arg_0: OrderId) => Promise<CancelOrderReceipt>,
  'deposit' : (arg_0: Token, arg_1: bigint) => Promise<DepositReceipt>,
  'getAllBalances' : () => Promise<Array<Balance>>,
  'getBalance' : (arg_0: Token) => Promise<bigint>,
  'getBalances' : () => Promise<Array<Balance>>,
  'getDepositAddress' : () => Promise<Array<number>>,
  'getOrder' : (arg_0: OrderId) => Promise<[] | [Order]>,
  'getOrders' : () => Promise<Array<Order>>,
  'getSymbol' : (arg_0: Token) => Promise<string>,
  'placeOrder' : (
      arg_0: Token,
      arg_1: bigint,
      arg_2: Token,
      arg_3: bigint,
    ) => Promise<OrderPlacementReceipt>,
  'whoami' : () => Promise<Principal>,
  'withdraw' : (arg_0: Token, arg_1: bigint) => Promise<WithdrawReceipt>,
}
