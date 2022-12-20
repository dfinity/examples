import type { Principal } from '@dfinity/principal';
export type Error = { 'CanisterError' : null } |
  { 'Trap' : { 'message' : string } } |
  { 'CannotNotify' : null } |
  { 'NoSuchToken' : null } |
  { 'Unauthorized' : null } |
  { 'NotOwner' : null };
export type ManageResult = { 'Ok' : null } |
  { 'Err' : Error };
export interface Nft { 'canister' : Principal, 'index' : TokenIndex }
export type TokenIndex = bigint;
export interface _SERVICE {
  'is_authorized' : () => Promise<boolean>,
  'onDIP721Received' : (
      arg_0: Principal,
      arg_1: Principal,
      arg_2: TokenIndex,
      arg_3: Array<number>,
    ) => Promise<undefined>,
  'owned_nfts' : () => Promise<Array<Nft>>,
  'register' : (arg_0: Nft) => Promise<ManageResult>,
  'set_authorized' : (arg_0: Principal, arg_1: boolean) => Promise<
      ManageResult
    >,
  'tokenTransferNotification' : (
      arg_0: string,
      arg_1: { 'principal' : Principal } |
        { 'address' : string },
      arg_2: bigint,
      arg_3: Array<number>,
    ) => Promise<[] | [bigint]>,
  'transfer' : (arg_0: Nft, arg_1: Principal, arg_2: [] | [boolean]) => Promise<
      ManageResult
    >,
}
