import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'get' : ActorMethod<
    [],
    { 'certificate' : [] | [Uint8Array | number[]], 'value' : number }
  >,
  'inc' : ActorMethod<[], number>,
  'set' : ActorMethod<[number], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
