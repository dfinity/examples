import type { Principal } from '@dfinity/principal';
export interface Balance {
  'token' : Token,
  'owner' : Principal,
  'amount' : bigint,
}
export type CancelOrderErr = { 'NotAllowed' : null } |
  { 'NotExistingOrder' : null };
export type CancelOrderReceipt = { 'Ok' : OrderId } |
  { 'Err' : CancelOrderErr };
export type DepositErr = { 'TransferFailure' : null } |
  { 'BalanceLow' : null };
export type DepositReceipt = { 'Ok' : bigint } |
  { 'Err' : DepositErr };
export interface Dex {
  'cancelOrder' : (arg_0: OrderId) => Promise<CancelOrderReceipt>,
  'clear' : () => Promise<undefined>,
  'credit' : (arg_0: Principal, arg_1: Principal, arg_2: bigint) => Promise<
      undefined
    >,
  'deposit' : (arg_0: Token) => Promise<DepositReceipt>,
  'getAllBalances' : () => Promise<Array<Balance>>,
  'getBalance' : (arg_0: Token) => Promise<bigint>,
  'getBalances' : () => Promise<Array<Balance>>,
  'getDepositAddress' : () => Promise<Array<number>>,
  'getOrder' : (arg_0: OrderId) => Promise<[] | [Order]>,
  'getOrders' : () => Promise<Array<Order>>,
  'getSymbol' : (arg_0: Token) => Promise<string>,
  'getWithdrawalAddress' : () => Promise<Array<number>>,
  'placeOrder' : (
      arg_0: Token,
      arg_1: bigint,
      arg_2: Token,
      arg_3: bigint,
    ) => Promise<OrderPlacementReceipt>,
  'whoami' : () => Promise<Principal>,
  'withdraw' : (arg_0: Token, arg_1: bigint, arg_2: Principal) => Promise<
      WithdrawReceipt
    >,
}
export interface Order {
  'id' : OrderId,
  'to' : Token,
  'fromAmount' : bigint,
  'owner' : Principal,
  'from' : Token,
  'toAmount' : bigint,
}
export type OrderId = number;
export type OrderPlacementErr = { 'InvalidOrder' : null } |
  { 'OrderBookFull' : null };
export type OrderPlacementReceipt = { 'Ok' : Order } |
  { 'Err' : OrderPlacementErr };
export type Token = Principal;
export type WithdrawErr = { 'TransferFailure' : null } |
  { 'BalanceLow' : null };
export type WithdrawReceipt = { 'Ok' : bigint } |
  { 'Err' : WithdrawErr };
export interface _SERVICE extends Dex {}
