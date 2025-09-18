import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'decrement' : ActorMethod<[], bigint>,
  'getCount' : ActorMethod<[], bigint>,
  'increment' : ActorMethod<[], bigint>,
  'reset' : ActorMethod<[], bigint>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
