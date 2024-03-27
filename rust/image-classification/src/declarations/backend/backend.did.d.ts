import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Classification { 'label' : string, 'score' : number }
export interface ClassificationError { 'message' : string }
export type ClassificationResult = { 'Ok' : Array<Classification> } |
  { 'Err' : ClassificationError };
export interface _SERVICE {
  'classify' : ActorMethod<[Uint8Array | number[]], ClassificationResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
