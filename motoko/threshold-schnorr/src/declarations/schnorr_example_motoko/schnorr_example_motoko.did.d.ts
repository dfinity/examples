import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type SchnorrAlgorithm = { 'ed25519' : null } |
  { 'bip340secp256k1' : null };
export interface _SERVICE {
  'for_test_only_change_management_canister_id' : ActorMethod<
    [string],
    { 'Ok' : null } |
      { 'Err' : string }
  >,
  'public_key' : ActorMethod<
    [SchnorrAlgorithm],
    { 'Ok' : { 'public_key_hex' : string } } |
      { 'Err' : string }
  >,
  'sign' : ActorMethod<
    [string, SchnorrAlgorithm],
    { 'Ok' : { 'signature_hex' : string } } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
