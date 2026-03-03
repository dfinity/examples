import type { Principal } from '@icp-sdk/core/principal';
import type { ActorMethod } from '@icp-sdk/core/agent';
import type { IDL } from '@icp-sdk/core/candid';

export interface Classification { 'label' : string, 'score' : number }
export interface ClassificationError { 'message' : string }
export type ClassificationResult = { 'Ok' : Array<Classification> } |
  { 'Err' : ClassificationError };
export interface _SERVICE {
  'classify' : ActorMethod<[Uint8Array | number[]], ClassificationResult>,
  'classify_query' : ActorMethod<[Uint8Array | number[]], ClassificationResult>,
  'run' : ActorMethod<[], ClassificationResult>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
